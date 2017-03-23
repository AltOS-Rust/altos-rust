/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

//! Scheduling
//!
//! This module contains functionality for scheduling tasks to run and scheduler initialization.

use task::{self, TaskControl, Delay, Priority, State};
use queue::{SyncQueue, Node};
use alloc::boxed::Box;
use core::ops::Index;
use task::NUM_PRIORITIES;
use atomic::{AtomicUsize, Ordering,ATOMIC_USIZE_INIT};
use arch;

/// The current task.
///
/// This keeps track of the currently running task, this should always be `Some` unless the task is
/// actively being switched out or the scheduler has not been started.
#[no_mangle]
#[doc(hidden)]
pub static mut CURRENT_TASK: Option<Box<Node<TaskControl>>> = None;
pub static PRIORITY_QUEUES: [SyncQueue<TaskControl>; NUM_PRIORITIES] = [
    SyncQueue::new(),
    SyncQueue::new(),
    SyncQueue::new(),
    SyncQueue::new()
];
pub static SLEEP_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();
pub static DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();
pub static OVERFLOW_DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();
pub static NORMAL_TASK_COUNTER: AtomicUsize = ATOMIC_USIZE_INIT;

const NORMAL_TASK_MAX: usize = 10;

impl Index<Priority> for [SyncQueue<TaskControl>] {
    type Output = SyncQueue<TaskControl>;
    fn index(&self, idx: Priority) -> &Self::Output {
        &self[idx as usize]
    }
}

/// Select a new task to run and switch its context, this function MUST only be called from the
/// PendSV handler, calling it from elsewhere could lead to undefined behavior. It must be exposed
/// publicly so that the compiler doesn't optimize it away when compiling for release.
#[no_mangle]
#[doc(hidden)]
pub fn switch_context() {
    // UNSAFE: Accessing CURRENT_TASK
    match unsafe { CURRENT_TASK.take() } {
        Some(mut running) => {
            if running.is_destroyed() {
                drop(running);
            } else {
                let queue_index = running.priority();
                if running.is_stack_overflowed() {
                    panic!("switch_context - The current task's stack overflowed!");
                }
                if running.state() == State::Blocked {
                    match running.delay_type() {
                        Delay::Timeout => DELAY_QUEUE.enqueue(running),
                        Delay::Overflowed => OVERFLOW_DELAY_QUEUE.enqueue(running),
                        Delay::Sleep => SLEEP_QUEUE.enqueue(running),
                        Delay::Invalid => panic!(
                            "switch_context - Running task delay type was not set when switched to Blocked!"
                        ),
                    }
                } else {
                    running.set_ready();
                    PRIORITY_QUEUES[queue_index].enqueue(running);
                }
            }

            // If more than NORMAL_TASK_MAX Normal tasks have run, don't try and schedule
            // a normal priorty task, instead giving a low priority task a shot at running.
            let selected = if NORMAL_TASK_COUNTER.load(Ordering::Relaxed) >= NORMAL_TASK_MAX {
                NORMAL_TASK_COUNTER.store(0, Ordering::Relaxed);
                select_task(Priority::all_except(Priority::Normal))
            }
            else {
                select_task(Priority::all())
            };
            if let Priority::Normal = selected.priority() {
                NORMAL_TASK_COUNTER.fetch_add(1, Ordering::Relaxed);
            }
            unsafe { CURRENT_TASK = Some(selected) };
        },
        None => panic!("switch_context - current task doesn't exist!"),
    }
}

/// Select the next task to run from PRIORITY_QUEUES using a provided Priority Iterator.
///
/// Will select the first available task from the priorities provided by the Iterator.
/// If no task is found, the function panics, but this should not happen due to the idle task.
fn select_task<I: Iterator<Item=Priority>>(priorities: I) -> Box<Node<TaskControl>> {
    for priority in priorities {
        while let Some(mut new_task) = PRIORITY_QUEUES[priority].dequeue() {
            if new_task.is_destroyed() {
                drop(new_task);
            } else {
                new_task.set_running();
                return new_task;
            }
        }
    }
    panic!("select_task - task not selected!");
}

/// Start running the first task in the queue.
pub fn start_scheduler() {
    task::init_idle_task();
    // UNSAFE: Accessing CURRENT_TASK
    unsafe { CURRENT_TASK = Some(select_task(Priority::all())) };
    arch::start_first_task();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    // A test helper function
    fn run_scheduler_with_single_priority(priority: Priority) {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
        test::create_and_schedule_test_task(512, priority, "test task 1");
        test::create_and_schedule_test_task(512, priority, "test task 2");
        start_scheduler();
        for _ in 0..100 {
            assert!(test::current_task().is_some());
            switch_context();
        }
    }

    #[test]
    fn test_system_starts_with_no_task_scheduled() {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
    }

    #[test]
    fn test_scheduler_starts() {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
        test::create_and_schedule_test_task(512, Priority::Normal, "scheduler test");
        start_scheduler();
        assert!(test::current_task().is_some());
    }

    #[test]
    fn test_scheduler_runs_tasks_in_round_robin() {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
        let handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 1");
        let handle_2 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 2");

        start_scheduler();
        for _ in 0..5 {
            assert!(test::current_task().is_some());
            assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
            switch_context();

            assert!(test::current_task().is_some());
            assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
            switch_context();
        }
    }

    #[test]
    fn test_scheduler_picks_critical_first() {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
        let handle_1 = test::create_and_schedule_test_task(512, Priority::Critical, "test task 2");
        test::create_and_schedule_test_task(512, Priority::Normal, "test task 1");

        start_scheduler();
        for _ in 0..100 {
            assert!(test::current_task().is_some());
            assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
            switch_context();
        }
    }

    #[test]
    fn test_scheduler_picks_from_lower_queue_if_higher_is_blocked() {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
        let handle_1 = test::create_and_schedule_test_task(512, Priority::Critical, "test task 1");
        let handle_2 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 2");
        let handle_3 = test::create_and_schedule_test_task(512, Priority::Low, "test task 3");

        start_scheduler();
        assert!(test::current_task().is_some());

        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        test::block_current_task(Delay::Timeout);
        switch_context();

        assert!(test::current_task().is_some());
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        test::block_current_task(Delay::Timeout);
        switch_context();

        assert!(test::current_task().is_some());
        assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_scheduler_doesnt_schedule_destroyed_tasks() {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
        let mut handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 1");
        test::create_and_schedule_test_task(512, Priority::Normal, "test task 2");
        start_scheduler();

        handle_1.destroy();

        for _ in 0..50 {
            switch_context();
            assert!(test::current_task().is_some());
            assert_ne!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        }
    }

    #[test]
    fn test_scheduler_runs_with_single_priority() {
        run_scheduler_with_single_priority(Priority::Critical);
        run_scheduler_with_single_priority(Priority::Normal);
        run_scheduler_with_single_priority(Priority::Low);
    }

    #[test]
    fn test_pick_idle_when_no_task_in_queues() {
        let _g = test::set_up();
        start_scheduler();
        assert_eq!(test::current_task().unwrap().priority(), Priority::__Idle);
    }

    #[test]
    fn test_pick_idle_when_all_tasks_are_blocked() {
        let _g = test::set_up();
        assert!(test::current_task().is_none());
        test::create_and_schedule_test_task(512, Priority::Low, "test task 1");
        test::create_and_schedule_test_task(512, Priority::Normal, "test task 2");
        test::create_and_schedule_test_task(512, Priority::Critical, "test task 3");

        start_scheduler();
        for _ in 0..3 {
            assert!(test::current_task().is_some());

            test::block_current_task(Delay::Timeout);
            switch_context();
        }

        for _ in 0 ..100 {
            assert_eq!(test::current_task().unwrap().priority(), Priority::__Idle);
            switch_context();
        }
    }

    #[test]
    fn test_scheduler_picks_critical_when_low_would_be_picked_before_normal() {
        let _g = test::set_up();
        let handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "Norm task");
        test::create_and_schedule_test_task(512, Priority::Low, "Low task");
        start_scheduler();

        for _ in 0..NORMAL_TASK_MAX  {
            switch_context();
            assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        }

        // We get the normal counter to max and now insert a critical task, low shouldn't run
        let handle_2 = test::create_and_schedule_test_task(512, Priority::Critical, "Critical task");
        switch_context();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_scheduler_selects_low_over_normal_according_to_ratio() {
        let _g = test::set_up();
        let handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "Norm task");
        let handle_2 = test::create_and_schedule_test_task(512, Priority::Low, "Low task");
        start_scheduler();
        for _ in 0..(NORMAL_TASK_MAX)  {
            switch_context();
            assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        }
        switch_context();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }
}

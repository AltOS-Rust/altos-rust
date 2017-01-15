// sched/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! Scheduling
//!
//! This module contains the code for the scheduler and initialization.

use task::{self, TaskControl, Delay, Priority, State};
use queue::{SyncQueue, Node};
use alloc::boxed::Box;
use core::ops::Index;
use task::NUM_PRIORITIES;
use arch;

/// The current task.
///
/// This keeps track of the currently running task, this should always be `Some` unless the task is
/// actively being switched out or the scheduler has not been started.
#[no_mangle]
#[doc(hidden)]
pub static mut CURRENT_TASK: Option<Box<Node<TaskControl>>> = None;

pub static PRIORITY_QUEUES: [SyncQueue<TaskControl>; NUM_PRIORITIES] = [SyncQueue::new(),
                                                                    SyncQueue::new(), 
                                                                    SyncQueue::new(), 
                                                                    SyncQueue::new()];
pub static SLEEP_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();
pub static DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();
pub static OVERFLOW_DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();

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
  /*
  if !is_kernel_running() {
    panic!("switch_context - This function should only get called from kernel code!");
  }
  */
  match unsafe { CURRENT_TASK.take() } {
    Some(mut running) => {
      if running.destroy {
        drop(running);
      }
      else {
        let queue_index = running.priority;
        if running.is_stack_overflowed() {
          panic!("switch_context - The current task's stack overflowed!");
        }
        if running.state == State::Blocked {
          match running.delay_type {
            Delay::Timeout => DELAY_QUEUE.enqueue(running),
            Delay::Overflowed => OVERFLOW_DELAY_QUEUE.enqueue(running),
            Delay::Sleep => SLEEP_QUEUE.enqueue(running),
            Delay::Invalid => panic!("switch_context - Running task delay type was not set when switched to Blocked!"),
          }
        }
        else {
          running.state = State::Ready;
          running.delay_type = Delay::Invalid;
          PRIORITY_QUEUES[queue_index].enqueue(running);
        }
      }

      'main: loop {
        for i in Priority::all() {
          while let Some(mut new_task) = PRIORITY_QUEUES[i].dequeue() {
            if new_task.destroy {
              drop(new_task);
            }
            else {
              new_task.state = State::Running;
              unsafe { CURRENT_TASK = Some(new_task) };
              break 'main;
            }
          }
        }
      }
    },
    None => panic!("switch_context - current task doesn't exist!"),
  }
}

/// Start running the first task in the queue
pub fn start_scheduler() {
    task::init_idle_task();
    unsafe {
      for i in Priority::all() {
        if let Some(mut task) = PRIORITY_QUEUES[i].dequeue() {
          task.state = State::Running;
          CURRENT_TASK = Some(task);
          break;
        }
      }
      debug_assert!(CURRENT_TASK.is_some());
      arch::start_first_task();
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test;

  #[test]
  fn test_start_scheduler() {
    let _g = test::set_up();
    assert!(test::current_task().is_none());
    test::create_and_schedule_test_task(512, Priority::Normal, "scheduler test");
    start_scheduler();
    assert!(test::current_task().is_some());
  }

  #[test]
  fn test_scheduler_round_robin() {
    let _g = test::set_up();
    assert!(test::current_task().is_none());
    let handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 1");
    let handle_2 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 2");
    start_scheduler();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
  }

  #[test]
  fn test_scheduler_higher_first() {
    let _g = test::set_up();
    assert!(test::current_task().is_none());
    let _handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 1");
    let handle_2 = test::create_and_schedule_test_task(512, Priority::Critical, "test task 2");
    start_scheduler();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
  }

  #[test]
  fn test_scheduler_lower_if_higher_blocked() {
    let _g = test::set_up();
    assert!(test::current_task().is_none());
    let handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 1");
    let handle_2 = test::create_and_schedule_test_task(512, Priority::Critical, "test task 2");
    start_scheduler();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    test::current_task().unwrap().state = State::Blocked;
    test::current_task().unwrap().delay_type = Delay::Timeout;

    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
  }

  #[test]
  fn test_scheduler_doesnt_schedule_destroyed_tasks() {
    let _g = test::set_up();
    assert!(test::current_task().is_none());
    let mut handle_1 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 1");
    let mut handle_2 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 2");
    let handle_3 = test::create_and_schedule_test_task(512, Priority::Normal, "test task 3");
    start_scheduler();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    handle_1.destroy();
    
    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));

    // Since task 1 was destroyed, we shouldn't schedule it
    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    handle_2.destroy();

    // Now we should only schedule task 3
    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));
    
    switch_context();
    assert!(test::current_task().is_some());
    assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));
  }
}

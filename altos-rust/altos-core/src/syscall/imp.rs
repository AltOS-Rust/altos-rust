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

use sched::{CURRENT_TASK, SLEEP_QUEUE, DELAY_QUEUE, OVERFLOW_DELAY_QUEUE, PRIORITY_QUEUES};
use task::{TaskHandle, TaskControl, Priority};
use task::args::Args;
use queue::Node;
use alloc::boxed::Box;
use tick;
use sync::{RawMutex, CondVar, CriticalSection};
use arch;

/// An alias for the channel to sleep on that will never be awoken by a wakeup signal. It will
/// still be woken after a timeout.
pub const FOREVER_CHAN: usize = 0;

#[doc(hidden)]
pub fn sys_new_task(code: fn(&mut Args), args: Args, stack_depth: usize, priority: Priority, name: &'static str)
    -> TaskHandle {

    new_task(code, args, stack_depth, priority, name)
}

pub fn new_task(code: fn(&mut Args), args: Args, stack_depth: usize, priority: Priority, name: &'static str)
    -> TaskHandle {

    // Make sure the task is allocated in one fell swoop
    let g = CriticalSection::begin();
    let task = Box::new(Node::new(TaskControl::new(code, args, stack_depth, priority, name)));
    drop(g);

    let handle = TaskHandle::new(&**task);
    PRIORITY_QUEUES[task.priority()].enqueue(task);
    handle
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_exit() {
    exit();
}

fn exit() {
    // UNSAFE: This can only be called from the currently running task, so we know we're the only
    // one with a reference to the task. The destroy method is atomic so we don't have to worry
    // about any threading issues.
    unsafe {
        debug_assert!(CURRENT_TASK.is_some());
        CURRENT_TASK.as_mut().unwrap().destroy();
    }
    sched_yield();
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_sched_yield() {
    sched_yield();
}

fn sched_yield() {
    arch::yield_cpu();
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_sleep(wchan: usize) {
    sleep(wchan);
}

fn sleep(wchan: usize) {
    debug_assert!(wchan != FOREVER_CHAN);
    // UNSAFE: Accessing CURRENT_TASK
    match unsafe { CURRENT_TASK.as_mut() } {
        Some(current) => current.sleep(wchan),
        None => panic!("sleep - current task doesn't exist!"),
    }
    sched_yield();
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_sleep_for(wchan: usize, delay: usize) {
    sleep_for(wchan, delay);
}

fn sleep_for(wchan: usize, delay: usize) {
    // UNSAFE: Accessing CURRENT_TASK
    match unsafe { CURRENT_TASK.as_mut() } {
        Some(current) => current.sleep_for(wchan, delay),
        None => panic!("sleep_for - current task doesn't exist!"),
    }
    sched_yield();
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_wake(wchan: usize) {
    wake(wchan);
}

fn wake(wchan: usize) {
    let mut to_wake = SLEEP_QUEUE.remove(|task| task.wchan() == wchan);
    to_wake.append(DELAY_QUEUE.remove(|task| task.wchan() == wchan));
    to_wake.append(OVERFLOW_DELAY_QUEUE.remove(|task| task.wchan() == wchan));
    for mut task in to_wake {
        task.wake();
        PRIORITY_QUEUES[task.priority()].enqueue(task);
    }
}

#[doc(hidden)]
pub fn sys_system_tick() {
    system_tick();
}

fn system_tick() {
    debug_assert!(arch::in_kernel_mode());

    tick::tick();

    // wake up all tasks sleeping until the current tick
    let ticks = tick::get_tick();

    let to_wake = DELAY_QUEUE.remove(|task| task.tick_to_wake() <= ticks);
    for mut task in to_wake {
        task.wake();
        PRIORITY_QUEUES[task.priority()].enqueue(task);
    }

    // If ticks == all 1's then it's about to overflow.
    if ticks == !0 {
        let overflowed = OVERFLOW_DELAY_QUEUE.remove_all();
        DELAY_QUEUE.append(overflowed);
    }

    // UNSAFE: Accessing CURRENT_TASK
    let current_priority = unsafe {
        match CURRENT_TASK.as_ref() {
            Some(task) => task.priority(),
            None => panic!("system_tick - current task doesn't exist!"),
        }
    };

    for i in Priority::higher(current_priority) {
        if !PRIORITY_QUEUES[i].is_empty() {
            // Only context switch if there's another task at the same or higher priority level
            sched_yield();
            break;
        }
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_mutex_lock(lock: &RawMutex) -> bool {
    mutex_lock(lock)
}

fn mutex_lock(lock: &RawMutex) -> bool {
    use sync::LockError;
    // UNSAFE: Accessing CURRENT_TASK
    let current_tid = match unsafe { CURRENT_TASK.as_ref() } {
        Some(task) => task.tid(),
        None => panic!("mutex_lock - current task doesn't exist!"),
    };
    match lock.try_lock(current_tid) {
        Err(LockError::AlreadyOwned) => {
            panic!("mutex_lock - attempted to acquire a lock that was already owned");
        },
        Err(LockError::Locked) => {
            let wchan = lock.address();
            sleep(wchan);
            false
        },
        Ok(_) => true,
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_mutex_try_lock(lock: &RawMutex) -> bool {
    mutex_try_lock(lock)
}

fn mutex_try_lock(lock: &RawMutex) -> bool {
    use sync::LockError;
    // UNSAFE: Accessing CURRENT_TASK
    let current_tid = match unsafe { CURRENT_TASK.as_ref() } {
        Some(task) => task.tid(),
        None => panic!("mutex_lock - current task doesn't exist!"),
    };
    match lock.try_lock(current_tid) {
        // We don't really care if we try to reacquire the lock since we're non-blocking
        Err(LockError::AlreadyOwned) => true,
        Err(LockError::Locked) => false,
        Ok(_) => true,
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_mutex_unlock(lock: &RawMutex) {
    mutex_unlock(lock);
}

fn mutex_unlock(lock: &RawMutex) {
    use sync::UnlockError;
    // UNSAFE: Accessing CURRENT_TASK
    let current_tid = match unsafe { CURRENT_TASK.as_ref() } {
        Some(task) => task.tid(),
        None => panic!("mutex_unlock - current task doesn't exist!"),
    };
    match lock.try_unlock(current_tid) {
        // No-op if we try to unlock a lock that's not locked
        Err(UnlockError::NotLocked) => {},

        // We tried to unlock a lock that we didn't acquire
        Err(UnlockError::NotOwned) => {
            panic!("mutex_unlock - tried to unlock a lock that was not owned");
        },

        // We successfully unlocked the lock, so we don't have to do any more
        Ok(_) => {
            let wchan = lock.address();
            wake(wchan);
        },
    }
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_condvar_wait(condvar: &CondVar, lock: &RawMutex) {
    condvar_wait(condvar, lock);
}

fn condvar_wait(condvar: &CondVar, lock: &RawMutex) {
    mutex_unlock(lock);

    sleep(condvar as *const _ as usize);
}

#[no_mangle]
#[doc(hidden)]
pub extern "C" fn sys_condvar_broadcast(condvar: &CondVar) {
    condvar_broadcast(condvar);
}

fn condvar_broadcast(condvar: &CondVar) {
    wake(condvar as *const _ as usize);
}

#[cfg(test)]
mod tests {
    use test;
    use super::*;
    use task::{State, Priority};
    use task::args::Args;
    use sched::start_scheduler;

    #[test]
    fn test_new_task() {
        let _g = test::set_up();
        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");
        assert_eq!(handle.name(), Ok("test creation task"));
        assert_eq!(handle.priority(), Ok(Priority::Normal));
        assert_eq!(handle.state(), Ok(State::Ready));
        assert_eq!(handle.stack_size(), Ok(512));

        assert_not!(PRIORITY_QUEUES[Priority::Normal].remove_all().is_empty());
    }

    #[test]
    fn test_sched_yield() {
        // This isn't the greatest test, as the functionality of this method is really just
        // dependent on the platform implementation, but at least we can make sure it's working
        // properly for the test suite
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_sleep() {
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        // There's some special logic when something sleeps on FOREVER_CHAN, so make sure we don't
        // sleep on it
        sleep(!FOREVER_CHAN);
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_wake() {
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        // There's some special logic when something sleeps on FOREVER_CHAN, so make sure we don't
        // sleep on it
        sleep(!FOREVER_CHAN);
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        sched_yield();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));


        wake(!FOREVER_CHAN);
        assert_ne!(handle_1.state(), Ok(State::Blocked));
        // wake should NOT yield the task, so we should still be running task 2
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        sched_yield();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_system_tick() {
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        let old_tick = tick::get_tick();
        system_tick();
        assert_eq!(old_tick + 1, tick::get_tick());
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_sleep_for_forever() {
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        sleep_for(FOREVER_CHAN, 4);
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        // 4 Ticks have passed, task 1 should be woken up now
        assert_ne!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_sleep_for_timeout() {
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        sleep_for(!FOREVER_CHAN, 4);
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        // 4 Ticks have passed, task 1 should be woken up now
        assert_ne!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_sleep_for_early_wake() {
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        sleep_for(!FOREVER_CHAN, 4);
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        wake(!FOREVER_CHAN);
        assert_ne!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_sleep_for_no_timeout_forever() {
        let _g = test::set_up();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        // This should yield the task but immediately wake up on the next tick
        sleep_for(FOREVER_CHAN, 0);
        assert_eq!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        system_tick();
        assert_ne!(handle_1.state(), Ok(State::Blocked));
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_mutex_lock() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        // We should not be blocked after this call
        mutex_lock(&raw_mutex);
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));
        assert_eq!(handle.tid().ok(), raw_mutex.holder());
    }

    #[test]
    #[should_panic]
    fn test_mutex_lock_twice_with_same_task_id_panics() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        // We should not be blocked after this call
        mutex_lock(&raw_mutex);
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));
        assert_eq!(handle.tid().ok(), raw_mutex.holder());

        mutex_lock(&raw_mutex);
    }

    // Hm... this test always fails because the second `mutex_lock` call should put the second task
    // to sleep and block until the lock is acquired... But because it's blocking we can never get
    // past that function call, so the scheduler just keeps trying to schedule tasks until it runs
    // out of tasks to schedule. I'm not so sure how to solve this since this is the behavior that
    // we want...
    #[test]
    #[ignore]
    fn test_mutex_lock_while_locked_sleeps_current_task() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        // We should not be blocked after this call
        mutex_lock(&raw_mutex);
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        assert_eq!(handle_1.tid().ok(), raw_mutex.holder());

        // Switch to task 2 while task 1 holds lock
        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_lock(&raw_mutex);
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        assert_eq!(handle_2.state(), Ok(State::Blocked));
    }

    #[test]
    fn test_mutex_try_lock() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        assert_eq!(mutex_try_lock(&raw_mutex), true);
    }

    #[test]
    fn test_mutex_try_lock_while_locked_returns_false() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        assert_eq!(mutex_try_lock(&raw_mutex), true);

        // Switch to task 2 while task 1 holds lock
        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        assert_eq!(mutex_try_lock(&raw_mutex), false);
    }

    #[test]
    fn test_mutex_try_lock_while_holding_lock_returns_true() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        assert_eq!(mutex_try_lock(&raw_mutex), true);
        assert_eq!(mutex_try_lock(&raw_mutex), true);
    }

    #[test]
    fn test_mutex_unlock() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();

        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_lock(&raw_mutex);
        assert_eq!(handle.tid().ok(), raw_mutex.holder());

        mutex_unlock(&raw_mutex);
        assert!(raw_mutex.holder().is_none());
    }

    #[test]
    fn test_mutex_unlock_while_unlocked_is_noop() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();

        start_scheduler();

        assert!(raw_mutex.holder().is_none());

        mutex_unlock(&raw_mutex);
        assert!(raw_mutex.holder().is_none());
    }

    #[test]
    #[should_panic]
    fn test_mutex_unlock_while_not_holding_panics() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();

        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_lock(&raw_mutex);
        assert_eq!(handle_1.tid().ok(), raw_mutex.holder());

        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_unlock(&raw_mutex);
    }

    #[test]
    fn test_mutex_unlock_wakes_sleeping_tasks() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();

        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_lock(&raw_mutex);
        assert_eq!(handle_1.tid().ok(), raw_mutex.holder());

        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        // Simulate blocking on acquiring the lock
        sleep(raw_mutex.address());
        assert_eq!(handle_2.state(), Ok(State::Blocked));
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_unlock(&raw_mutex);
        assert_eq!(handle_2.state(), Ok(State::Ready));

        // Task 2 can be scheduled again
        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_condvar_wait() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let cond_var = CondVar::new();

        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_lock(&raw_mutex);
        assert_eq!(handle.tid().ok(), raw_mutex.holder());

        condvar_wait(&cond_var, &raw_mutex);
        assert_eq!(handle.state(), Ok(State::Blocked));
    }

    #[test]
    fn test_condvar_wait_using_unlocked_lock_succeeds() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let cond_var = CondVar::new();

        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        condvar_wait(&cond_var, &raw_mutex);
        assert_eq!(handle.state(), Ok(State::Blocked));
    }

    #[test]
    #[should_panic]
    fn test_condvar_wait_using_unacquired_lock_panics() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let cond_var = CondVar::new();

        let (handle_1, handle_2) = test::create_two_tasks();

        start_scheduler();
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_lock(&raw_mutex);
        assert_eq!(handle_1.tid().ok(), raw_mutex.holder());

        // Switch to Task 2 while Task 1 holds the lock
        sched_yield();
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        condvar_wait(&cond_var, &raw_mutex);
    }

    #[test]
    fn test_condvar_broadcast_wakes_waiting_tasks() {
        let _g = test::set_up();
        let raw_mutex = RawMutex::new();
        let cond_var = CondVar::new();

        let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "test creation task");

        start_scheduler();
        assert_eq!(handle.tid(), Ok(test::current_task().unwrap().tid()));

        mutex_lock(&raw_mutex);
        assert_eq!(handle.tid().ok(), raw_mutex.holder());

        condvar_wait(&cond_var, &raw_mutex);
        assert_eq!(handle.state(), Ok(State::Blocked));

        condvar_broadcast(&cond_var);
        assert_eq!(handle.state(), Ok(State::Ready));
    }

    // Stub used for new_task calls.
    fn test_task(_args: &mut Args) {}
}

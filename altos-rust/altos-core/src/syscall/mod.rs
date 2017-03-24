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

//! Syscall interface for the AltOS-Rust kernel.

use sched::{CURRENT_TASK, SLEEP_QUEUE, DELAY_QUEUE, OVERFLOW_DELAY_QUEUE, PRIORITY_QUEUES};
use task::Priority;
use task::args::Args;
use task::{TaskHandle, TaskControl};
use queue::Node;
use alloc::boxed::Box;
use tick;
use sync::{RawMutex, CondVar, CriticalSection};
use arch;

// FIXME: When we can guarantee that syscalls will be executed in an interrupt free context, get
// rid of critical sections in this file.

/// An alias for the channel to sleep on that will never be awoken by a wakeup signal. It will
/// still be woken after a timeout.
pub const FOREVER_CHAN: usize = 0;

/// Creates a new task and puts it into the task queue for running. It returns a `TaskHandle`
/// which is used to monitor the task.
///
/// `new_task` takes several arguments, a `fn(&mut Args)` pointer which specifies the code to run
/// for the task, an `Args` argument for the arguments that will be passed to the task, a `usize`
/// argument for how much space should be allocated for the task's stack, a `Priority` argument for
/// the priority that the task should run with, and a `&str` argument to give the task a readable
/// name.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::{start_scheduler, Priority};
/// use altos_core::syscall::new_task;
/// use altos_core::args::Args;
///
/// // Create the task and hold onto the handle
/// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
///
/// // Start running the task
/// start_scheduler();
///
/// fn test_task(_args: &mut Args) {
///   // Do stuff here...
///   loop {}
/// }
/// ```
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

/// Exits and destroys the currently running task.
///
/// This function must only be called from within task code. Doing so from elsewhere (like an
/// interrupt handler, for example) will still destroy the currently running task. Since something
/// like an interrupt handler can interrupt any task, there's no way to determine which task it
/// would destroy.
///
/// It marks the currently running task to be destroyed, then immediatly yields to the scheduler
/// to allow another task to run.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::syscall;
///
/// fn test_task(_args: &mut Args) {
///   // Do some stuff
///
///   syscall::exit();
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the task is not successfully destroyed (i.e. it gets scheduled
/// after this function is called), but this should never happen.
pub fn exit() -> ! {
    // UNSAFE: This can only be called from the currently running task, so we know we're the only
    // one with a reference to the task. The destroy method is atomic so we don't have to worry
    // about any threading issues.
    unsafe {
        debug_assert!(CURRENT_TASK.is_some());
        CURRENT_TASK.as_mut().unwrap().destroy();
    }
    sched_yield();
    panic!("syscall::exit - task returned from exit!");
}

/// Yield the current task to the scheduler so another task can run.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::syscall::sched_yield;
/// use altos_core::args::Args;
///
/// fn test_task(_args: &mut Args) {
///   loop {
///     // Do some important work...
///
///     // Okay, we're done...
///     sched_yield();
///     // Go back and do it again
///   }
/// }
/// ```
pub fn sched_yield() {
    arch::yield_cpu();
}

/// Put the current task to sleep, waiting on a channel to be woken up.
///
/// `sleep` takes a `usize` argument that acts as an identifier for when to wake up the task. The
/// task will sleep indefinitely if no wakeup signal is sent.
///
/// # Examples
///
/// ```no_run
/// use altos_core::syscall::sleep;
/// use altos_core::atomic::{AtomicBool, ATOMIC_BOOL_INIT, Ordering};
///
/// static flag: AtomicBool = ATOMIC_BOOL_INIT;
///
/// while !flag.load(Ordering::SeqCst) {
///   // Block until some other thread wakes us up
///   sleep(&flag as *const _ as usize);
/// }
/// ```
pub fn sleep(wchan: usize) {
    debug_assert!(wchan != FOREVER_CHAN);
    // Make the critical section for the whole function, wouldn't want to be rude and make a task
    // give up its time slice for no reason
    let _g = CriticalSection::begin();
    // UNSAFE: Accessing CURRENT_TASK
    match unsafe { CURRENT_TASK.as_mut() } {
        Some(current) => current.sleep(wchan),
        None => panic!("sleep - current task doesn't exist!"),
    }
    sched_yield();
}

/// Put the current task to sleep with a timeout, waiting on a channel to be woken up.
///
/// `sleep_for` takes a `usize` argument that acts as an identifier to wake up the task. It also
/// takes a second `usize` argument for the maximum ticks it should sleep before waking.
///
/// # Examples
///
/// ```no_run
/// use altos_core::syscall::{sleep_for, FOREVER_CHAN};
///
/// // Sleep for 300 ticks
/// sleep_for(FOREVER_CHAN, 300);
/// ```
pub fn sleep_for(wchan: usize, delay: usize) {
    // Make the critical section for the whole function, wouldn't want to be rude and make a task
    // give up its time slice for no reason
    let _g = CriticalSection::begin();
    // UNSAFE: Accessing CURRENT_TASK
    match unsafe { CURRENT_TASK.as_mut() } {
        Some(current) => current.sleep_for(wchan, delay),
        None => panic!("sleep_for - current task doesn't exist!"),
    }
    sched_yield();
}

/// Wake up all tasks sleeping on a channel.
///
/// `wake` takes a `usize` argument that acts as an identifier. This will wake up any tasks
/// sleeping on that identifier.
pub fn wake(wchan: usize) {
    // Since we're messing around with all the task queues, lets make sure everything gets done at
    // once
    let _g = CriticalSection::begin();
    let mut to_wake = SLEEP_QUEUE.remove(|task| task.wchan() == wchan);
    to_wake.append(DELAY_QUEUE.remove(|task| task.wchan() == wchan));
    to_wake.append(OVERFLOW_DELAY_QUEUE.remove(|task| task.wchan() == wchan));
    for mut task in to_wake {
        task.wake();
        PRIORITY_QUEUES[task.priority()].enqueue(task);
    }
}

/// Update the system tick count and wake up any delayed tasks that need to be woken.
///
/// This function will wake any tasks that have a delay.
#[doc(hidden)]
pub fn system_tick() {
    debug_assert!(arch::in_kernel_mode());

    // TODO: Do we need a critical section here? We should be in the tick handler
    let _g = CriticalSection::begin();
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

/// Lock a mutex
///
/// This system call will acquire a lock on the `RawMutex` passed in. If the lock is already held
/// by another thread, the calling thread will block. When the lock is released by the other thread
/// it will wake any threads waiting on the lock.
///
/// Normally you should not call this function directly, if you require a mutex lock primitive use
/// the `Mutex` type provided in the `sync` module.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::atomic::RawMutex;
/// use altos_core::syscall::mutex_lock;
///
/// let raw_mutex: RawMutex = RawMutex::new();
///
/// // Lock the mutex to acquire exclusive access
/// mutex_lock(&raw_mutex);
/// ```
///
/// # Panics
///
/// This will panic if there is no task currently running, as is sometimes the case in kernel code,
/// since there would be no task to put to sleep if we were to fail to acquire the lock.
///
/// In order to prevent deadlock, if a thread tries to acquire a lock that it already owns it will
/// panic.
///
/// ```rust,no_run
/// use altos_core::atomic::RawMutex;
/// use altos_core::syscall::mutex_lock;
///
/// let raw_mutex: RawMutex = RawMutex::new();
///
/// // Acquire the lock
/// mutex_lock(&raw_mutex);
///
/// // Try to acquire the lock again... panic!
/// mutex_lock(&raw_mutex);
/// ```
pub fn mutex_lock(lock: &RawMutex) {
    use sync::LockError;
    // UNSAFE: Accessing CURRENT_TASK
    let current_tid = match unsafe { CURRENT_TASK.as_ref() } {
        Some(task) => task.tid(),
        None => panic!("mutex_lock - current task doesn't exist!"),
    };
    loop {
        match lock.try_lock(current_tid) {
            Err(LockError::AlreadyOwned) => {
                panic!("mutex_lock - attempted to acquire a lock that was already owned");
            },
            Err(LockError::Locked) => {
                let wchan = lock.address();
                sleep(wchan);
            },
            Ok(_) => break,
        }
    }
}


/// Attempt to acquire a mutex in a non-blocking fashion
///
/// This system call will acquire a lock on the `RawMutex` passed in. If the lock is already held
/// by another thread, the function will return `false`. If the lock is successfully acquired the
/// function will return `true`.
///
/// If the lock is already held by the calling thread, this function will return true as if it had
/// just acquired the lock.
///
/// Normally you should not call this function directly, if you require a mutex lock primitive use
/// the `Mutex` type provided in the `sync` module.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::atomic::RawMutex;
/// use altos_core::syscall::mutex_try_lock;
///
/// let raw_mutex: RawMutex = RawMutex::new();
///
/// // Lock the mutex to acquire exclusive access
/// mutex_try_lock(&raw_mutex);
/// ```
///
/// # Panics
///
/// This will panic if there is no task currently running, as is sometimes the case in kernel code,
/// since we need to be able to check if the current task already have the lock, as well as mark
/// that the current task has acquired it if it does so.
pub fn mutex_try_lock(lock: &RawMutex) -> bool {
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

/// Unlock a mutex
///
/// This system call will unlock a locked mutex. There is no check to see if the calling thread
/// actually has ownership over the lock. Calling this function will wake any tasks that are
/// blocked on the lock.
///
/// Normally you should not call this function directly, if you require a mutex lock primitive use
/// the `Mutex` type provided in the `sync` module.
///
/// # Example
///
/// ```rust,no_run
/// use altos_core::sync::RawMutex;
/// use altos_core::syscall::{mutex_lock, mutex_unlock};
///
/// let raw_mutex: RawMutex = RawMutex::new();
///
/// // Acquire the lock
/// mutex_lock(&raw_mutex);
///
/// // Do something requiring exclusive access to a resource...
///
/// // Release the lock
/// mutex_unlock(&raw_mutex);
/// ```
///
/// # Panics
///
/// This will panic if there is no task currently running, as is sometimes the case in kernel code,
/// since it needs to be able to verify that the current task is the one that acquired the lock.
///
/// In order to preserve exclusive access guarantees, if a thread tries to unlock a lock that it
/// doesn't own it will panic.
pub fn mutex_unlock(lock: &RawMutex) {
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

/// Wait on a condition variable
///
/// This system call will wait for a signal from the condition variable before proceeding. It will
/// unlock the mutex passed in before putting the running thread to sleep. Signals are not
/// buffered, so calling wait after a signal will still put the calling thread to sleep. The lock
/// *WILL NOT* be reacquired after returning from this system call, it must be manually reacquired.
///
/// Normally you should not call this function directly, if you require a condition variable
/// primitive use the `CondVar` type in the `sync` module.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::syscall::condvar_wait;
/// use altos_core::sync::{CondVar, RawMutex};
///
/// let raw_mutex: RawMutex = RawMutex::new();
/// let cond_var: CondVar = CondVar::new();
///
/// // Acquire the lock
/// raw_mutex.lock();
///
/// // Wait on the condition variable
/// condvar_wait(&cond_var, &raw_mutex);
/// ```
///
/// # Panics
///
/// This funciton will panic if you attempt to pass in a mutex that you have not locked
pub fn condvar_wait(condvar: &CondVar, lock: &RawMutex) {
    let _g = CriticalSection::begin();

    mutex_unlock(lock);

    sleep(condvar as *const _ as usize);
}

/// Wake all threads waiting on a condition
///
/// This system call will notify all threads that are waiting on a given condition variable.
/// Signals are not buffered, so calling `broadcast` before another thread calls `wait` will still
/// put the other thread to sleep.
///
/// Normally you should not call this function directly, if you require a condition variable
/// primitive use the `CondVar` type in the `sync` module.
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::syscall::{condvar_wait, condvar_broadcast};
/// use altos_core::sync::{CondVar, RawMutex};
///
/// let raw_mutex: RawMutex = RawMutex::new();
/// let cond_var: CondVar = CondVar::new();
///
/// // Acquire the lock
/// raw_mutex.lock();
///
/// // Wait on the condition variable
/// condvar_wait(&cond_var, &raw_mutex);
///
/// // From some other thread...
/// condvar_broadcast(&cond_var);
///
/// // Original thread can now proceed
/// ```
pub fn condvar_broadcast(condvar: &CondVar) {
    let _g = CriticalSection::begin();

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

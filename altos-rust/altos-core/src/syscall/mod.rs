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

//! System call interface for the AltOS-Rust kernel.
//!
//! This module provides priviledged functions that interact directly with the kernel, modifying
//! tasks and performing special operations. All system calls can be considered atomic. As an
//! implementation note, some architectures do not allow supervisor calls while interrupts are
//! disabled or while another interrupt is occuring, if this is the case, a `sys_*` version of each
//! syscall is exposed (i.e. `sys_sleep(wchan)` as opposed to `sleep(wchan`). These are the
//! underlying implementations of each system call, and so the functionality is exactly the same.
//! These `sys_*` versions should only be called in an interrupt free environment to ensure that
//! all their operations are atomic, otherwise use the regular system calls which will ensure
//! atomicity for you.
//!
//! # Syscall Calling Convention
//!
//! System calls in the AltOS-Rust kernel have a special calling convention that any implementing
//! portability layers must be aware of.
//!
//! ## Argument Passing
//!
//! System calls can have varying numbers of arguments. When a system call is initiated, the system
//! call number must be passed in as the usual first argument register for whatever architecture is
//! being used. All arguments to the system call must be passed in with the regular argument
//! registers for the architecture that is being targeted. If the architecture's calling convention
//! specifies that an argument should go on the stack, however, that argument should go into the
//! next available caller saved register (after saving the original value onto the stack). This way
//! all arguments to the system call are passed in registers for the supervisor call handler.
//!
//! ## Return Values
//!
//! Some system calls return a value, this value will be in the regular return register for the
//! target architecture's calling convention. Some targets, like the `Cortex-M0` processor and many
//! ARM targets, will save the scratch registers on entry to an interrupt handler, as is the case
//! for supervisor calls on those systems. In this case the implementor must ensure that the
//! returned value is written back to the memory where the register was saved, so that upon return
//! from the interrupt the correct value is stored in the return register.
//!
//! # Important Note
//!
//! This calling convention only applies to the way arguments will be passed to the supervisor
//! interrupt. When actually implementing the supervisor call handler, calling each system call
//! should be done in the platform's standard calling convention. This means you will likely have
//! to shift the arguments over some registers or store some on the stack if neccessary.

mod imp;
mod defs;

use task::Priority;
use task::args::Args;
use task::TaskHandle;
use sync::{RawMutex, CondVar};
use arch;
pub use self::defs::*;
pub use self::imp::*;

/// Create a new task and put it into the task queue for running.
///
/// This function returns a `TaskHandle` which is used to monitor the task.
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

    imp::new_task(code, args, stack_depth, priority, name)
}

/// Exit and destroy the currently running task.
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
    arch::syscall0(SYS_EXIT);
    unreachable!();
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
    arch::syscall0(SYS_SCHED_YIELD);
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
    arch::syscall1(SYS_SLEEP, wchan);
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
    arch::syscall2(SYS_SLEEP_FOR, wchan, delay);
}

/// Wake up all tasks sleeping on a channel.
///
/// `wake` takes a `usize` argument that acts as an identifier. This will wake up any tasks
/// sleeping on that identifier.
pub fn wake(wchan: usize) {
    arch::syscall1(SYS_WAKE, wchan);
}

/// Update the system tick count and wake up any delayed tasks that need to be woken.
///
/// This function will wake any tasks that have a delay.
#[doc(hidden)]
pub fn system_tick() {
    imp::sys_system_tick();
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
    loop {
        if arch::syscall1(SYS_MX_LOCK, lock as *const _ as usize) != 0 {
            break;
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
    arch::syscall1(SYS_MX_TRY_LOCK, lock as *const _ as usize) != 0
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
    arch::syscall1(SYS_MX_UNLOCK, lock as *const _ as usize);
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
/// This function will panic if you attempt to pass in a mutex that you have not locked
pub fn condvar_wait(condvar: &CondVar, lock: &RawMutex) {
    arch::syscall2(SYS_CV_WAIT, condvar as *const _ as usize, lock as *const _ as usize);
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
    arch::syscall1(SYS_CV_BROADCAST, condvar as *const _ as usize);
}

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

//! Sleep based synchronization.
//!
//! This module provides implementation for the `Mutex` to allow for sleep-based thread
//! synchronization. When a thread should block on a shared resource, it will be put to sleep and
//! woken up when the resource become free again. This allows for more efficient use of CPU time as
//! a thread that is waiting on a resource cannot do any work.
//!
//! When a thread is woken up, it is not guaranteed that the resource is available. Another thread
//! could have been waiting on the same resource and woken up first. If this is the case, then that
//! other thread could now be holding the lock.

use atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};
use core::ops::{Drop, Deref, DerefMut};
use core::cell::UnsafeCell;
use syscall;

const LOCK_MASK: usize = ::core::isize::MIN as usize;
const UNLOCKED: usize = 0;
const HOLDER_MASK: usize = !LOCK_MASK;

/// A result type for locking operations
///
/// This result will only carry error information about why a locking operation has failed,
/// otherwise if an operation was successful there is no data to pass on to the calling routine.
#[must_use]
pub type LockResult<E> = Result<(), E>;

/// Errors that can occur when trying to acquire a lock
///
/// When attempting to acquire a lock, there are several issues that can be encountered. If a lock
/// is already owned, it could potentially cause deadlock, so it is best to return an error
/// expressing such a condition has occurred. Otherwise, the most common error state to run into
/// would be if the lock is already held by another thread.
#[derive(Copy, Clone, Debug)]
pub enum LockError {
    /// The lock is already held by the thread trying to acquire it
    AlreadyOwned,

    /// The lock is held by another thread
    Locked,
}

/// Errors that can occur when trying to release a lock
///
/// When trying to release a lock, the only truly valid state should be when the releasing thread
/// is also the holder of the lock.
#[derive(Copy, Clone, Debug)]
pub enum UnlockError {
    /// The lock is not held by any thread currently
    NotLocked,

    /// The lock is held by a thread other than the releasing thread
    NotOwned,
}

/// A mutex primitive for locking and unlocking
///
/// This primitive will keep track of which thread holds ownership over it. The locking and
/// unlocking functions on this type will return a `LockResult` carrying information about any
/// issues that may have been encountered while trying to change the state of the lock. These range
/// from common conditions like another thread holding the lock that is trying to be acquired, to
/// more serious ones such as a thread trying to acquire a lock it already holds.
///
/// This primitive is for very fine grained operations around shared resources, if you require a
/// more managed locking primitive use the `Mutex` type, which is a wrapper around this type.
pub struct RawMutex {
    lock: AtomicUsize,
}

/// A mutex lock to synchronize access to some shared resource.
///
/// If the lock is already held by another thread when the running thread tries to obtain it then
/// it will block and another task will be selected to run.
// We need this to be `repr(C)` because we need the lock field to be the first field in memory
#[repr(C)]
pub struct Mutex<T: ?Sized> {
    lock: RawMutex,
    data: UnsafeCell<T>,
}

/// A guard that controls access to a shared resource.
///
/// When a lock is acquired, a `MutexGuard` will be created for the locking thread. The thread can
/// then use that guard to access the shared data. When the guard goes out of scope, the lock will
/// automatically be freed.
pub struct MutexGuard<'mx, T: ?Sized + 'mx> {
    lock: &'mx RawMutex,
    data: &'mx mut T,
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl RawMutex {
    /// Create a new, unlocked, mutex
    pub const fn new() -> Self {
        RawMutex {
            lock: ATOMIC_USIZE_INIT,
        }
    }

    /// Attempt to acquire the lock for the given thread id
    ///
    /// This function will try to acquire the lock by first checking if it's already held by
    /// anyone. If it is not held by anyone it will attempt to grab the lock with an atomic
    /// operation, returning `Ok` if it was successfully acquired. If the lock is already held by
    /// another thread, or another thread beats this one to locking it then this function will
    /// return an `Err` with more information.
    pub fn try_lock(&self, tid: usize) -> LockResult<LockError> {
        match self.holder() {
            // WE are the bearer of the lock...
            Some(holder) if holder == tid => Err(LockError::AlreadyOwned),

            // Someone else is holding the lock right now
            Some(_) => Err(LockError::Locked),

            // No one is holding the lock, so let's grab it if we can
            None => {
                if self.lock.compare_and_swap(UNLOCKED,
                        LOCK_MASK | tid,
                        Ordering::Acquire) != UNLOCKED {
                    // Someone else grabbed it between the initial check and this exchange
                    Err(LockError::Locked)
                }
                else {
                    Ok(())
                }
            }
        }
    }

    /// Attempt to release the lock for the given thread id
    ///
    /// This function will try to release the lock. It will only succeed if the `tid` passed in is
    /// the same as the one that was used to acquire the lock. If the mutex was not locked to begin
    /// with an `Err` will still be returned, though it will be as if the operation had succeeded.
    pub fn try_unlock(&self, tid: usize) -> LockResult<UnlockError> {
        match self.holder() {
            // We hold the lock, so we can release it
            Some(holder) if holder == tid => {
                self.lock.store(UNLOCKED, Ordering::Release);
                Ok(())
            },

            // Someone else has got the lock, it's not in our rights to release it
            Some(_) => Err(UnlockError::NotOwned),

            // It's not locked...
            None => Err(UnlockError::NotLocked),
        }
    }

    /// Get the current holder of the mutex, if one exists
    ///
    /// This function will return the task id of the thread that is holding the mutex. If the mutex
    /// is not locked then `None` will be returned. This is not an atomic operation, so it is
    /// possible that after reading the value of the lock its state could change.
    pub fn holder(&self) -> Option<usize> {
        let lock_value = self.lock.load(Ordering::Relaxed);

        if lock_value == UNLOCKED {
            None
        }
        else {
            // Mask all but the top bit to get which task is currently holding the lock
            Some(lock_value & HOLDER_MASK)
        }
    }

    /// Get the address of this mutex in memory
    ///
    /// This is mainly used as a wake/sleep channel, so if a thread tries to acquire a lock but
    /// fails it can go to sleep on the channel identified by this mutex's address. When the lock
    /// is later released, the same address can be used to wake up the sleeping tasks.
    pub fn address(&self) -> usize {
        self as *const _ as usize
    }
}

impl<T> Mutex<T> {
    /// Creates a new `Mutex` wrapping the supplied data
    pub const fn new(data: T) -> Self {
        Mutex {
            lock: RawMutex::new(),
            data: UnsafeCell::new(data),
        }
    }
}

impl<T: ?Sized> Mutex<T> {

    /// Try to obtain the lock in a blocking fashion.
    ///
    /// If the lock is not able to be obtained, the thread will be put to sleep, waiting for the
    /// lock to become unlocked by another thread. When the lock is released by the other thread
    /// this thread will wake up and become ready to run again.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use altos_core::sync::Mutex;
    ///
    /// let lock = Mutex::new(0);
    ///
    /// // Acquire the lock
    /// let mut guard = lock.lock();
    /// // We are guaranteed to have the lock now until `guard` is dropped
    /// *guard = 100;
    /// drop(guard); // Could just let guard drop out of scope too...
    /// ```
    pub fn lock(&self) -> MutexGuard<T> {
        syscall::mutex_lock(&self.lock);
        // UNSAFE: lock controls access to data, so only one thread can ever get this &mut
        unsafe { self.build_guard() }
    }

    /// Try to obtain the lock in a non-blocking fashion.
    ///
    /// If the lock is not able to be obtained, this just returns `None`, instead of blocking.
    /// This is useful if a thread has other potential work to do instead of waiting on this
    /// shared resource.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use altos_core::sync::Mutex;
    ///
    /// let lock = Mutex::new(0);
    ///
    /// let guard = lock.try_lock();
    /// if let Some(guard) = guard {
    ///   // Do work with the shared resource...
    /// }
    /// else {
    ///   // Move on with life
    /// }
    /// ```
    pub fn try_lock(&self) -> Option<MutexGuard<T>> {
        if syscall::mutex_try_lock(&self.lock) {
            // UNSAFE: We are guaranteed to have acquired exclusive access over the lock if we've
            // gotten to this case
            Some(unsafe { self.build_guard() })
        }
        else {
            None
        }
    }

    // Build a `MutexGuard` from this Mutex
    //
    // This is a helper function to generate a `MutexGuard` referencing the mutex, and should only
    // be called after successfully acquiring the lock.
    unsafe fn build_guard(&self) -> MutexGuard<T> {
        MutexGuard {
            lock: &self.lock,
            data: &mut *self.data.get(),
        }
    }
}

// Get the underlying mutex from a `MutexGuard` so that it can be manipulated
//
// This should only be used in core library functions since it would allow manipulation of a mutex
// while it is locked. Improper use can break the invariants of the `Mutex` and `MutexGuard` types.
#[doc(hidden)]
pub unsafe fn mutex_from_guard<'a, T>(guard: &MutexGuard<'a, T>) -> &'a RawMutex {
    &guard.lock
}

impl<'mx, T: ?Sized> Deref for MutexGuard<'mx, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.data
    }
}

impl<'mx, T: ?Sized> DerefMut for MutexGuard<'mx, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.data
    }
}

impl<'mx, T: ?Sized> Drop for MutexGuard<'mx, T> {
    /// Dropping the guard will unlock the lock it came from and wake any tasks waiting on it.
    fn drop(&mut self) {
        syscall::mutex_unlock(self.lock);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use task::State;
    use sched;
    use syscall;
    use test;

    // A temporary task id for testing
    const TASK_ID: usize = 0;

    #[test]
    fn test_raw_mutex_try_lock() {
        let raw_mutex = RawMutex::new();
        match raw_mutex.try_lock(TASK_ID) {
            Ok(_) => assert_ne!(raw_mutex.lock.load(Ordering::Relaxed), UNLOCKED),
            Err(_) => assert!(false, "Failed to acquire the lock"),
        }
    }

    #[test]
    fn test_raw_mutex_try_lock_fails_when_locked_by_same_task_id() {
        let raw_mutex = RawMutex::new();

        match raw_mutex.try_lock(TASK_ID) {
            Ok(_) => assert_ne!(raw_mutex.lock.load(Ordering::Relaxed), UNLOCKED),
            Err(_) => assert!(false, "Failed to acquire lock on first try"),
        }

        match raw_mutex.try_lock(TASK_ID) {
            Ok(_) => assert!(false, "Successfully acquired lock when it should be locked"),
            Err(LockError::Locked) => assert!(false, "A different task id had the lock"),
            Err(LockError::AlreadyOwned) => {},
        }
    }

    #[test]
    fn test_raw_mutex_try_lock_fails_when_locked_by_different_task_id() {
        let raw_mutex = RawMutex::new();
        match raw_mutex.try_lock(TASK_ID) {
            Ok(_) => assert_ne!(raw_mutex.lock.load(Ordering::Relaxed), UNLOCKED),
            Err(_) => assert!(false, "Failed to acquire lock on first try"),
        }

        match raw_mutex.try_lock(!TASK_ID) {
            Ok(_) => assert!(false, "Successfully acquired lock when it should be locked"),
            Err(LockError::AlreadyOwned) => assert!(false, "The same task id was holding the lock"),
            Err(LockError::Locked) => {},
        }
    }

    #[test]
    fn test_raw_mutex_try_unlock() {
        let raw_mutex = RawMutex::new();

        // Force lock the mutex
        raw_mutex.lock.store(LOCK_MASK|TASK_ID, Ordering::Relaxed);

        match raw_mutex.try_unlock(TASK_ID) {
            Ok(_) => assert_eq!(raw_mutex.lock.load(Ordering::Relaxed), UNLOCKED),
            Err(_) => assert!(false, "Failed to unlock owned mutex"),
        }

    }

    #[test]
    fn test_raw_mutex_try_unlock_fails_with_wrong_task_id() {
        let raw_mutex = RawMutex::new();

        // Force lock the mutex
        raw_mutex.lock.store(LOCK_MASK|TASK_ID, Ordering::Relaxed);

        // Unlock with some other task id
        match raw_mutex.try_unlock(!TASK_ID) {
            Ok(_) => assert!(false, "Lock was successfully released with the wrong task id"),
            Err(UnlockError::NotLocked) => assert!(false, "Lock was not locked in the first place"),
            Err(UnlockError::NotOwned) => {},
        }
    }

    #[test]
    fn test_raw_mutex_try_unlock_fails_when_not_locked() {
        let raw_mutex = RawMutex::new();

        match raw_mutex.try_unlock(TASK_ID) {
            Ok(_) => assert!(false, "Lock was successfully released when it wasn't locked"),
            Err(UnlockError::NotOwned) => assert!(false, "Lock was owned when it should be free"),
            Err(UnlockError::NotLocked) => {},
        }
    }

    #[test]
    fn test_raw_mutex_holder_returns_tid_of_holding_task() {
        let raw_mutex = RawMutex::new();

        raw_mutex.try_lock(TASK_ID).expect("Failed to acquire lock on first try");
        assert_eq!(raw_mutex.holder(), Some(TASK_ID));
    }

    #[test]
    fn test_raw_mutex_holder_returns_none_if_not_locked() {
        let raw_mutex = RawMutex::new();

        assert_eq!(raw_mutex.holder(), None);
    }

    #[test]
    fn test_mutex_smoke() {
        let _g = test::set_up();
        let mutex = Mutex::new(());
        sched::start_scheduler();

        let guard = mutex.lock();
        // lock and load baby
        assert_ne!(mutex.lock.lock.load(Ordering::Relaxed), UNLOCKED);

        drop(guard);
        assert_eq!(mutex.lock.lock.load(Ordering::Relaxed), UNLOCKED);
    }

    #[test]
    fn test_mutex_wakes_on_release() {
        let _g = test::set_up();
        let mutex = Mutex::new(());
        let (handle_1, handle_2) = test::create_two_tasks();

        sched::start_scheduler();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        let guard = mutex.lock();
        assert_ne!(mutex.lock.lock.load(Ordering::Relaxed), UNLOCKED);

        // Switch to second task
        syscall::sched_yield();
        assert!(test::current_task().is_some());
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

        // task 2 is running, let's try to acquire the lock
        // Because these locks don't actually put the thread to sleep unless our operating system
        // is running, we need to simulate a failed lock attempt by calling sleep on the
        // lock's wchan.
        syscall::sleep(mutex.lock.address());

        // task 1 is simulated to have acquired the lock, lets say it holds the lock for a
        // few context switches.
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        // Now it's done with the lock, so it releases
        drop(guard);
        assert_eq!(mutex.lock.lock.load(Ordering::Relaxed), UNLOCKED);

        // Next context switch should go back to task 2, where theoretically it would
        // acquire the lock
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_mutex_wakes_all_on_release() {
        let _g = test::set_up();
        let mutex = Mutex::new(());
        let (handle_1, handle_2) = test::create_two_tasks();
        let (handle_3, handle_4) = test::create_two_tasks();

        sched::start_scheduler();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        let guard = mutex.lock();

        syscall::sched_yield();

        // See above test for details
        // Second task fails to acquire lock
        syscall::sleep(mutex.lock.address());
        assert_eq!(handle_2.state(), Ok(State::Blocked));
        assert!(test::current_task().is_some());
        assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));
        // Third task fails to acquire lock
        syscall::sleep(mutex.lock.address());
        assert_eq!(handle_3.state(), Ok(State::Blocked));
        assert!(test::current_task().is_some());
        assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));
        // Fourth task fails to acquire lock
        syscall::sleep(mutex.lock.address());
        assert_eq!(handle_4.state(), Ok(State::Blocked));
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        // Task 1 holds the lock, lets context switch a few times
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

        // Release the lock
        drop(guard);
        assert_ne!(handle_2.state(), Ok(State::Blocked));
        assert_ne!(handle_3.state(), Ok(State::Blocked));
        assert_ne!(handle_4.state(), Ok(State::Blocked));

        // Make sure each task can get scheduled
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));
        syscall::system_tick();
        assert!(test::current_task().is_some());
        assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
    }

    #[test]
    fn test_mutex_guard_derefrences_to_owned_data() {
        let _g = test::set_up();
        let mutex = Mutex::new(0);
        sched::start_scheduler();

        let mut guard = mutex.lock();

        *guard = 100;
        assert_eq!(*guard, unsafe { *mutex.data.get() });
    }
}

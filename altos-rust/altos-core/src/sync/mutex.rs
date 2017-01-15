// sync/mutex.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/1/16

//! Sleep based synchronization.
//!
//! This module provides implementation for the `Mutex` to allow for sleep-based thread
//! synchronization. When a thread should block on a shared resource, it will be put to sleep and
//! woken up when the resource become free again. This allows for more efficient use of CPU time as
//! a thread that is waiting on a resource cannot do any work.
//!
//! When a thread is woken up it is not guaranteed that the resource is available, another thread
//! could have been waiting on the same resource and woken up first. If this is the case then that
//! other thread could now be holding the lock.

use atomic::{ATOMIC_BOOL_INIT, AtomicBool, Ordering};
use core::ops::{Drop, Deref, DerefMut};
use core::cell::UnsafeCell;

/// A mutex lock to synchronize access to some shared resource.
///
/// If the lock is already held by another thread when the running thread tries to obtain it then
/// it will block and another task will be selected to run.
pub struct Mutex<T: ?Sized> {
  lock: AtomicBool,
  data: UnsafeCell<T>,
}

/// A guard that controls access to a shared resource.
///
/// When a lock is acquired, a `MutexGuard` will be created for the locking thread. The thread can
/// then use that guard to access the shared data. When the guard goes out of scope the lock will
/// automatically be freed.
pub struct MutexGuard<'mx, T: ?Sized + 'mx> {
  wchan: usize,
  lock: &'mx AtomicBool,
  data: &'mx mut T,
}

unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
unsafe impl<T: ?Sized + Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
  /// Creates a new `Mutex` wrapping the supplied data
  pub const fn new(data: T) -> Self {
    Mutex {
      lock: ATOMIC_BOOL_INIT,
      data: UnsafeCell::new(data),
    }
  }
}

impl<T: ?Sized> Mutex<T> {
  fn wchan(&self) -> usize {
    &self.lock as *const _ as usize
  }

  fn obtain_lock(&self) {
    while self.lock.compare_and_swap(false, true, Ordering::Acquire) != false {
      // let another process run if we can't get the lock
      let wchan = self.wchan();
      ::syscall::sleep(wchan);
    }
  }

  /// Try to obtain the lock in a blocking fashion.
  ///
  /// If the lock is not able to be obtained, the thread will be put to sleep waiting for the lock to
  /// become unlocked by another thread. When the lock is released by the other thread this thread
  /// will wake up and become ready to run again.
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
    self.obtain_lock();
    MutexGuard {
      wchan: self.wchan(),
      lock: &self.lock,
      data: unsafe { &mut *self.data.get() },
    }
  }

  /// Try to obtain the lock in a non-blocking fashion.
  ///
  /// If the lock is not able to be obtained, instead of blocking this just returns `None`. This is
  /// useful if a thread has other potential work to do instead of waiting on this shared resource.
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
    if self.lock.compare_and_swap(false, true, Ordering::Acquire) == false {
      Some(
        MutexGuard {
          wchan: self.wchan(),
          lock: &self.lock,
          data: unsafe { &mut *self.data.get() },
        }
      )
    }
    else {
      None
    }
  }
}

#[doc(hidden)]
pub fn mutex_from_guard<'a, T>(guard: &MutexGuard<'a, T>) -> &'a Mutex<T> {
  unsafe { &*(guard.wchan as *const Mutex<T>) }
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
    // Do we care if we get pre-empted and another thread steals the lock before we wake the
    // sleeping tasks?
    self.lock.store(false, Ordering::SeqCst);
    ::syscall::wake(self.wchan);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use task::State;
  use sched;
  use syscall;
  use test;

  #[test]
  fn test_mutex_smoke() {
    let _g = test::set_up();
    let mutex = Mutex::new(());

    let guard = mutex.lock();
    // lock and load baby
    assert_eq!(mutex.lock.load(Ordering::Relaxed), true);

    drop(guard);
    assert_eq!(mutex.lock.load(Ordering::Relaxed), false);
  }

  #[test]
  fn test_mutex_try_lock_fails() {
    let _g = test::set_up();
    let mutex = Mutex::new(());

    let guard = mutex.lock();
    assert_eq!(mutex.lock.load(Ordering::Relaxed), true);

    let guard2 = mutex.try_lock();
    assert!(guard2.is_none());

    drop(guard);
    assert_eq!(mutex.lock.load(Ordering::Relaxed), false);

    let guard3 = mutex.try_lock();
    assert!(guard3.is_some());
    assert_eq!(mutex.lock.load(Ordering::Relaxed), true);
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
    assert_eq!(mutex.lock.load(Ordering::Relaxed), true);

    // Because these locks don't actually put the thread to sleep unless our operating system is
    // running, we need to simulate a failed lock attempt by calling sleep on the lock's wchan.
    syscall::sleep(mutex.wchan());
    assert_eq!(handle_1.state(), Ok(State::Blocked));
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    // task 2 is simulated to have acquired the lock, lets say it holds the lock for a few context
    // switches.
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    // Now it's done with the lock, so it releases
    drop(guard);
    assert_eq!(mutex.lock.load(Ordering::Relaxed), false);

    // Next context switch shoul go back to task 1, where theoretically it would acquire the lock
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
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
    
    // See above test for details
    // First task fails to acquire lock
    syscall::sleep(mutex.wchan());
    assert_eq!(handle_1.state(), Ok(State::Blocked));
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    // Second task fails to acquire lock
    syscall::sleep(mutex.wchan());
    assert_eq!(handle_2.state(), Ok(State::Blocked));
    assert!(test::current_task().is_some());
    assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));
    // Third task fails to acquire lock
    syscall::sleep(mutex.wchan());
    assert_eq!(handle_3.state(), Ok(State::Blocked));
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));

    // Task 4 holds the lock, lets context switch a few times
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));

    // Release the lock
    drop(guard);
    assert_ne!(handle_1.state(), Ok(State::Blocked));
    assert_ne!(handle_2.state(), Ok(State::Blocked));
    assert_ne!(handle_3.state(), Ok(State::Blocked));

    // Make sure each task can get scheduled
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_3.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));
  }

  #[test]
  fn test_mutex_guard_derefrences_to_owned_data() {
    let mutex = Mutex::new(0);
    let mut guard = mutex.lock();

    *guard = 100;
    assert_eq!(*guard, unsafe { *mutex.data.get() });
  }
}

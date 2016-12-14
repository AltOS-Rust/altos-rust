// sync/spin_mutex.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/8/16

//! Spin based synchronization.
//!
//! This module provides implementation for the `SpinMutex` to allow for spin-based thread
//! synchronization. These spinning primitives are meant for use within the kernel, outside of the
//! kernel `Mutex` should be used as it provides a much more efficient use of CPU time.

use atomic::{ATOMIC_BOOL_INIT, AtomicBool, Ordering};
use core::ops::{Drop, Deref, DerefMut};
use core::cell::UnsafeCell;

/// A spin lock used to synchronize access to a shared resource.
///
/// This locking primitive is based on spinning. If the lock is already held by another thread
/// instead of the running thread yielding the CPU to another thread it will just spin. In a single
/// threaded environment this will just waste time spinning at best, at worst it could deadlock the
/// CPU if no other thread are able to run while this one is spinning. Nevertheless it is a useful
/// tool to have within the kernel as the kernel itself cannot be preempted. In order to provide
/// synchronization across multiple kernel threads a spin lock must be used. If there is only one
/// kernel thread then there will be no contention over the resources and so deadlock will be a
/// non-issue.
pub struct SpinMutex<T: ?Sized> {
  lock: AtomicBool,
  data: UnsafeCell<T>,
}

/// A guard that controls access to a shared resource.
///
/// When a lock is acquired, a `SpinGuard` will be created for the locking thread. The thread can
/// then use that guard to access the shared data. When the guard goes out of scope the lock will
/// automatically be freed.
pub struct SpinGuard<'mx, T: ?Sized + 'mx> {
  lock: &'mx AtomicBool,
  data: &'mx mut T,
}

unsafe impl<T: ?Sized + Send> Send for SpinMutex<T> {}
unsafe impl<T: ?Sized + Send> Sync for SpinMutex<T> {}

impl<T> SpinMutex<T> {
  /// Create a new `SpinMutex` wrapping the provided data.
  pub const fn new(data: T) -> Self {
    SpinMutex {
      lock: ATOMIC_BOOL_INIT,
      data: UnsafeCell::new(data),
    }
  }
}

impl<T: ?Sized> SpinMutex<T> {
  fn obtain_lock(&self) {
    while self.lock.compare_and_swap(false, true, Ordering::Acquire) != false {/* spin */}
  }

  /// Try to obtain the lock in a blocking fashion.
  ///
  /// If the lock is not able to be obtained, the thread will just spin waiting for the lock to
  /// become unlocked by another thread. 
  ///
  /// # Example
  ///
  /// ```rust,no_run
  /// use altos_core::sync::SpinMutex;
  ///
  /// let lock = SpinMutex::new(0);
  ///
  /// // Acquire the lock
  /// let mut guard = lock.lock();
  /// // We are guaranteed to have the lock now until `guard` is dropped
  /// *guard = 100;
  /// drop(guard); // Could just let guard drop out of scope too...
  /// ```
  pub fn lock(&self) -> SpinGuard<T> {
    self.obtain_lock();
    SpinGuard {
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
  /// use altos_core::sync::SpinMutex;
  ///
  /// let lock = SpinMutex::new(0);
  ///
  /// let guard = lock.try_lock();
  /// if let Some(guard) = guard {
  ///   // Do work with the shared resource...
  /// }
  /// else {
  ///   // Move on with life
  /// }
  /// ```
  pub fn try_lock(&self) -> Option<SpinGuard<T>> {
    if self.lock.compare_and_swap(false, true, Ordering::Acquire) == false {
      Some(
        SpinGuard {
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

impl<'mx, T: ?Sized> Deref for SpinGuard<'mx, T> {
  type Target = T;

  fn deref(&self) -> &T {
    &*self.data
  }
}

impl<'mx, T: ?Sized> DerefMut for SpinGuard<'mx, T> {
  fn deref_mut(&mut self) -> &mut T {
    &mut *self.data
  }
}

impl<'mx, T: ?Sized> Drop for SpinGuard<'mx, T> {
  /// Dropping the guard will unlock the lock it came from
  fn drop(&mut self) {
    self.lock.store(false, Ordering::Release);
  }
}

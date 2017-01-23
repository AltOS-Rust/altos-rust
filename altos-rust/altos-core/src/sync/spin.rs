// sync/spin_mutex.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/8/16

//! Spin based synchronization.
//!
//! This module provides implementation for the `SpinMutex` to allow for spin-based thread
//! synchronization. These spinning primitives are meant for use within the kernel, outside of the
//! kernel `Mutex` should be used as it provides a much more efficient use of CPU time.

// NOTE: This is mainly taken from the `spin` crate (https://github.com/mvdnes/spin-rs), we've
// reproduced it here because atomic operations (which are needed for the crate) are not supported
// on our target, and so we've had to implement our own atomic library.

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
      // UNSAFE: access to data is controlled by lock
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
          // UNSAFE: executing this branch means we've obtained the lock
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

#[cfg(test)]
mod tests {
    use std::prelude::v1::*;

    use std::sync::mpsc::channel;
    use std::sync::Arc;
    use std::thread;

    use super::*;

    #[derive(Eq, PartialEq, Debug)]
    struct NonCopy(i32);

    #[test]
    fn smoke() {
        let m = SpinMutex::new(());
        drop(m.lock());
        drop(m.lock());
    }

    #[test]
    fn lots_and_lots() {
        static M: SpinMutex<()>  = SpinMutex::new(());
        static mut CNT: u32 = 0;
        const J: u32 = 1000;
        const K: u32 = 3;

        fn inc() {
            for _ in 0..J {
                unsafe {
                    let _g = M.lock();
                    CNT += 1;
                }
            }
        }

        let (tx, rx) = channel();
        for _ in 0..K {
            let tx2 = tx.clone();
            thread::spawn(move|| { inc(); tx2.send(()).unwrap(); });
            let tx2 = tx.clone();
            thread::spawn(move|| { inc(); tx2.send(()).unwrap(); });
        }

        drop(tx);
        for _ in 0..2 * K {
            rx.recv().unwrap();
        }
        assert_eq!(unsafe {CNT}, J * K * 2);
    }

    #[test]
    fn try_lock() {
        let mutex = SpinMutex::new(42);

        // First lock succeeds
        let a = mutex.try_lock();
        assert_eq!(a.as_ref().map(|r| **r), Some(42));

        // Additional lock failes
        let b = mutex.try_lock();
        assert!(b.is_none());

        // After dropping lock, it succeeds again
        ::core::mem::drop(a);
        let c = mutex.try_lock();
        assert_eq!(c.as_ref().map(|r| **r), Some(42));
    }

    #[test]
    fn test_mutex_arc_nested() {
        // Tests nested mutexes and access
        // to underlying data.
        let arc = Arc::new(SpinMutex::new(1));
        let arc2 = Arc::new(SpinMutex::new(arc));
        let (tx, rx) = channel();
        let _t = thread::spawn(move|| {
            let lock = arc2.lock();
            let lock2 = lock.lock();
            assert_eq!(*lock2, 1);
            tx.send(()).unwrap();
        });
        rx.recv().unwrap();
    }

    #[test]
    fn test_mutex_arc_access_in_unwind() {
        let arc = Arc::new(SpinMutex::new(1));
        let arc2 = arc.clone();
        let _ = thread::spawn(move|| -> () {
            struct Unwinder {
                i: Arc<SpinMutex<i32>>,
            }
            impl Drop for Unwinder {
                fn drop(&mut self) {
                    *self.i.lock() += 1;
                }
            }
            let _u = Unwinder { i: arc2 };
            panic!();
        }).join();
        let lock = arc.lock();
        assert_eq!(*lock, 2);
    }

    #[test]
    fn test_mutex_unsized() {
        let mutex: &SpinMutex<[i32]> = &SpinMutex::new([1, 2, 3]);
        {
            let b = &mut *mutex.lock();
            b[0] = 4;
            b[2] = 5;
        }
        let comp: &[i32] = &[4, 2, 5];
        assert_eq!(&*mutex.lock(), comp);
    }
}

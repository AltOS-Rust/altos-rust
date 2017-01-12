// sync/condvar.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/13/16

//! Condition variable.

use atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use sync::mutex::{MutexGuard, Mutex};

/// A Condition Variable
///
/// (Taken from the Rust std::sync::CondVar description)
/// Condition variables represent the ability to block a thread such that it consumes no CPU time
/// while waiting for an event to occur. Condition variables are typically associated with a
/// boolean predicate (a condition) and a mutex. The predicate is always verified inside of the
/// mutex before determining that thread must block.
///
/// Each condition variable can be use with only one mutex at runtime, any attempt to use multiple
/// mutexes on the same condition variable will result in a panic.
pub struct CondVar {
  mutex: AtomicUsize,
}

unsafe impl Send for CondVar {}
unsafe impl Sync for CondVar {}

impl CondVar {
  /// Creates a new `CondVar` which is ready to be used.
  pub const fn new() -> Self {
    CondVar { 
      mutex: ATOMIC_USIZE_INIT,
    }
  }

  /// Blocks the current task until this condition variable recieves a notification
  ///
  /// This function will automatically unlock the mutex represented by the guard passed in an block
  /// the current task. Calls to notify after the mutex is unlocked can wake up this task. When
  /// this call returns the lock will have been reacquired.
  pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
    // Get a reference to the locked mutex
    let mutex = ::sync::mutex_from_guard(&guard);

    self.verify(mutex);

    // unlock the mutex
    drop(guard);

    // Sleep on the cond var channel
    ::syscall::sleep(self as *const _ as usize);
    
    // re-acquire lock before returning
    mutex.lock()
  }

  /// Wakes up all tasks that are blocked on this condition variable.
  ///
  /// This method will wake up any waiters on this condition variable. The calls to `notify_all()`
  /// are not buffered in any way, calling `wait()` on another thread after calling `notify_all()` will
  /// still block the thread.
  pub fn notify_all(&self) {
    ::syscall::wake(self as *const _ as usize);
  }

  fn verify<T>(&self, mutex: &Mutex<T>) {
    let addr = mutex as *const _ as usize;
    match self.mutex.compare_and_swap(0, addr, Ordering::SeqCst) {
      // We have successfully bound the mutex
      0 => {},

      // Some other thread has bound the mutex before us
      n if n == addr => {},

      // We're using more than one mutex on this CondVar
      _ => panic!("Attempted to use a condition variable with two mutexes!"),
    }
  }
}

/*
impl Drop for CondVar {
  fn drop(&mut self) {
    panic!("Dropping CondVars is not implemented yet!");
  }
}
*/

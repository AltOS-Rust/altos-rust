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

#[cfg(test)]
mod tests {
  use super::*;
  use sync::Mutex;
  use task::State;
  use sched;
  use syscall;
  use test;

  #[test]
  fn test_condvar_smoke() {
    let _g = test::set_up();
    let condvar = CondVar::new();
    let mutex = Mutex::new(());

    let (handle_1, handle_2) = test::create_two_tasks();
    sched::start_scheduler();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

    // Because these mutex locks don't actually put the running thread to sleep we need to simulate
    // two tasks running in parallel and watch what the current task is to see which is 'running'
    let mut guard = mutex.lock();

    // We should be in task 2 after the wait
    guard = condvar.wait(guard);
    assert_eq!(handle_1.state(), Ok(State::Blocked));
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    // Task 1 should be sleeping until the notification, lets context switch a few times...
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    // This notification should wake up task 1, but we haven't context switched yet
    condvar.notify_all();
    assert_ne!(handle_1.state(), Ok(State::Blocked));
    assert!(test::current_task().is_some());
    assert_eq!(handle_2.tid(), Ok(test::current_task().unwrap().tid()));

    // Now we should be back in task 1
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

    drop(guard);
  }

  #[test]
  #[should_panic]
  fn test_condvar_using_two_mutexes_panics() {
    let _g = test::set_up();
    let condvar = CondVar::new();
    let mutex1 = Mutex::new(());
    let mutex2 = Mutex::new(());

    let guard1 = mutex1.lock();
    let guard2 = mutex2.lock();

    condvar.wait(guard1);
    condvar.wait(guard2);
  }

  #[test]
  fn test_condvar_notify_wakes_all_tasks() {
    let _g = test::set_up();
    let condvar = CondVar::new();
    let mutex = Mutex::new(());

    let (handle_1, handle_2) = test::create_two_tasks();
    let (handle_3, handle_4) = test::create_two_tasks();
    sched::start_scheduler();
    assert!(test::current_task().is_some());
    assert_eq!(handle_1.tid(), Ok(test::current_task().unwrap().tid()));

    // See smoke test for details
    let mut guard = mutex.lock();
    // Task 1 waits on condvar
    guard = condvar.wait(guard);
    assert_eq!(handle_1.state(), Ok(State::Blocked));
    // Task 2 waits on condvar
    guard = condvar.wait(guard);
    assert_eq!(handle_2.state(), Ok(State::Blocked));
    // Task 3 waits on condvar
    guard = condvar.wait(guard);
    assert_eq!(handle_3.state(), Ok(State::Blocked));
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));

    // Only Task 4 should be getting scheduled
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));
    syscall::system_tick();
    assert!(test::current_task().is_some());
    assert_eq!(handle_4.tid(), Ok(test::current_task().unwrap().tid()));

    // Wake everyone waiting on this condvar up
    condvar.notify_all();
    assert_ne!(handle_1.state(), Ok(State::Blocked));
    assert_ne!(handle_2.state(), Ok(State::Blocked));
    assert_ne!(handle_3.state(), Ok(State::Blocked));
    assert_ne!(handle_4.state(), Ok(State::Blocked));

    // All tasks should be able to be scheduled now
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

    drop(guard);
  }
}

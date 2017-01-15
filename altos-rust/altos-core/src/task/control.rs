// task/control.rs
// AltOS Rust
//
// Created by Daniel Seitz on 1/11/17

use super::stack::Stack;
use super::args::Args;
use alloc::boxed::Box;
use sync::CriticalSection;

pub const NUM_PRIORITIES: usize = 4;

pub const VALID_TASK: usize = 0xBADB0100;
pub const INVALID_TASK: usize = 0x0;

type HandleResult<T> = Result<T, ()>;

mod tid {
  use atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};

  static CURRENT_TID: AtomicUsize = ATOMIC_USIZE_INIT;
  
  /// Atomically increment the task id and return the old value
  pub fn fetch_next_tid() -> usize {
    CURRENT_TID.fetch_add(1, Ordering::Relaxed)
  }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Delay {
  Timeout,
  Sleep,
  Overflowed,
  Invalid,
}

/// Priorities that a task can have.
///
/// Priorities declare which tasks should be run before others. A higher priority task will always
/// be run before a lower priority task if it is ready to be run.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Priority {
  /// The highest priority.
  ///
  /// Tasks with this priority will always be run before any other task. This priority should be
  /// reserved for short lived, time critical tasks that do work important to the functioning of
  /// the system.
  Critical = 0,

  /// The standard task priority.
  ///
  /// Most tasks should be given this priority. The task can be preempted at any time so should not
  /// rely on any operation to be atomic unless specifically marked in a critical section.
  Normal = 1,

  /// The minimal task priority.
  ///
  /// These tasks should be purely optional for the system to run. They will only be run if there
  /// are no other tasks to run, so on some systems they may never run at all.
  Low = 2,

  #[doc(hidden)]
  __Idle = 3,
}

impl Priority {
  /// Returns a range of values corresponding to each priority, starting from the highest.
  pub fn all() -> ::core::ops::Range<usize> {
    (0..NUM_PRIORITIES)
  }

  /// Returns a range of values corresponding to priorities equal to or higher than `self`.
  ///
  /// Use this to iterate over priority queues starting from the highest priority queue down to the
  /// current priority.
  pub fn higher(&self) -> ::core::ops::Range<usize> {
    0..(*self as usize + 1)
  }
}

/// States a task can be in
///
/// States describe the current condition of a task. The scheduler uses this to determine which
/// tasks are available to run.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
  /// The task is in the process of being created, it has not been initialized yet and is not yet
  /// usable.
  Embryo,

  /// The task is ready to be run if the scheduler decides to pick it.
  Ready,
  
  /// The task is currently running.
  Running,

  /// The task is blocked on some I/O or event. This could mean it's waiting for a device or some
  /// shared resource to become available.
  Blocked,

  /// The task is suspended, it will not run until it is resumed.
  Suspended,
}

/// A `TaskControl` tracks the running state of a task.
/// 
/// This struct keeps track of information about a specific task. When a `TaskControl` goes out of
/// scope the memory associated with it is freed.
#[repr(C)]
#[doc(hidden)]
#[derive(Debug)]
pub struct TaskControl {
  stack: Stack, /*** stack MUST be the first field of the struct ***/
  args: Box<Args>,
  tid: usize,
  name: &'static str,
  valid: usize,
  pub wchan: usize,
  pub delay: usize,
  pub delay_type: Delay,
  pub destroy: bool,
  pub priority: Priority,
  pub state: State,
}

unsafe impl Send for TaskControl {}
unsafe impl Sync for TaskControl {}

impl TaskControl {
  /// Creates a new `TaskControl` initialized and ready to be scheduled.
  ///
  /// All of the arguments to this function are the same as the `new_task` kernel function.
  pub fn new(code: fn(&mut Args), args: Args, depth: usize, priority: Priority, name: &'static str) -> Self {
    let stack = Stack::new(depth);

    // Arguments struct stored right above the stack
    let args_mem: Box<Args> = Box::new(args);

    let tid = tid::fetch_next_tid();

    let mut task = TaskControl {
      stack: stack,
      args: args_mem,
      tid: tid,
      name: name,
      valid: VALID_TASK + (tid & 0xFF),
      wchan: 0,
      delay: 0,
      delay_type: Delay::Invalid,
      destroy: false,
      priority: priority,
      state: State::Embryo,
    };
    task.initialize(code);
    task
  }

  /// This initializes the task's stack. This method MUST only be called once, calling it more than
  /// once could at best waste some stack space and at worst corrupt an active stack.
  fn initialize(&mut self, code: fn(&mut Args)) {
    self.stack.initialize(code, &self.args);
    self.state = State::Ready;
  }

  pub fn destroy(&mut self) {
    // TODO: Check if task is INIT task? So at least we always have a safe task to run...
    let _g = CriticalSection::begin();
    self.destroy = true;
    self.valid = INVALID_TASK;
  }

  /// Checks if the stack has gone past its bounds, returns true if it has.
  ///
  /// Used to check if the stack has exceeded the memory allocated for it. If it has this means
  /// that we may have corrupted some memory.
  pub fn is_stack_overflowed(&self) -> bool {
    // TODO: Add some stack guard bytes to check if we've overflowed during execution?
    //  This would add some extra overhead, maybe have some #[cfg] that determines if we should add
    //  this extra security?
    self.stack.check_overflow()
  }

  pub fn tid(&self) -> usize { self.tid }
}

/// A `TaskHandle` references a `TaskControl` and provides access to some state about it.
/// 
/// A `TaskHandle` is created whenever a new task is requested from the operating system. It
/// provides a way to examine the state of the task at run time as well as perform some operations
/// on it like marking it for destruction. 
///
/// This struct is thread safe, as all accesses to the internal `TaskControl` are checked for
/// validity. If a task has been destroyed by one thread, then any other thread trying to access it
/// will be returned an `Err`.
#[derive(Copy, Clone, Debug)]
pub struct TaskHandle(*const TaskControl);

unsafe impl Send for TaskHandle {}
unsafe impl Sync for TaskHandle {}

impl TaskHandle {
  /// Creates a new `TaskHandle` referencing a `TaskControl`.
  pub fn new(task: &TaskControl) -> Self {
    TaskHandle(task)
  }

  /// Marks a task for destruction by the OS, returns true if it was in a valid state before the
  /// call, false otherwise.
  ///
  /// This does not immediately clean up the task, it only marks the task for destruction. The
  /// memory associated with that task will be reclaimed at the operating system's convenience.
  /// There is no guarantee about when this will happen, and in some circumstances it may in fact
  /// never happen, but once a task has been marked for destruction all attempts to access its data
  /// through a `TaskHandle` will return `Err(())`.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// # use altos_core::{TaskHandle, Priority};
  /// # use altos_core::syscall::new_task;
  /// # use altos_core::args::Args;
  ///
  /// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
  ///
  /// if handle.destroy() {
  ///   // Task was valid, now invalid
  /// }
  /// else {
  ///   // Task had already been destroyed
  /// }
  ///
  /// # fn test_task(_args: &mut Args) {
  /// #   loop {}
  /// # }
  /// ```
  pub fn destroy(&mut self) -> bool {
    // FIXME: If the task has allocated any dynamic memory on it own, this will be leaked when the
    //  task is destroyed.
    //    A possible solution... allocate a heap space for each task. Pass a heap allocation
    //    interface to the task implicitly and do all dynamic memory allocation through this
    //    interface. When the task is destroyed we can just free the whole task-specific heap so we
    //    wont have to worry about leaking memory. This means we would likely have to disallow core 
    //    library `Box` allocations within the task. Or... we just don't allow dynamic allocation 
    //    within tasks. - Daniel Seitz
    let _g = CriticalSection::begin();
    if self.is_valid() {
      let task = self.task_ref_mut();
      task.destroy();
      true
    }
    else {
      false
    }
  }

  /// Returns a task's priority.
  ///
  /// The `Priority` of a task determines in what order it should be run compared to other tasks.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// # use altos_core::{TaskHandle, Priority};
  /// # use altos_core::syscall::new_task;
  /// # use altos_core::args::Args;
  ///
  /// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
  ///
  /// match handle.priority() {
  ///   Ok(priority) => { /* Task was valid */ },
  ///   Err(()) => { /* Task was destroyed */ },
  /// }
  ///
  /// # fn test_task(_args: &mut Args) {
  /// #   loop {}
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// If the task has been destroyed then this method will return an `Err(())`.
  pub fn priority(&self) -> HandleResult<Priority> {
    let _g = CriticalSection::begin();
    if self.is_valid() {
      let task = self.task_ref();
      Ok(task.priority)
    }
    else {
      Err(())
    }
  }

  /// Returns a task's current state.
  ///
  /// The `State` of a task determines if it is able to run or not.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// # use altos_core::{TaskHandle, Priority};
  /// # use altos_core::syscall::new_task;
  /// # use altos_core::args::Args;
  ///
  /// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
  ///
  /// match handle.state() {
  ///   Ok(state) => { /* Task was valid */ },
  ///   Err(()) => { /* Task was destroyed */ },
  /// }
  ///
  /// # fn test_task(_args: &mut Args) {
  /// #   loop {}
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// If the task has been destroyed then this method will return an `Err(())`.
  pub fn state(&self) -> HandleResult<State> {
    let _g = CriticalSection::begin();
    if self.is_valid() {
      let task = self.task_ref();
      Ok(task.state)
    }
    else {
      Err(())
    }
  }

  /// Returns a task's tid (task identifier).
  ///
  /// The tid is a unique identifier that differentiates different tasks even if they have the same
  /// name.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// # use altos_core::{TaskHandle, Priority};
  /// # use altos_core::syscall::new_task;
  /// # use altos_core::args::Args;
  ///
  /// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
  ///
  /// match handle.tid() {
  ///   Ok(tid) => { /* Task was valid */ },
  ///   Err(()) => { /* Task was destroyed */ },
  /// }
  ///
  /// # fn test_task(_args: &mut Args) {
  /// #   loop {}
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// If the task has been destroyed then this method will return an `Err(())`.
  pub fn tid(&self) -> HandleResult<usize> {
    let _g = CriticalSection::begin();
    if self.is_valid() {
      let task = self.task_ref();
      Ok(task.tid)
    }
    else {
      Err(())
    }
  }

  /// Returns the task's name.
  ///
  /// ```rust,no_run
  /// # use altos_core::{TaskHandle, Priority};
  /// # use altos_core::syscall::new_task;
  /// # use altos_core::args::Args;
  ///
  /// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
  /// 
  /// match handle.name() {
  ///   Ok(name) => { /* Task was valid */ },
  ///   Err(()) => { /* Task was destroyed */ },
  /// }
  ///
  /// # fn test_task(_args: &mut Args) {
  /// #   loop {}
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// If the task has been destroyed then this method will return an `Err(())`.
  pub fn name(&self) -> HandleResult<&'static str> {
    let _g = CriticalSection::begin();
    if self.is_valid() {
      let task = self.task_ref();
      Ok(task.name)
    }
    else {
      Err(())
    }
  }

  /// Returns the task's stack size.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// # use altos_core::{TaskHandle, Priority};
  /// # use altos_core::syscall::new_task;
  /// # use altos_core::args::Args;
  ///
  /// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
  ///
  /// match handle.stack_size() {
  ///   Ok(size) => { /* Task was valid */ },
  ///   Err(()) => { /* Task was destroyed */ },
  /// }
  ///
  /// # fn test_task(_args: &mut Args) {
  /// #   loop {}
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// If the task has been destroyed then this method will return an `Err(())`.
  pub fn stack_size(&self) -> HandleResult<usize> {
    let _g = CriticalSection::begin();
    if self.is_valid() {
      let task = self.task_ref();
      Ok(task.stack.depth())
    }
    else {
      Err(())
    }
  }

  /// Check if the task pointed to by this handle is valid
  /// 
  /// # Examples
  /// 
  /// ```rust,no_run
  /// use altos_core::syscall::new_task;
  /// 
  /// let handle = new_task(test_task, Args::empty(), 512, Priority::Normal, "new_task_name");
  /// 
  /// if handle.is_valid() {
  ///   // The task is still valid
  /// }
  /// else {
  ///   // The task has been destroyed
  /// }
  /// 
  /// ```
  pub fn is_valid(&self) -> bool {
    let (tid, valid) = unsafe { ((*self.0).tid, (*self.0).valid) };
    let tid_mask = tid & 0xFF;
    valid == VALID_TASK + tid_mask
  }

  fn task_ref(&self) -> &TaskControl {
    unsafe { &*self.0 }
  }

  fn task_ref_mut(&mut self) -> &mut TaskControl {
    unsafe { &mut *(self.0 as *mut TaskControl) }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test;
  
  fn get_task() -> TaskControl {
    // NOTE: We can't return the TaskControl and the TaskHandle as a tuple here because the
    // TaskControl object get's moved on return so we would end up with a dangling pointer in our
    // TaskHandle
    test::create_test_task(512, Priority::Normal, "task test")
  }

  fn get_invalid_task() -> TaskControl {
    let mut task = test::create_test_task(512, Priority::Normal, "invalid test");
    task.valid = INVALID_TASK;
    task
  }

  #[test]
  fn task_handle_valid() {
    let mut task = get_task();
    let handle = TaskHandle::new(&task);

    assert!(handle.is_valid());
    task.valid = INVALID_TASK;
    assert!(!handle.is_valid());
  }

  #[test]
  fn task_handle_destroy() {
    let task = get_task();
    let mut handle = TaskHandle::new(&task);

    assert!(handle.is_valid());
    assert!(handle.destroy());

    assert!(task.destroy);
    assert!(!handle.is_valid());
  }

  #[test]
  fn invalid_task_handle_destroy() {
    let task = get_invalid_task();
    let mut handle = TaskHandle::new(&task);

    assert!(!handle.is_valid());
    assert!(!handle.destroy());
    assert!(!handle.is_valid());
  }

  #[test]
  fn task_handle_stack_size() {
    let task = get_task();
    let handle = TaskHandle::new(&task);

    assert_eq!(handle.stack_size(), Ok(512));
  }

  #[test]
  fn invalid_task_handle_stack_size() {
    let task = get_invalid_task();
    let handle = TaskHandle::new(&task);

    assert!(handle.stack_size().is_err());
  }

  #[test]
  fn task_handle_priority() {
    let task = get_task();
    let handle = TaskHandle::new(&task);

    assert_eq!(handle.priority(), Ok(Priority::Normal));
  }

  #[test]
  fn invalid_task_handle_priority() {
    let task = get_invalid_task();
    let handle = TaskHandle::new(&task);

    assert!(handle.priority().is_err());
  }

  #[test]
  fn task_handle_state() {
    let task = get_task();
    let handle = TaskHandle::new(&task);

    assert_eq!(handle.state(), Ok(State::Ready));
  }

  #[test]
  fn invalid_task_handle_state() {
    let task = get_invalid_task();
    let handle = TaskHandle::new(&task);

    assert!(handle.state().is_err());
  }

  #[test]
  fn task_handle_name() {
    let task = get_task();
    let handle = TaskHandle::new(&task);

    assert_eq!(handle.name(), Ok("task test"));
  }

  #[test]
  fn invalid_task_handle_name() {
    let task = get_invalid_task();
    let handle = TaskHandle::new(&task);

    assert!(handle.name().is_err());
  }

  #[test]
  fn task_handle_tid() {
    let task = get_task();
    let handle = TaskHandle::new(&task);

    assert_eq!(handle.tid(), Ok(task.tid));

  }

  #[test]
  fn invalid_task_handle_tid() {
    let task = get_invalid_task();
    let handle = TaskHandle::new(&task);

    assert!(handle.tid().is_err());
  }
}

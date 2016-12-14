// task/task_control.rs
// AltOSRust
//
// Created by Daniel Seitz

use alloc::boxed::Box;
use collections::Vec;
use volatile::Volatile;
use super::args::Args;
use super::NUM_PRIORITIES;
use sync::CriticalSection;


const VALID_TASK: usize = 0xBADB0100;
const INVALID_TASK: usize = 0x0;

type HandleResult<T> = Result<T, ()>;

/// Priorities that a task can have.
///
/// Priorities declare which tasks should be run before others. A higher priority task will always
/// be run before a lower priority task if it is ready to be run.
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone, PartialEq)]
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
pub struct TaskControl {
  stack: usize, /* stack pointer MUST be first field */
  stack_base: usize,
  stack_depth: usize,
  args: Box<Args>,
  tid: usize,
  name: &'static str,
  valid: usize,
  pub wchan: usize,
  pub delay: usize,
  pub destroy: bool,
  pub overflowed: bool,
  pub priority: Priority,
  pub state: State,
}

impl TaskControl {
  /// Creates a new `TaskControl` initialized and ready to be scheduled.
  ///
  /// All of the arguments to this function are the same as the `new_task` kernel function.
  pub fn new(code: fn(&mut Args), args: Args, depth: usize, priority: Priority, name: &'static str) -> Self {
    let stack_mem: Vec<u8> = Vec::with_capacity(depth);
    // Arguments struct stored right above the stack
    let args_mem: Box<Args> = Box::new(args);

    let stack = stack_mem.as_ptr() as usize;
    // Don't free the heap space, we'll clean up when we drop the TaskControl
    ::core::mem::forget(stack_mem);
    let tid = tid::fetch_next_tid();
    let mut task = TaskControl {
      stack: stack + depth,
      stack_base: stack,
      stack_depth: depth,
      args: args_mem,
      tid: tid,
      name: name,
      valid: VALID_TASK + (tid & 0xFF),
      wchan: 0,
      delay: 0,
      destroy: false,
      overflowed: false,
      priority: priority,
      state: State::Embryo,
    };
    task.initialize(code);
    task
  }

  /*
  #[allow(dead_code)]
  const fn uninitialized(name: &'static str) -> Self {
    TaskControl {
      stack: 0,
      stack_base: 0,
      stack_depth: 0,
      args: None,
      tid: !0,
      name: name,
      valid: INVALID_TASK,
      wchan: 0,
      delay: 0,
      destroy: false, 
      overflowed: false,
      priority: Priority::Low,
      state: State::Embryo,
    }
  }
  */

  /// This initializes the task's stack. This method MUST only be called once, calling it more than
  /// once could at best waste some stack space and at worst corrupt an active stack.
  fn initialize(&mut self, code: fn(&mut Args)) {
    unsafe {
      let stack_ptr = Volatile::new(self.stack as *const usize);
      self.stack = ::initialize_stack(stack_ptr, code, &self.args);
    }
    self.state = State::Ready;
  }

  /// Checks if the stack has gone past its bounds, returns true if it has.
  ///
  /// Used to check if the stack has exceeded the memory allocated for it. If it has this means
  /// that we may have corrupted some memory.
  pub fn is_stack_overflowed(&self) -> bool {
    // TODO: Add some stack guard bytes to check if we've overflowed during execution?
    //  This would add some extra overhead, maybe have some #[cfg] that determines if we should add
    //  this extra security?
    // FIXME: If the stack has overflowed, then that means that it's overflowed into our
    //  TaskControl! So this check actually does very little when it comes to stack safety.
    //  Possibly reordering how the TaskControl and stack are layed out in memory could help a lot
    //  with avoiding this, or adding some guard bytes (though with our memory constraints, too
    //  many of these could cause a lot of space overhead).
    self.stack <= self.stack_base
  }
}

impl Drop for TaskControl {
  fn drop(&mut self) {
    // Rebuild stack vec then drop stack memory
    let size = self.stack_depth;
    unsafe { 
      drop(Vec::from_raw_parts(self.stack_base as *mut u8, size, size));
    }
  }
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
#[derive(Copy, Clone)]
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
  /// # use altos_core::task::{new_task, TaskHandle, Priority};
  /// # use altos_core::task::args::Args;
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
  pub fn destroy(&self) -> bool {
    // FIXME: If the task has allocated any dynamic memory on it own, this will be leaked when the
    //  task is destroyed.
    if self.is_valid() {
      let task = self.task_ref_mut();
      let critical_guard = CriticalSection::begin();
      task.destroy = true;
      task.valid = INVALID_TASK;
      drop(critical_guard);
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
  /// # use altos_core::task::{new_task, TaskHandle, Priority};
  /// # use altos_core::task::args::Args;
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
  /// # use altos_core::task::{new_task, TaskHandle, Priority};
  /// # use altos_core::task::args::Args;
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
  /// # use altos_core::task::{new_task, TaskHandle, Priority};
  /// # use altos_core::task::args::Args;
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
  /// # use altos_core::task::{new_task, TaskHandle, Priority};
  /// # use altos_core::task::args::Args;
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
  /// # use altos_core::task::{new_task, TaskHandle, Priority};
  /// # use altos_core::task::args::Args;
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
    if self.is_valid() {
      let task = self.task_ref();
      Ok(task.stack_depth)
    }
    else {
      Err(())
    }
  }

  fn is_valid(&self) -> bool {
    let (tid, valid) = unsafe { ((*self.0).tid, (*self.0).valid) };
    let tid_mask = tid & 0xFF;
    valid + tid_mask == VALID_TASK + tid_mask 
  }

  fn task_ref(&self) -> &TaskControl {
    unsafe { &*self.0 }
  }

  fn task_ref_mut(&self) -> &mut TaskControl {
    unsafe { &mut *(self.0 as *mut TaskControl) }
  }
}

mod tid {
  use atomic::{ATOMIC_USIZE_INIT, AtomicUsize, Ordering};

  static CURRENT_TID: AtomicUsize = ATOMIC_USIZE_INIT;
  
  /// Atomically increment the task id and return the old value
  pub fn fetch_next_tid() -> usize {
    CURRENT_TID.fetch_add(1, Ordering::SeqCst)
  }
}

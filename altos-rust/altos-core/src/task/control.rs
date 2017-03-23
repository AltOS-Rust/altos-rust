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
/// Priorities declare which tasks should be run before others. In most cases, a higher priority
/// task will be run before a lower priority task, if it's ready to run.
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
    /// Most tasks should be given this priority. The task can be preempted at any time so should
    /// not rely on any operation to be atomic unless specifically marked in a critical section.
    Normal = 1,

    /// The minimal task priority.
    ///
    /// These tasks run with the lowest priority, meaning that they will not be run as often
    /// as normal tasks.
    Low = 2,

    #[doc(hidden)]
    __Idle = 3,
}

impl Priority {
    /// Returns an Iterator over every Priority.
    pub fn all() -> IterPriority {
        IterPriority::new()
    }

    /// Returns an Iterator over Priorities that are higher than, or equal to, the given priority.
    ///
    /// Use this to iterate over priority queues starting from the highest priority queue
    /// down to the current priority.
    pub fn higher(starting_priority: Priority) -> IterPriorityHigher {
        IterPriorityHigher::new(starting_priority)
    }

    /// Returns an Iterator over Priority that will skip the specified priority.
    ///
    /// This is used to customize the behavior of task selection by selectively skipping over
    /// a specific priority.
    pub fn all_except(exclude_priority: Priority) -> IterPrioritySkip {
        IterPrioritySkip::new(exclude_priority)
    }

    // Returns the next priority, starting from higher priorities to lower priorities.
    fn next(&self) -> Option<Priority> {
        match *self {
            Priority::Critical => Some(Priority::Normal),
            Priority::Normal => Some(Priority::Low),
            Priority::Low => Some(Priority::__Idle),
            Priority::__Idle => None,
        }
    }
}

// An Iterator over all the items in Priority
pub struct IterPriority {
    priority: Option<Priority>,
}

impl IterPriority {
    const fn new() -> IterPriority {
        IterPriority {
            priority: Some(Priority::Critical),
        }
    }
}

impl Iterator for IterPriority {
    type Item = Priority;

    fn next(&mut self) -> Option<Priority> {
        match self.priority {
            Some(priority) => {
                ::core::mem::replace(&mut self.priority, priority.next())
            }
            None => None
        }
    }
}

// An Iterator over all the items in Priority higher than the specified priority
pub struct IterPriorityHigher {
    priority: Option<Priority>,
    stop_priority: Priority,
}

impl IterPriorityHigher {
    const fn new(stop_at_priority: Priority) -> IterPriorityHigher {
        IterPriorityHigher {
            priority: Some(Priority::Critical),
            stop_priority: stop_at_priority,
        }
    }
}

impl Iterator for IterPriorityHigher {
    type Item = Priority;

    fn next(&mut self) -> Option<Priority> {
        match self.priority {
            Some(priority) => {
                if priority == self.stop_priority {
                    ::core::mem::replace(&mut self.priority, None)
                }
                else {
                    ::core::mem::replace(&mut self.priority, priority.next())
                }
            }
            None => None
        }
    }
}

// An Iterator over all the items in Priority, except for the specified `skip` priority
pub struct IterPrioritySkip {
    priority: Option<Priority>,
    skip: Priority,
}

impl IterPrioritySkip {
    const fn new(skip: Priority) -> IterPrioritySkip {
        IterPrioritySkip {
            priority: Some(Priority::Critical),
            skip: skip,
        }
    }
}

impl Iterator for IterPrioritySkip {
    type Item = Priority;

    fn next(&mut self) -> Option<Priority> {
        match self.priority {
            Some(priority) => {
                if priority == self.skip {
                    self.priority = priority.next();
                }
                let next_priority = self.priority.unwrap().next();
                ::core::mem::replace(&mut self.priority, next_priority)
            }
            None => None
        }
    }
}

/// States a task can be in.
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
    wchan: usize,
    delay: usize,
    delay_type: Delay,
    destroy: bool,
    priority: Priority,
    state: State,
}

unsafe impl Send for TaskControl {}
unsafe impl Sync for TaskControl {}

impl TaskControl {
    /// Creates a new `TaskControl` initialized and ready to be scheduled.
    ///
    /// All of the arguments to this function are the same as the `new_task` kernel function.
    pub fn new(code: fn(&mut Args), args: Args, depth: usize, priority: Priority, name: &'static str)
        -> Self {

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

    /// This initializes the task's stack. This method MUST only be called once, calling it more
    /// than once could, at best, waste some stack space and, at worst, corrupt an active stack.
    fn initialize(&mut self, code: fn(&mut Args)) {
        self.stack.initialize(code, &self.args);
        self.state = State::Ready;
    }

    pub fn destroy(&mut self) {
        if let Priority::__Idle = self.priority {
            panic!("Tried to destroy the Idle task!");
        }

        let _g = CriticalSection::begin();
        self.destroy = true;
        self.valid = INVALID_TASK;
    }

    /// Checks if the stack has gone past its bounds, returns true if it has.
    ///
    /// Used to check if the stack has exceeded the memory allocated for it. If it has, this means
    /// that we may have corrupted some memory.
    pub fn is_stack_overflowed(&self) -> bool {
        // TODO: Add some stack guard bytes to check if we've overflowed during execution? This
        // would add some extra overhead, maybe have some #[cfg] that determines if we should add
        // this extra security?
        self.stack.check_overflow()
    }

    pub fn set_ready(&mut self) {
        self.state = State::Ready;
        self.delay_type = Delay::Invalid;
    }

    pub fn set_running(&mut self) {
        self.state = State::Running;
    }

    pub fn block(&mut self, delay_type: Delay) {
        self.state = State::Blocked;
        self.delay_type = delay_type;
    }

    /// Wake a sleeping task
    ///
    /// Set a task to the `Ready` state from the `Blocked` state.
    pub fn wake(&mut self) {
        debug_assert!(self.state == State::Blocked);
        self.set_ready();
        self.wchan = 0;
        self.delay = 0;
    }

    /// Put a task to sleep without timeout
    ///
    /// The task will sleep on `wchan` until woken up. If a wake signal is never received the task
    /// will never awaken.
    pub fn sleep(&mut self, wchan: usize) {
        debug_assert!(self.state == State::Running);
        self.block(Delay::Sleep);
        self.wchan = wchan;
    }

    /// Put a task to sleep
    ///
    /// The task will sleep on `wchan` until woken up or until a number of ticks > `delay` has
    /// passed. The task is guaranteed to wake up eventually.
    pub fn sleep_for(&mut self, wchan: usize, delay: usize) {
        debug_assert!(self.state == State::Running);
        let ticks = ::tick::get_tick();
        self.wchan = wchan;
        self.delay = ticks.wrapping_add(delay);
        if self.delay < ticks {
            self.block(Delay::Overflowed);
        }
        else {
            self.block(Delay::Timeout);
        }
    }

    pub fn tid(&self) -> usize { self.tid }

    pub fn wchan(&self) -> usize { self.wchan }

    pub fn tick_to_wake(&self) -> usize { self.delay }

    pub fn delay_type(&self) -> Delay { self.delay_type }

    pub fn priority(&self) -> Priority { self.priority }

    pub fn is_destroyed(&self) -> bool { self.destroy }

    pub fn state(&self) -> State { self.state }
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

    /// Marks a task for destruction by the OS. Returns true if it was in a valid state before the
    /// call, false otherwise.
    ///
    /// This does not immediately clean up the task, it only marks the task for destruction. The
    /// memory associated with that task will be reclaimed at the operating system's convenience.
    /// There is no guarantee about when this will happen, and in some circumstances it may in fact
    /// never happen, but once a task has been marked for destruction all attempts to access its
    /// data through a `TaskHandle` will return `Err(())`.
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
        // FIXME: If the task has allocated any dynamic memory on it own, this will be leaked when
        // the task is destroyed.
        //   A possible solution: Allocate a heap space for each task. Pass a heap allocation
        //   interface to the task implicitly and do all dynamic memory allocation through this
        //   interface. When the task is destroyed we can just free the whole task-specific heap
        //   so we wont have to worry about leaking memory. This means we would likely have to
        //   disallow core library `Box` allocations within the task. Or, we just don't allow
        //   dynamic allocation within tasks. - Daniel Seitz
        let _g = CriticalSection::begin();
        if self.is_valid() {
            let task = self.task_ref_mut();
            task.destroy();
            true
        } else {
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
        let priority = self.task_ref().priority;
        if self.is_valid() {
            Ok(priority)
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
        let state = self.task_ref().state;
        if self.is_valid() {
            Ok(state)
        } else {
            Err(())
        }
    }

    /// Returns a task's tid (task identifier).
    ///
    /// The tid is a unique identifier that differentiates different tasks even if they have the
    /// same name.
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
        let tid = self.task_ref().tid;
        if self.is_valid() {
            Ok(tid)
        } else {
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
        let name = self.task_ref().name;
        if self.is_valid() {
            Ok(name)
        } else {
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
        let size = self.task_ref().stack.depth();
        if self.is_valid() {
            Ok(size)
        } else {
            Err(())
        }
    }

    /// Check if the task pointed to by this handle is valid.
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
        // UNSAFE: Yes, potentially we're reading from a dangling pointer (if the task has been
        // freed, for instance), but there should be specific values at these locations, and if
        // they aren't there then at least we'll know not to do anymore reads.
        let (tid, valid) = unsafe { ((*self.0).tid, (*self.0).valid) };
        let tid_mask = tid & 0xFF;
        valid == VALID_TASK + tid_mask
    }

    fn task_ref(&self) -> &TaskControl {
        // UNSAFE: This is used internally to the TaskHandle, and is only ever called after
        // checking if the handle is still valid. All operations are within critical sections and
        // so can be considered atomic
        unsafe { &*self.0 }
    }

    fn task_ref_mut(&mut self) -> &mut TaskControl {
        // UNSAFE: This is used internally to the TaskHandle, and is only ever called after
        // checking if the handle is still valid. All operations are within critical sections and
        // so can be considered atomic
        unsafe { &mut *(self.0 as *mut TaskControl) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    // get_task and get_invalid_task are helper functions for other tests
    fn get_task() -> TaskControl {
        // NOTE: We can't return the TaskControl and the TaskHandle as a tuple here because the
        // TaskControl object get's moved on return so we would end up with a dangling pointer
        // in our TaskHandle
        test::create_test_task(512, Priority::Normal, "task test")
    }

    fn get_invalid_task() -> TaskControl {
        let mut task = test::create_test_task(512, Priority::Normal, "invalid test");
        task.valid = INVALID_TASK;
        task
    }

    #[test]
    fn test_task_handle_valid() {
        let mut task = get_task();
        let handle = TaskHandle::new(&task);

        assert!(handle.is_valid());
        task.valid = INVALID_TASK;
        assert!(!handle.is_valid());
    }

    #[test]
    fn test_task_handle_destroy() {
        let task = get_task();
        let mut handle = TaskHandle::new(&task);

        assert!(handle.is_valid());
        assert!(handle.destroy());

        assert!(task.destroy);
        assert!(!handle.is_valid());
    }

    #[test]
    fn test_invalid_task_handle_destroy() {
        let task = get_invalid_task();
        let mut handle = TaskHandle::new(&task);

        assert!(!handle.is_valid());
        assert!(!handle.destroy());
        assert!(!handle.is_valid());
    }

    #[test]
    fn test_task_handle_stack_size() {
        let task = get_task();
        let handle = TaskHandle::new(&task);

        assert_eq!(handle.stack_size(), Ok(512));
    }

    #[test]
    fn test_invalid_task_handle_stack_size() {
        let task = get_invalid_task();
        let handle = TaskHandle::new(&task);

        assert!(handle.stack_size().is_err());
    }

    #[test]
    fn test_task_handle_priority() {
        let task = get_task();
        let handle = TaskHandle::new(&task);

        assert_eq!(handle.priority(), Ok(Priority::Normal));
    }

    #[test]
    fn test_invalid_task_handle_priority() {
        let task = get_invalid_task();
        let handle = TaskHandle::new(&task);

        assert!(handle.priority().is_err());
    }

    #[test]
    fn test_task_handle_state() {
        let task = get_task();
        let handle = TaskHandle::new(&task);

        assert_eq!(handle.state(), Ok(State::Ready));
    }

    #[test]
    fn test_invalid_task_handle_state() {
        let task = get_invalid_task();
        let handle = TaskHandle::new(&task);

        assert!(handle.state().is_err());
    }

    #[test]
    fn test_task_handle_name() {
        let task = get_task();
        let handle = TaskHandle::new(&task);

        assert_eq!(handle.name(), Ok("task test"));
    }

    #[test]
    fn test_invalid_task_handle_name() {
        let task = get_invalid_task();
        let handle = TaskHandle::new(&task);

        assert!(handle.name().is_err());
    }

    #[test]
    fn test_task_handle_tid() {
        let task = get_task();
        let handle = TaskHandle::new(&task);

        assert_eq!(handle.tid(), Ok(task.tid));

    }

    #[test]
    fn test_invalid_task_handle_tid() {
        let task = get_invalid_task();
        let handle = TaskHandle::new(&task);

        assert!(handle.tid().is_err());
    }

    #[test]
    fn test_iter_priority() {
        let mut iter_priority = IterPriority::new();
        assert_eq!(iter_priority.next().unwrap(), Priority::Critical);
        assert_eq!(iter_priority.next().unwrap(), Priority::Normal);
        assert_eq!(iter_priority.next().unwrap(), Priority::Low);
        assert_eq!(iter_priority.next().unwrap(), Priority::__Idle);
        assert_eq!(iter_priority.next(), None);
    }

    #[test]
    fn test_iter_priority_higher() {
        let mut iter_priority_higher = IterPriorityHigher::new(Priority::Normal);
        assert_eq!(iter_priority_higher.next().unwrap(), Priority::Critical);
        assert_eq!(iter_priority_higher.next().unwrap(), Priority::Normal);
        assert_eq!(iter_priority_higher.next(), None);
    }

    #[test]
    fn test_iter_priority_skip() {
        let mut iter_priority_skip = IterPrioritySkip::new(Priority::Normal);
        assert_eq!(iter_priority_skip.next().unwrap(), Priority::Critical);
        assert_eq!(iter_priority_skip.next().unwrap(), Priority::Low);
        assert_eq!(iter_priority_skip.next().unwrap(), Priority::__Idle);
        assert_eq!(iter_priority_skip.next(), None);
    }
}

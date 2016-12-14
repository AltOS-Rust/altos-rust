// task/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! Task creation, scheduling  and system calls.
//!
//! This module contains the functions used to create tasks and modify them within the kernel. It
//! also contains the code for the scheduler.

pub mod public;
pub mod args;
mod task_control;

use self::task_control::{TaskControl, State};
pub use self::task_control::{TaskHandle, Priority};
use timer::Timer;
use self::args::Args;
use queue::{Queue, SyncQueue, Node};
use alloc::boxed::Box;
use core::ops::Index;
use sync::CriticalSection;
use sync::MutexGuard;

const NUM_PRIORITIES: usize = 4;

/// An alias for the channel to sleep on that will never be awoken
pub const FOREVER_CHAN: usize = 0;

/// The current task.
///
/// This keeps track of the currently running task, this should always be `Some` unless the task is
/// actively being switched out or the scheduler has not been started.
#[no_mangle]
#[doc(hidden)]
pub static mut CURRENT_TASK: Option<Box<Node<TaskControl>>> = None;

static PRIORITY_QUEUES: [SyncQueue<TaskControl>; NUM_PRIORITIES] = [SyncQueue::new(),
                                                                    SyncQueue::new(), 
                                                                    SyncQueue::new(), 
                                                                    SyncQueue::new()];
static DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();
static OVERFLOW_DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();

impl Index<Priority> for [SyncQueue<TaskControl>] {
  type Output = SyncQueue<TaskControl>;
  fn index(&self, idx: Priority) -> &Self::Output {
    &self[idx as usize]
  }
}

/// Select a new task to run and switch its context, this function MUST only be called from the
/// PendSV handler, calling it from elsewhere could lead to undefined behavior. It must be exposed
/// publicly so that the compiler doesn't optimize it away when compiling for release.
#[no_mangle]
#[doc(hidden)]
pub unsafe fn switch_context() {
  if !is_kernel_running() {
    panic!("switch_context - This function should only get called from kernel code!");
  }
  match CURRENT_TASK.take() {
    Some(mut running) => {
      if running.destroy {
        drop(running);
      }
      else {
        let queue_index = running.priority;
        if running.is_stack_overflowed() {
          panic!("switch_context - The current task's stack overflowed!");
        }
        if running.state == State::Blocked {
          if running.overflowed {
            OVERFLOW_DELAY_QUEUE.enqueue(running);
          }
          else {
            DELAY_QUEUE.enqueue(running);
          }
        }
        else {
          running.state = State::Ready;
          PRIORITY_QUEUES[queue_index].enqueue(running);
        }
      }

      'main: loop {
        for i in Priority::all() {
          while let Some(mut new_task) = PRIORITY_QUEUES[i].dequeue() {
            if new_task.destroy {
              drop(new_task);
            }
            else {
              new_task.state = State::Running;
              CURRENT_TASK = Some(new_task);
              break 'main;
            }
          }
        }
      }
    },
    None => panic!("switch_context - current task doesn't exist!"),
  }
}

/// Creates a new task and put it into the task queue for running. It returns a `TaskHandle` to
/// monitor the task with
///
/// `new_task` takes several arguments, a `fn(&Args)` pointer which specifies the code to run for
/// the task, an `Args` argument for the arguments that will be passed to the task, a `usize`
/// argument for how much space should be allocated for the task's stack, a `Priority` argument for
/// the priority that the task should run at, and a `&str` argument to give the task a readable
/// name.
///
/// # Examples
///
/// ```no_run
/// use altos_core::task::{start_scheduler, new_task, Priority};
/// use altos_core::task::args::Args;
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
#[inline(never)]
pub fn new_task(code: fn(&mut Args), args: Args, stack_depth: usize, priority: Priority, name: &'static str) -> TaskHandle {
  // Make sure the task is allocated in one fell swoop
  let g = CriticalSection::begin();
  let task = Box::new(Node::new(TaskControl::new(code, args, stack_depth, priority, name)));
  drop(g);

  let handle = TaskHandle::new(&**task);
  PRIORITY_QUEUES[task.priority].enqueue(task); 
  handle
}

/// Yield the current task to the scheduler so another task can run.
///
/// # Examples
///
/// ```no_run
/// use altos_core::task::yield_task;
/// use altos_core::task::args::Args;
///
/// fn test_task(_args: &Args) {
///   loop {
///     // Do some important work...
///   
///     // Okay, we're done...
///     yield_task();
///     // Go back and do it again
///   }
/// }
/// ```
pub fn yield_task() {
  unsafe { ::yield_cpu() };
}

/// Put the current task to sleep, waiting on a channel to be woken up.
///
/// `sleep` takes a `usize` argument that acts as an identifier for when to wake up the task. The
/// task will sleep indefinitely if no wakeup signal is sent.
///
/// # Examples
///
/// ```no_run
/// use altos_core::task::sleep;
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
  sleep_for(wchan, 0);
}

/// Put the current task to sleep with a timeout, waiting on a channel to be woken up.
///
/// `sleep_for` takes a `usize` argument that acts as an identifier to wake up the task. It also
/// takes a second `usize` argument for the maximum time it should sleep before waking.
///
/// # Examples
///
/// ```no_run
/// use altos_core::task::{sleep_for, FOREVER_CHAN};
///
/// // Sleep for 300 ticks
/// sleep_for(FOREVER_CHAN, 300);
/// ```
pub fn sleep_for(wchan: usize, delay: usize) {
  let _g = CriticalSection::begin();
  unsafe {
    if let Some(current) = CURRENT_TASK.as_mut() {
      let ticks = Timer::get_current().msec;
      current.wchan = wchan;
      current.state = State::Blocked;
      current.delay = ticks + delay;
      if ticks + delay < ticks {
        current.overflowed = true;
      }
    }
    else {
      panic!("sleep_for - current task doesn't exist!");
    }
  }
  yield_task();
}

/// Wake up all tasks sleeping on a channel.
///
/// `wake` takes a `usize` argument that acts as an identifier to only wake up tasks sleeping on
/// that same identifier. 
pub fn wake(wchan: usize) {
  let _g = CriticalSection::begin();
  let mut to_wake: Queue<TaskControl> = DELAY_QUEUE.remove(|task| task.wchan == wchan);
  to_wake.append(OVERFLOW_DELAY_QUEUE.remove(|task| task.wchan == wchan));
  for mut task in to_wake.into_iter() {
    task.wchan = 0;
    task.state = State::Ready;
    PRIORITY_QUEUES[task.priority].enqueue(task);
  }
}

#[doc(hidden)]
pub fn system_tick() {
  if !is_kernel_running() {
    panic!("alarm_wake - This function should only be called from kernel code!");
  }
  let _g = CriticalSection::begin();
  Timer::tick();
  alarm_wake();

  let current_priority = unsafe { 
    match CURRENT_TASK.as_ref() {
      Some(task) => task.priority,
      None => panic!("system_tick - current task doesn't exist!"),
    }
  };
  
  for i in current_priority.higher() {
    if !PRIORITY_QUEUES[i].is_empty() {
      // Only context switch if there's another task at the same or higher priority level
      yield_task();
      break;
    }
  }
}

/// Start running the first task in the queue
pub fn start_scheduler() {
    init_idle_task();
    unsafe {
      for i in Priority::all() {
        if let Some(mut task) = PRIORITY_QUEUES[i].dequeue() {
          task.state = State::Running;
          CURRENT_TASK = Some(task);
          break;
        }
      }
      debug_assert!(CURRENT_TASK.is_some());
      ::start_first_task();
    }
}

#[doc(hidden)]
pub fn condvar_wait<'a, T>(wchan: usize, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
  let g = CriticalSection::begin();

  // Get a reference to the locked mutex
  let mutex = ::sync::mutex_from_guard(&guard);

  // unlock the mutex
  drop(guard);

  // Sleep on the cond var channel
  sleep(wchan);
  
  // finish critical section so we can context switch
  drop(g);
  
  // re-acquire lock before returning
  mutex.lock()
}

fn is_kernel_running() -> bool {
  unsafe { ::in_kernel_mode() }
}

fn alarm_wake() {
  if !is_kernel_running() {
    panic!("alarm_wake - This function should only be called from kernel code!");
  }

  let ticks = Timer::get_current().msec;
  
  let to_wake: Queue<TaskControl> = DELAY_QUEUE.remove(|task| task.delay <= ticks && task.wchan == FOREVER_CHAN);
  for mut task in to_wake.into_iter() {
    task.wchan = 0;
    task.state = State::Ready;
    task.delay = 0;
    PRIORITY_QUEUES[task.priority].enqueue(task);
  }

  if ticks == !0 {
    let mut overflowed: Queue<TaskControl> = OVERFLOW_DELAY_QUEUE.remove_all();
    for task in overflowed.iter_mut() {
      task.overflowed = false;
    }
    DELAY_QUEUE.append(overflowed);
  }
}

fn init_idle_task() {
  let task = TaskControl::new(idle_task_code, Args::empty(), 256, Priority::__Idle, "idle");

  PRIORITY_QUEUES[task.priority].enqueue(Box::new(Node::new(task)));
}

fn idle_task_code(_args: &mut Args) {
  loop {
    #[cfg(target_arch="arm")]
    unsafe {
      asm!("wfi");
    }
    yield_task();
  }
}


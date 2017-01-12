// sched/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! Scheduling
//!
//! This module contains the code for the scheduler and initialization.

use task::{self, TaskControl, Priority, State};
use queue::{SyncQueue, Node};
use alloc::boxed::Box;
use core::ops::Index;
use task::NUM_PRIORITIES;
use arch;

/// The current task.
///
/// This keeps track of the currently running task, this should always be `Some` unless the task is
/// actively being switched out or the scheduler has not been started.
#[no_mangle]
#[doc(hidden)]
pub static mut CURRENT_TASK: Option<Box<Node<TaskControl>>> = None;

pub static PRIORITY_QUEUES: [SyncQueue<TaskControl>; NUM_PRIORITIES] = [SyncQueue::new(),
                                                                    SyncQueue::new(), 
                                                                    SyncQueue::new(), 
                                                                    SyncQueue::new()];
pub static DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();
pub static OVERFLOW_DELAY_QUEUE: SyncQueue<TaskControl> = SyncQueue::new();

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
  /*
  if !is_kernel_running() {
    panic!("switch_context - This function should only get called from kernel code!");
  }
  */
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

/// Start running the first task in the queue
pub fn start_scheduler() {
    task::init_idle_task();
    unsafe {
      for i in Priority::all() {
        if let Some(mut task) = PRIORITY_QUEUES[i].dequeue() {
          task.state = State::Running;
          CURRENT_TASK = Some(task);
          break;
        }
      }
      debug_assert!(CURRENT_TASK.is_some());
      arch::start_first_task();
    }
}

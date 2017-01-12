// task/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! Task creation
//!
//! This module contains the functions used to create tasks and modify them within the kernel.

pub mod args;
mod stack;
mod control;

pub use self::control::{TaskHandle, TaskControl, State, Priority};
pub use self::control::NUM_PRIORITIES;

use args::Args;

#[doc(hidden)]
pub fn init_idle_task() {
  use sched::PRIORITY_QUEUES;
  use queue::Node;
  use alloc::boxed::Box;
  const INIT_TASK_STACK_SIZE: usize = 256;

  let task = TaskControl::new(idle_task_code, Args::empty(), INIT_TASK_STACK_SIZE, Priority::__Idle, "idle");

  PRIORITY_QUEUES[task.priority].enqueue(Box::new(Node::new(task)));
}

fn idle_task_code(_args: &mut Args) {
  use syscall::sched_yield;

  loop {
    sched_yield();
  }
}

// arch/test.rs
// AltOS Rust
//
// Created by Daniel Seitz on 1/7/17

//! This module is used to provide stubs for the architecture layer for testing.

use volatile::Volatile;
use task::args::Args;
use alloc::boxed::Box;
use sched;

pub fn yield_cpu() {
  // no-op
  sched::switch_context();
}

pub fn initialize_stack(stack_ptr: Volatile<usize>, _code: fn(&mut Args), _args: &Box<Args>) -> usize {
  // no-op
  stack_ptr.as_ptr() as usize
}

pub fn start_first_task() {
  // no-op
}
pub fn in_kernel_mode() -> bool {
  // no-op
  true
}

pub fn begin_critical() -> usize {
  // no-op
  0
}

pub fn end_critical(_mask: usize) {
  // no-op
}

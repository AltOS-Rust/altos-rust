// arch/unknown.rs
// AltOS Rust
//
// Created by Daniel Seitz on 1/7/17

//! This module is used to provide stubs for the architecture layer

use volatile::Volatile;
use task::args::Args;
use alloc::boxed::Box;
use sched;
use core::fmt;

extern "Rust" {
  fn __yield_cpu();
  fn __initialize_stack(stack_ptr: Volatile<usize>, code: fn(&mut Args), args: &Box<Args>) -> usize;
  fn __start_first_task();
  fn __in_kernel_mode() -> bool;
  fn __begin_critical() -> usize;
  fn __end_critical(mask: usize);
  fn __debug_fmt(args: fmt::Arguments);
}

pub fn yield_cpu() {
  unsafe { __yield_cpu() };
}

pub fn initialize_stack(stack_ptr: Volatile<usize>, code: fn(&mut Args), args: &Box<Args>) -> usize {
  unsafe { __initialize_stack(stack_ptr, code, args) }
}

pub fn start_first_task() {
  unsafe { __start_first_task() };
}

pub fn in_kernel_mode() -> bool {
  unsafe { __in_kernel_mode() }
}

pub fn begin_critical() -> usize {
  unsafe { __begin_critical() }
}

pub fn end_critical(mask: usize) {
  unsafe { __end_critical(mask) };
}

pub fn debug_fmt(args: fmt::Arguments) {
  unsafe { __debug_fmt(args) };
}

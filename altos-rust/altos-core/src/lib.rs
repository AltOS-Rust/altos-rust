// altos-core/lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/8/16

// TODO: Add more description for the AltOSRust operation system
//! AltOSRust microkernel for embedded devices.
//!
//! This microkernel provides task creation and scheduling for applications running on embedded
//! devices.

#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(collections)]
#![feature(drop_types_in_const)]
#![feature(cfg_target_has_atomic)]
#![warn(missing_docs)]
#![no_std]

#[cfg(not(test))]
extern crate bump_allocator;
pub extern crate alloc;
pub extern crate collections;
#[cfg(not(target_has_atomic="ptr"))]
pub extern crate cm0_atomic as atomic;

pub mod timer;
pub mod volatile;
pub mod task;
pub mod sync;
pub mod queue;
pub mod init;

#[cfg(target_has_atomic="ptr")]
pub use core::sync::atomic as atomic;
use task::args::Args;
pub use task::{CURRENT_TASK, switch_context};
use alloc::boxed::Box;

// List of methods we'll likely need from port layer
#[allow(improper_ctypes)] // We're only interfacing with other Rust modules, but we can't have any explicit circular dependencies
extern "Rust" {
  fn yield_cpu();
  fn initialize_stack(stack_ptr: volatile::Volatile<usize>, code: fn(&mut Args), args: &Box<Args>) -> usize;
  fn start_first_task();
  fn in_kernel_mode() -> bool;

  // Provided by the portability layer to ensure that interrupts are disabled for a critical section
  fn begin_critical() -> usize;
  fn end_critical(mask: usize);
}

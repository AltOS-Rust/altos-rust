// altos-core/lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/8/16

// TODO: Add more description for the AltOSRust operation system
//! AltOSRust microkernel for embedded devices.
//!
//! This microkernel provides task creation and scheduling for applications running on embedded
//! devices.

#![feature(asm)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(collections)]
#![feature(drop_types_in_const)]
#![feature(cfg_target_has_atomic)]
#![feature(heap_api)]
#![feature(oom)]
#![warn(missing_docs)]
#![no_std]

#[cfg(all(not(test), feature="bump_allocator"))]
extern crate bump_allocator as allocator;

pub extern crate alloc;
pub extern crate collections;
#[cfg(not(target_has_atomic="ptr"))]
pub extern crate cm0_atomic as atomic;
pub extern crate volatile;

#[cfg(target_os="altos-cm0")]
#[path = "arch/cm0.rs"]
mod arch;

// FIXME: Figure out this cfg
#[cfg(any(test, not(target_os="altos-cm0")))]
#[path = "arch/test.rs"]
mod arch;

pub mod tick;
pub mod syscall;
mod task;
mod sched;
pub mod sync;
pub mod queue;
pub mod init;

#[cfg(target_has_atomic="ptr")]
pub use core::sync::atomic as atomic;
pub use task::{TaskHandle, Priority};
pub use sched::{CURRENT_TASK, switch_context, start_scheduler};
pub use task::args;

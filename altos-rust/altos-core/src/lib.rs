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
#![deny(trivial_numeric_casts)]
#![no_std]

#[cfg(any(test, feature="test"))]
#[macro_use]
extern crate std;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::debug_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(all(not(test), not(feature="test"), feature="free_list_allocator"))]
extern crate free_list_allocator as allocator;

pub extern crate alloc;
pub extern crate collections;
#[cfg(not(target_has_atomic="ptr"))]
pub extern crate cm0_atomic as atomic;
pub extern crate volatile;

#[cfg(test)]
#[macro_use]
mod test;

#[cfg(all(not(test), target_arch="arm", feature="cm0"))]
#[path = "arch/cm0.rs"]
mod arch;

#[cfg(any(test, feature="test"))]
#[path = "arch/test.rs"]
mod arch;

#[cfg(all(not(feature="test"), not(feature="cm0")))]
#[path = "arch/unknown.rs"]
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
pub use arch::debug_print;

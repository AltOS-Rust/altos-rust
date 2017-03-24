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

//! This module is used to provide stubs for the architecture layer.

use volatile::Volatile;
use task::args::Args;
use alloc::boxed::Box;
use sched;

extern "Rust" {
    fn __yield_cpu();
    fn __initialize_stack(stack_ptr: Volatile<usize>, code: fn(&mut Args), args: &Box<Args>) -> usize;
    fn __start_first_task();
    fn __in_kernel_mode() -> bool;
    fn __begin_critical() -> usize;
    fn __end_critical(mask: usize);
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

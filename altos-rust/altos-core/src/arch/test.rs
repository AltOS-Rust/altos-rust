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

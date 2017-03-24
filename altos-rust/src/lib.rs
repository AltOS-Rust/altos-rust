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

// This module is the entry point to the operating system.
// Application tasks are created here.

#![feature(const_fn)]
#![no_std]
#![allow(dead_code)]

#![allow(unused_imports)]
#[macro_use]
extern crate cortex_m0;

use cortex_m0::arm;
use cortex_m0::kernel;
use cortex_m0::time;
use cortex_m0::kernel::task::Priority;
use cortex_m0::kernel::task::args::Args;
use cortex_m0::kernel::sync::Mutex;
use cortex_m0::peripheral::gpio::{self, Port};
use cortex_m0::io;

mod shell;

#[no_mangle]
pub fn application_entry() -> ! {
    // -----------------
    // Tasks go between the lines.
    // ----------------
    kernel::syscall::new_task(shell::shell, Args::empty(), 1024, Priority::Normal, "shell");
    kernel::task::start_scheduler();

    loop { unsafe { arm::asm::bkpt() }; }
}

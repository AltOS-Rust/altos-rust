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
use cortex_m0::kernel::sync::{Mutex, RawMutex};
use cortex_m0::peripheral::gpio::{self, Port};
use cortex_m0::io;
use kernel::syscall;

static TEST_MUTEX: RawMutex = RawMutex::new();

#[no_mangle]
pub fn application_entry() -> ! {
    // -----------------
    // Tasks go between the lines.
    // ----------------
    kernel::syscall::new_task(mutex_task, Args::empty(), 1024, Priority::Normal, "syscall");
    kernel::syscall::new_task(mutex_task_2, Args::empty(), 1024, Priority::Normal, "syscall");
    kernel::task::start_scheduler();

    loop { unsafe { arm::asm::bkpt() }; }
}

fn print_task(_args: &mut Args) {
    loop {
        println!("Hello world with a new syscall interface!");
    }
}

fn exit_task(_args: &mut Args) {
    loop {
        println!("Exiting!");
        syscall::exit();
    }
}

fn delay_task(_args: &mut Args) {
    let mut val = 0;
    loop {
        println!("About to sleep... Value is {}", val);
        time::delay_s(1);
        val += 1;
    }
}

fn mutex_task(_args: &mut Args) {
    loop {
        let res = syscall::mutex_try_lock(&TEST_MUTEX);
        println!("(task 1) Result of try lock is: {}", res);
        time::delay_ms(2000);
    }
}

fn mutex_task_2(_args: &mut Args) {
    loop {
        let res = syscall::mutex_try_lock(&TEST_MUTEX);
        println!("(task 2) Result of try lock is: {}", res);
        time::delay_ms(2000);
    }
}

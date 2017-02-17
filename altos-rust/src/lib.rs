// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

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


#[no_mangle]
pub fn application_entry() -> ! {
    // -----------------
    // Tasks go between the lines.
    // ----------------
    kernel::task::start_scheduler();

    loop { unsafe { arm::asm::bkpt() }; }
}


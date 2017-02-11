// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

#![feature(const_fn)]
#![no_std]
#![allow(dead_code)]

#[macro_use]
extern crate cortex_m0;

use cortex_m0::arm;
use cortex_m0::kernel;
use cortex_m0::time;
use cortex_m0::kernel::task::Priority;
use cortex_m0::kernel::task::args::Args;
use cortex_m0::kernel::sync::Mutex;
use cortex_m0::peripheral::gpio::{self, Port};

// Since we can't statically create a port object (maybe we should be able to?) we make it an
// option then initialize it down in `application_entry`
static LED: Mutex<Option<Port>> = Mutex::new(None);

#[no_mangle]
pub fn application_entry() -> ! {
    // Initialize the LED lock
    {
        let mut led = LED.lock();
        *led = Some(Port::new(3, gpio::Group::B));
    }

//    kernel::syscall::new_task(hello_task, Args::empty(), 1024, Priority::Normal, "hello_task");
//    kernel::syscall::new_task(goodbye_task, Args::empty(), 1024, Priority::Normal, "goodbye_task");
    kernel::task::start_scheduler();

    loop { unsafe { arm::asm::bkpt() }; }
}

fn hello_task(_args: &mut Args) {
    let mut i: u32 = 0;
    loop {
        println!("Hello World");
        for _ in 0 .. 1_000 {
            i.wrapping_add(1);
        }
    }
}

fn goodbye_task(_args: &mut Args) {
    let mut i: u32 = 0;
    loop {
        println!("Goodbye World");
        for _ in 0 .. 1_000 {
            i.wrapping_add(1);
        }
    }
}

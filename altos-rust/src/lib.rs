// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

#![feature(const_fn)]
#![no_std]
#![allow(dead_code)]

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

  kernel::syscall::new_task(blink_1, Args::empty(), 512, Priority::Normal, "blink_1");
  kernel::syscall::new_task(blink_2, Args::empty(), 512, Priority::Normal, "blink_1");
  kernel::task::start_scheduler();

  loop { unsafe { arm::asm::bkpt() }; }
}

fn blink_1(_args: &mut Args) {
  loop {
    // Grab the LED lock
    let guard = LED.lock();
    {
      // Get a reference to the underlying port
      let led = guard.as_ref().unwrap();
      // Blink 10 times at 100 ms intervals
      for _ in 0..10 {
        led.set();
        time::delay_ms(100);
        led.reset();
        time::delay_ms(100);
      }
    }
    // Release the lock before yielding our time slice
    drop(guard);
    kernel::syscall::sched_yield();
  }
}

fn blink_2(_args: &mut Args) {
  loop {
    let guard = LED.lock();
    {
      let led = guard.as_ref().unwrap();
      for _ in 0..5 {
        led.set();
        time::delay_ms(500);
        led.reset();
        time::delay_ms(500);
      }
    }
    drop(guard);
    kernel::syscall::sched_yield();
  }
}

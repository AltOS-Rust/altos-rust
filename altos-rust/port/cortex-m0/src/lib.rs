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

//! This crate is the hardware interface for the cortex-m0 processor.

#![warn(missing_docs)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(drop_types_in_const)]
#![allow(dead_code)]
#![feature(linkage)]
//#![feature(compiler_builtins_lib)] // Keep this around in case we want to try and get it working
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate altos_core;
#[allow(unused_imports)]
#[macro_use]
extern crate altos_macros;

pub extern crate arm;
//pub extern crate compiler_builtins; // See above comment

#[cfg(test)]
mod test;

pub mod io;
pub mod exceptions;
pub mod interrupt;
pub mod system_control;
pub mod peripheral;
pub mod time;

use peripheral::gpio;
use peripheral::rcc;
use peripheral::systick;

#[cfg(target_arch="arm")]
pub use exceptions::EXCEPTIONS;

use altos_core::volatile;

/// Re-exports a subset of the core operating system interface.
///
/// This is to enable a higher level of control as to what the user
/// can access.
pub mod kernel {
    pub use altos_core::syscall;
    /// Types and functions related to the creation, running, and destruction of a Task.
    pub mod task {
        pub use altos_core::args;
        pub use altos_core::TaskHandle;
        pub use altos_core::{start_scheduler};
        pub use altos_core::{Priority};
    }
    /// Allocation interface to allow dynamic allocation.
    pub mod alloc {
        pub use altos_core::alloc::boxed::Box;
    }
    /// Collection types for storing data on the heap.
    pub mod collections {
        pub use altos_core::collections::Vec;
        pub use altos_core::queue::{SortedList, Queue, Node};
    }
    /// Synchronization primitives.
    pub mod sync {
        pub use altos_core::sync::{RawMutex, Mutex, MutexGuard};
        pub use altos_core::sync::CondVar;
        pub use altos_core::sync::CriticalSection;
    }
}

#[cfg(not(any(test, feature="doc")))]
#[lang = "eh_personality"] extern "C" fn eh_personality() {}
#[cfg(not(any(test, feature="doc")))]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt(fmt: core::fmt::Arguments, (file, line): (&'static str, u32)) -> ! {
    unsafe { arm::asm::disable_interrupts() };
    kprintln!("Panicked at File: {}, Line: {}", file, line);
    kprintln!("{}", fmt);
    loop {
        unsafe {
            arm::asm::bkpt();
        }
    }
}

extern "Rust" {
    // The application layer's entry point
    fn application_entry() -> !;
}

#[doc(hidden)]
#[export_name="_reset"]
pub fn init() -> ! {
    // TODO: set pendsv and systick interrupts to lowest priority
    unsafe { arm::asm::disable_interrupts() };

    /*
    unsafe {
        asm!("svc 0" : : : : "volatile");
    }
    */
    init_data_segment();
    init_bss_segment();
    init_heap();
    init_led();
    init_clock();
    init_ticks();
    init_usart();

    unsafe { application_entry() };
}

fn init_data_segment() {
    #[cfg(target_arch="arm")]
    unsafe {
        asm!(
            concat!(
                "ldr r1, =_sidata\n", /* start of data in flash */
                "ldr r2, =_sdata\n",  /* start of memory location in RAM */
                "ldr r3, =_edata\n",  /* end of memory location in RAM */
                "copy:\n",
                "cmp r2, r3\n", /* check if we've reached the end of our segment */
                "bpl d_done\n",
                "ldr r0, [r1]\n", /* if not, keep copying */
                "adds r1, #4\n",
                "str r0, [r2]\n",
                "adds r2, #4\n",
                "b copy\n", /* repeat until done */
                "d_done:\n"
            )
            : /* no outputs */
            : /* no inputs */
            : "r0", "r1", "r2", "r3" /* clobbers */
            : "volatile"
        );
    }
}

fn init_bss_segment() {
    #[cfg(target_arch="arm")]
    unsafe {
        asm!(
            concat!(
                "movs r0, #0\n", /* store zero for later */
                "ldr r1, =_sbss\n", /* start of bss in RAM */
                "ldr r2, =_ebss\n", /* end of bss in RAM */
                "loop:\n",
                "cmp r1, r2\n", /* check if we've reached the end of our segment */
                "bpl b_done\n",
                "str r0, [r1]\n", /* if not, zero out memory at current location */
                "adds r1, #4\n",
                "b loop\n", /* repeat until done */
                "b_done:\n"
            )
            : /* no outputs */
            : /* no inputs */
            : "r0", "r1", "r2" /* clobbers */
            : "volatile"
        );
    }
}

fn init_heap() {
    #[cfg(target_arch="arm")]
    unsafe {
        let heap_start: usize;
        let heap_size: usize;
        asm!(
            concat!(
                "ldr r0, =_heap_start\n",
                "ldr r1, =_heap_end\n",
                "subs r2, r1, r0\n"
            )
            : "={r0}"(heap_start), "={r2}"(heap_size)
            : /* no inputs */
            : "r0", "r1", "r2"
            : "volatile"
        );
        altos_core::init::init_heap(heap_start, heap_size);
    }
}

fn init_led() {
    gpio::GPIO::enable(gpio::Group::B);

    let mut pb3 = gpio::Port::new(3, gpio::Group::B);
    pb3.set_mode(gpio::Mode::Output);
    pb3.set_type(gpio::Type::PushPull);
}

fn init_clock() {
    let mut rcc = rcc::rcc();

    // 12 is the max we can go since our input clock is (8MHz / 2)
    let clock_multiplier: u8 = 12;

    // PLL must be off in order to configure
    rcc.disable_clock(rcc::Clock::PLL);

    // Make sure HSI is the PLL source clock
    rcc.set_pll_source(rcc::Clock::HSI);

    // Set the multiplier... DO NOT EXCEED 48 MHz
    rcc.set_pll_multiplier(clock_multiplier);

    // Enable the PLL clock
    rcc.enable_clock(rcc::Clock::PLL);

    // Wait for it to be ready
    while !rcc.clock_is_ready(rcc::Clock::PLL) {}
    // Switch over to the PLL for running the system
    rcc.set_system_clock_source(rcc::Clock::PLL);

    // Our system clock sets itself to interrupt every 1 ms
    time::set_resolution(1);
}

fn init_ticks() {
    let mut systick = systick::systick();

    systick.use_processor_clock();
    systick.clear_current_value();
    systick.enable_counter();
    systick.enable_interrupts();

}

fn init_usart() {
    #[cfg(feature="serial")]
    peripheral::usart::init();
}


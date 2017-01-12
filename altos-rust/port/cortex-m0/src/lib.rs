// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

#![feature(lang_items)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(const_fn)]
#![feature(drop_types_in_const)] // Probably can come back and remove this later
#![allow(dead_code)]
#![feature(linkage)]
#![no_std]

extern crate altos_core;

pub extern crate arm;

mod exceptions;
pub mod peripheral;
pub mod time;
mod interrupt;
mod system_control;

use peripheral::gpio;
use peripheral::rcc;
use peripheral::systick;

#[cfg(target_arch="arm")]
pub use vector_table::RESET;
#[cfg(target_arch="arm")]
pub use exceptions::EXCEPTIONS;

use altos_core::volatile;

pub mod kernel {
  pub use altos_core::syscall;

  pub mod task {
    pub use altos_core::args;
    pub use altos_core::TaskHandle;
    pub use altos_core::{start_scheduler};
    pub use altos_core::{Priority};
  }
  
  // TODO: Do we want to expose an allocation interface?
  pub mod alloc {
    pub use altos_core::alloc::boxed::Box;
  }

  pub mod collections {
    // TODO: Do we want to expose an allocation interface?
    pub use altos_core::collections::Vec;
    pub use altos_core::queue::{SortedList, Queue, Node};
  }

  pub mod sync {
    pub use altos_core::sync::{Mutex, MutexGuard};
    pub use altos_core::sync::CondVar;
    pub use altos_core::sync::CriticalSection;
  }
}

#[cfg(not(test))]
#[lang = "eh_personality"] extern "C" fn eh_personality() {}
#[cfg(not(test))]
#[lang = "panic_fmt"] 
extern "C" fn panic_fmt(_fmt: core::fmt::Arguments, _file: &'static str, _line: usize) -> ! {
  loop {
    unsafe {
      arm::asm::bkpt();
    }
  }
}

extern {
  // The application layer's entry point
  fn application_entry() -> !;
}

#[no_mangle]
pub fn init() -> ! {
  // TODO: set pendsv and systick interrupts to lowest priority
  unsafe { arm::asm::disable_interrupts() };
  init_data_segment();
  init_bss_segment();
  init_heap();
  init_led();
  init_clock();
  init_ticks();

  unsafe { application_entry() };
}

// TODO: Do we want to keep this linker section or just expose `init` as a special symbol?
#[cfg(target_arch="arm")]
mod vector_table {
  #[link_section = ".reset"]
  #[no_mangle]
  pub static RESET: fn() -> ! = ::init;
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
      "d_done:\n")
    : /* no outputs */ 
    : /* no inputs */ 
    : "r0", "r1", "r2", "r3" /* clobbers */
    : "volatile");  
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
      "b_done:\n")
    : /* no outputs */
    : /* no inputs */
    : "r0", "r1", "r2" /* clobbers */
    : "volatile");
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

        "subs r2, r1, r0\n")
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
  let rcc = rcc::rcc();

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
  let systick = systick::systick();

  systick.use_processor_clock();
  systick.clear_current_value();
  systick.enable_counter();
  systick.enable_interrupts();

}

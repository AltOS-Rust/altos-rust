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
#![no_std]

extern crate altos_core;
pub extern crate arm;

mod exceptions;
pub mod peripheral;
mod interrupt;
mod system_control;

use peripheral::gpio;
use peripheral::rcc;
use peripheral::systick;

#[doc(hidden)]
#[cfg(target_arch="arm")]
pub use vector_table::RESET;
#[doc(hidden)]
#[cfg(target_arch="arm")]
pub use exceptions::EXCEPTIONS;
use altos_core::alloc::boxed::Box;

use altos_core::task::args::Args;
use altos_core::task;
use altos_core::volatile;

pub mod kernel {
  pub use altos_core::task::public as task;

  pub mod alloc {
    pub use altos_core::alloc::boxed::Box;
  }

  pub mod collections {
    pub use altos_core::collections::Vec;
    pub use altos_core::queue::{SortedList, Queue, Node};
  }

  pub mod sync {
    pub use altos_core::sync::{Mutex, MutexGuard};
    pub use altos_core::sync::CondVar;
  }

  pub mod timer {
    pub use altos_core::timer::Timer;
  }
}

#[no_mangle]
pub fn yield_cpu() {
  let scb = system_control::scb();
  scb.set_pend_sv();
}

#[no_mangle]
pub fn initialize_stack(mut stack_ptr: volatile::Volatile<usize>, code: fn(&mut Args), args: &Box<Args>) -> usize {
  const INITIAL_XPSR: usize = 0x0100_0000;
  unsafe {
    // Offset added to account for way MCU uses stack on entry/exit of interrupts
    stack_ptr -= 4;
    stack_ptr.store(INITIAL_XPSR); /* xPSR */
    stack_ptr -= 4;
    stack_ptr.store(code as usize); /* PC */
    stack_ptr -= 4;
    stack_ptr.store(exit_error as usize); /* LR */
    stack_ptr -= 20; /* R12, R3, R2, R1 */
    stack_ptr.store(&**args as *const _ as usize); /* R0 */
    stack_ptr -= 32; /* R11..R4 */
    stack_ptr.as_ptr() as usize
  }
}

#[no_mangle]
#[inline(never)]
pub fn start_first_task() {
  unsafe {
    #![cfg(target_arch="arm")]
    asm!(
      concat!(
          "ldr r2, current_task_const_2\n", /* get location of current_task */
          "ldr r3, [r2]\n",
          "ldr r0, [r3]\n",

          "adds r0, #32\n", /* discard everything up to r0 */
          "msr psp, r0\n", /* this is the new top of stack to use for the task */

          "movs r0, #2\n", /* switch to the psp stack */
          "msr CONTROL, r0\n", /* we're using psp instead of msp now */

          "isb\n", /* instruction barrier */

          "pop {r0-r5}\n", /* pop the registers that are saved automatically */
          "mov lr, r5\n", /* lr is now in r5, so put it back where it belongs */
          "pop {r3}\n", /* pop return address (old pc) into r3 */
          "pop {r2}\n", /* pop and discard xPSR */
          "cpsie i\n", /* first task has its context, so interrupts can be enabled */
          "bx r3\n", /* start executing user code */

           ".align 4\n",
          "current_task_const_2: .word CURRENT_TASK\n")
      : /* no outputs */
      : /* no inputs */
      : /* no clobbers */
      : "volatile");
  }
}

#[no_mangle]
pub fn in_kernel_mode() -> bool {
  if cfg!(target_arch = "arm") {
    const MAIN_STACK: usize = 0b0;
    const PROGRAM_STACK: usize = 0b10;
    unsafe {
      let stack_mask: usize;
      asm!("mrs $0, CONTROL\n" /* get the stack control mask */
        : "=r"(stack_mask)
        : /* no inputs */
        : /* no clobbers */
        : "volatile");
      stack_mask == MAIN_STACK
    }
  }
  else {
    true
  }
}

#[no_mangle]
pub fn begin_critical() -> usize {
  if cfg!(target_arch = "arm") {
    let primask: usize;
    unsafe {
      asm!(
        concat!(
          "mrs $0, PRIMASK\n",
          "cpsid i\n")
        : "=r"(primask)
        : /* no inputs */
        : /* no clobbers */
        : "volatile");
    }
    primask
  } 
  else {
    0
  }
}

#[no_mangle]
pub fn end_critical(primask: usize) {
  #[cfg(target_arch="arm")]
  unsafe {
    asm!("msr PRIMASK, $0"
      : /* no outputs */
      : "r"(primask)
      : /* no clobbers */
      : "volatile");
  }
}

fn exit_error() {
  unsafe {
    ::arm::asm::bkpt();
    loop{}
  }
}

#[cfg(not(test))]
#[lang = "eh_personality"] extern fn eh_personality() {}
#[cfg(not(test))]
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{unsafe {arm::asm::bkpt();}}}

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
}

fn init_ticks() {
  let systick = systick::systick();

  systick.use_processor_clock();
  systick.clear_current_value();
  systick.enable_counter();
  systick.enable_interrupts();

}

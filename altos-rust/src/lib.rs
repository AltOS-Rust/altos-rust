// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

#![feature(const_fn)]
#![no_std]
#![allow(dead_code)]

extern crate cortex_m0;

use cortex_m0::arm;

#[no_mangle]
pub fn application_entry() -> ! {
  loop { unsafe { arm::asm::bkpt() }; }
}

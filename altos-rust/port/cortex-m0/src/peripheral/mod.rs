// peripheral/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! This module handles the memory mapped peripherals that are a part of the Cortex-M0. Submodules
//! will handle the more specific details of each peripheral.

pub mod rcc;
pub mod gpio;
pub mod systick;

use volatile::Volatile;

pub trait Control {
  unsafe fn mem_addr(&self) -> Volatile<u32>;
}

pub trait Register {
  fn new(base_addr: *const u32) -> Self;

  fn base_addr(&self) -> *const u32;
  // FIXME: Return isize...????
  fn mem_offset(&self) -> u32;
  unsafe fn addr(&self) -> Volatile<u32> {
    Volatile::new(self.base_addr().offset(self.mem_offset() as isize))
  }
}

pub trait Field {
  fn mask(&self) -> u32;
}

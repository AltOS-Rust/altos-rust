// peripheral/gpio/bsrr.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::Register;

#[derive(Copy, Clone)]
pub struct BSRR {
  base_addr: u32,
}

impl Register for BSRR {
  fn new(base_addr: u32) -> Self {
    BSRR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x18
  }
}

impl BSRR {
  /// Set the bit high for the specified port, port must be a value between [0..15] or the kernel
  /// will panic.
  pub fn set(&self, port: u8) {
    if port > 15 {
      panic!("BSRR::set - specified port must be between [0..15]!");
    }

    unsafe {
      let mut reg = self.addr();

      *reg |= 0b1 << port;
    }
  }

  pub fn reset(&self, port: u8) {
    if port > 15 {
      panic!("BSRR::reset - specified port must be between [0..15]!");
    }

    unsafe {
      let mut reg = self.addr();

      *reg |= 0b1 << (port + 16);
    }
  }
}

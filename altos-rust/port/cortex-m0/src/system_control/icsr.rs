// system_control/icsr.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use ::peripheral::Register;

#[derive(Copy, Clone)]
pub struct ICSR {
  base_addr: u32,
}

impl Register for ICSR {
  fn new(base_addr: u32) -> Self {
    ICSR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x04
  }
}

impl ICSR {
  pub fn set_pend_sv(&self) {
    const PEND_SV_SET: u32 = 0b1 << 28;
    unsafe {
      let mut reg = self.addr();
      *reg |= PEND_SV_SET;
    }
  }

  pub fn clear_pend_sv(&self) {
    const PEND_SV_CLEAR: u32 = 0b1 << 27;
    unsafe {
      let mut reg = self.addr();
      *reg |= PEND_SV_CLEAR;
    }
  }
}

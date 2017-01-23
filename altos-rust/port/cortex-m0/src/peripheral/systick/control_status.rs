// peripheral/systick/control_status.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::Register;

pub enum ClockSource {
  Reference,
  Processor,
}

/// The control and status register for the SysTick timer
#[derive(Copy, Clone)]
pub struct CSR {
  base_addr: u32,
}

impl Register for CSR {
  fn new(base_addr: u32) -> Self {
    CSR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x0
  }
}

impl CSR {
  pub fn set_enable(&self, enable: bool) {
    let mask = 0b1 << 0;

    unsafe {
      let mut reg = self.addr();
      if enable {
        *reg |= mask;
      }
      else {
        *reg &= !mask;
      }
    }
  }

  pub fn set_interrupt(&self, enable: bool) {
    let mask = 0b1 << 1;

    unsafe {
      let mut reg = self.addr();
      if enable {
        *reg |= mask;
      }
      else {
        *reg &= !mask;
      }
    }
  }

  pub fn set_source(&self, source: ClockSource) {
    let mask = 0b1 << 2;

    unsafe {
      let mut reg = self.addr();
      match source {
        ClockSource::Reference => *reg &= !mask,
        ClockSource::Processor => *reg |= mask,
      };
    }
  }

  /// Returns true if the counter has reached zero since the last time it was checked.
  pub fn did_underflow(&self) -> bool {
    let mask = 0b1 << 16;

    unsafe {
      let reg = self.addr();
      *reg & mask != 0
    }
  }
}

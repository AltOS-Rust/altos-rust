// interrupt/pending.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use peripheral::Register;

#[derive(Copy, Clone)]
pub struct PendingControl {
  ispr: ISPR,
  icpr: ICPR,
}

impl PendingControl {
  pub fn new(base_addr: u32) -> Self {
    PendingControl {
      ispr: ISPR::new(base_addr),
      icpr: ICPR::new(base_addr),
    }
  }

  pub fn set_pending(&self, interrupt: u8) {
    self.ispr.set_pending(interrupt);
  }

  pub fn clear_pending(&self, interrupt: u8) {
    self.icpr.clear_pending(interrupt);
  }

  pub fn interrupt_is_pending(&self, interrupt: u8) -> bool {
    self.ispr.interrupt_is_pending(interrupt)
  }
}

#[derive(Copy, Clone)]
struct ISPR {
  base_addr: u32,
}

impl Register for ISPR {
  fn new(base_addr: u32) -> Self {
    ISPR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }
  
  fn mem_offset(&self) -> u32 {
    0x100
  }
}

impl ISPR {
  fn set_pending(&self, interrupt: u8) {
    if interrupt > 31 {
      panic!("ISPR::set_pending - specified interrupt must be between [0..31]!");
    }

    unsafe {
      let mut reg = self.addr();
      *reg |= 0b1 << interrupt;
    }
  }

  fn interrupt_is_pending(&self, interrupt: u8) -> bool {
    if interrupt > 31 {
      panic!("ISPR::interrupt_is_pending - specified interrupt must be between [0..31]!");
    }

    unsafe {
      let reg = self.addr();
      (*reg & (0b1 << interrupt)) != 0
    }
  }

}

#[derive(Copy, Clone)]
struct ICPR {
  base_addr: u32,
}

impl Register for ICPR {
  fn new(base_addr: u32) -> Self {
    ICPR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x180
  }
}

impl ICPR {
  fn clear_pending(&self, interrupt: u8) {
    if interrupt > 31 {
      panic!("ICPR::clear_pending - specified interrupt must be between [0..31]!");
    }

    unsafe {
      let mut reg = self.addr();
      *reg |= 0b1 << interrupt;
    }
  }
}

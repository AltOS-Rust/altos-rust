// interrupt/enable.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use peripheral::Register;

#[derive(Copy, Clone)]
pub struct EnableControl {
  iser: ISER,
  icer: ICER,
}

impl EnableControl {
  pub fn new(base_addr: u32) -> Self {
    EnableControl {
      iser: ISER::new(base_addr),
      icer: ICER::new(base_addr),
    }
  }

  pub fn enable_interrupt(&self, interrupt: u8) {
    self.iser.enable_interrupt(interrupt);
  }

  pub fn disable_interrupt(&self, interrupt: u8) {
    self.icer.disable_interrupt(interrupt);
  }

  pub fn interrupt_is_enabled(&self, interrupt: u8) -> bool {
    self.iser.interrupt_is_enabled(interrupt)
  }
}

#[derive(Copy, Clone)]
struct ISER {
  base_addr: u32,
}

impl Register for ISER {
  fn new(base_addr: u32) -> Self {
    ISER { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x0
  }
}

impl ISER {
  fn enable_interrupt(&self, interrupt: u8) {
    if interrupt > 31 {
      panic!("ISER::enable_interrupt - specified interrupt must be between [0..31]!");
    }

    unsafe {
      let mut reg = self.addr();
      *reg |= 0b1 << interrupt;
    }
  }

  fn interrupt_is_enabled(&self, interrupt: u8) -> bool {
    if interrupt > 31 {
      panic!("ISER::get_interrupt - specified interrupt must be between [0..31]!");
    }

    unsafe {
      let reg = self.addr();
      (*reg & (0b1 << interrupt)) != 0
    }
  }
}

#[derive(Copy, Clone)]
struct ICER {
  base_addr: u32,
}

impl Register for ICER {
  fn new(base_addr: u32) -> Self {
    ICER { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x80
  }
}

impl ICER {
  fn disable_interrupt(&self, interrupt: u8) {
    if interrupt > 31 {
      panic!("ISER::disable_interrupt - specified interupt must be between [0..31]!");
    }

    unsafe {
      let mut reg = self.addr();
      *reg |= 0b1 << interrupt;
    }
  }
}


// interrupt/priority.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use peripheral::{Register, Field};

#[derive(Copy, Clone)]
pub enum Priority {
  Highest,
  High,
  Low,
  Lowest,
}

impl Field for Priority {
  fn mask(&self) -> u32 {
    match *self {
      Priority::Highest => 0b00 << 6,
      Priority::High => 0b01 << 6,
      Priority::Low => 0b10 << 6,
      Priority::Lowest => 0b11 << 6,
    }
  }
}

impl Priority {
  fn from_mask(mask: u32) -> Self {
    match mask >> 6 {
      0b00 => Priority::Highest,
      0b01 => Priority::High,
      0b10 => Priority::Low,
      0b11 => Priority::Lowest,
      _ => panic!("Priority::from_mask - mask was not a valid value!"),
    }
  }
}

#[derive(Copy, Clone)]
pub struct PriorityControl {
  ipr_registers: [IPR; 8],
}

impl PriorityControl {
  pub fn new(base_addr: u32) -> Self {
    PriorityControl {
      ipr_registers: [
        IPR::new(base_addr, 0x00),
        IPR::new(base_addr, 0x04),
        IPR::new(base_addr, 0x08),
        IPR::new(base_addr, 0x0C),
        IPR::new(base_addr, 0x10),
        IPR::new(base_addr, 0x14),
        IPR::new(base_addr, 0x18),
        IPR::new(base_addr, 0x1C)],
    }
  }

  pub fn set_priority(&self, priority: Priority, interrupt: u8) {
    if interrupt > 31 {
      panic!("PriorityControl::set_priority - specified interrupt must be between [0..31]!");
    }

    let ipr = self.ipr_registers[(interrupt / 4) as usize];
    ipr.set_priority(priority, interrupt % 4);
  }

  pub fn get_priority(&self, interrupt: u8) -> Priority {
    if interrupt > 31 {
      panic!("PriorityControl::get_priority - specified interrupt must be between [0..31]!");
    }

    let ipr = self.ipr_registers[(interrupt / 4) as usize];
    ipr.get_priority(interrupt % 4)
  }
}

#[derive(Copy, Clone)]
struct IPR {
  base_addr: u32,
  mem_offset: u32,
}

impl Register for IPR {
  fn new(_base_addr: u32) -> Self {
    unimplemented!();
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    self.mem_offset
  }
}

impl IPR {
  fn new(base_addr: u32, offset: u32) -> Self {
    IPR {
      base_addr: base_addr,
      mem_offset: offset,
    }
  }

  fn set_priority(&self, priority: Priority, interrupt: u8) {
    if interrupt > 3 {
      panic!("IPR::set_priority - IPR register only contains up to 4 interrupt priorities!");
    }
    
    let mask = priority.mask();
    unsafe {
      let mut reg = self.addr();

      // Clear top bits first
      *reg &= !((0b11 << 6) << (interrupt * 8));
      *reg |= mask << (interrupt * 8);
    }
  }

  fn get_priority(&self, interrupt: u8) -> Priority {
    if interrupt > 3 {
      panic!("IPR::get_priority - IPR register only contains up to 4 interrupt priorities!");
    }

    let mask = unsafe {
      let reg = self.addr();

      (*reg & (0b11 << 6) << (interrupt * 8)) >> (interrupt * 8)
    };
    Priority::from_mask(mask)
  }
}

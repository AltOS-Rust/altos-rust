// peripheral/gpio/pupdr.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::{Register, Field};

#[derive(Copy, Clone)]
pub enum Pull {
  Neither,
  Up,
  Down,
}

impl Field for Pull {
  fn mask(&self) -> u32 {
    match *self {
      Pull::Neither => 0b00,
      Pull::Up => 0b01,
      Pull::Down => 0b10,
    }
  }
}

impl Pull {
  fn from_mask(mask: u32) -> Self {
    match mask {
      0b00 => Pull::Neither,
      0b01 => Pull::Up,
      0b10 => Pull::Down,
      _ => panic!("Pull::from_mask - mask was an invalid value!"),
    }
  }
}

#[derive(Copy, Clone)]
pub struct PUPDR {
  base_addr: u32,
}

impl Register for PUPDR {
  fn new(base_addr: u32) -> Self {
    PUPDR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x0C
  }
}

impl PUPDR {
  pub fn set_pull(&self, pull: Pull, port: u8) {
    if port > 15 {
      panic!("PUPDR::set_pull - specified port must be between [0..15]!");
    }
    let mask = pull.mask();

    unsafe {
      let mut reg = self.addr();

      *reg &= !(0b11 << (port * 2));
      *reg |= mask << (port * 2);
    }
  }

  pub fn get_pull(&self, port: u8) -> Pull {
    if port > 15 {
      panic!("PUPDR::get_pull - specified port must be between [0..15]!");
    }

    let mask = unsafe {
      let reg = self.addr();

      (*reg & (0b11 << (port * 2))) >> (port * 2)
    };
    Pull::from_mask(mask)
  }
}

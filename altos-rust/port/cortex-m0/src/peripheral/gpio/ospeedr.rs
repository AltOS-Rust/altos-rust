// peripheral/gpio/ospeedr.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::{Register, Field};

#[derive(Copy, Clone)]
pub enum Speed {
  Low,
  Medium,
  High,
}

impl Field for Speed {
  fn mask(&self) -> u32 {
    match *self {
      Speed::Low => 0b00,
      Speed::Medium => 0b01,
      Speed::High => 0b11,
    }
  }
}

impl Speed {
  fn from_mask(mask: u32) -> Self {
    match mask {
      0b00 | 0b10 => Speed::Low,
      0b01 => Speed::Medium,
      0b11 => Speed::High,
      _ => panic!("Speed::from_mask - mask was not a valid value!"),
    }
  }
}

#[derive(Copy, Clone)]
pub struct OSPEEDR {
  base_addr: u32,
}

impl Register for OSPEEDR {
  fn new(base_addr: u32) -> Self {
    OSPEEDR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x08
  }
}

impl OSPEEDR {
  pub fn set_speed(&self, speed: Speed, port: u8) {
    if port > 15 {
      panic!("OSPEEDR::set_speed - specified port must be between [0..15]!");
    }
    let mask = speed.mask();

    unsafe {
      let mut reg = self.addr();

      *reg &= !(0b11 << (port * 2));
      *reg |= mask << (port * 2);
    }
  }

  pub fn get_speed(&self, port: u8) -> Speed {
    if port > 15 {
      panic!("OSPEEDR::get_speed - specified port must be between [0..15]!");
    }
    
    let mask = unsafe {
      let reg = self.addr();

      (*reg & (0b11 << (port * 2))) >> (port * 2)
    };
    Speed::from_mask(mask)
  }
}

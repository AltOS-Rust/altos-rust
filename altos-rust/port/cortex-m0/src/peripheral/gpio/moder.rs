// peripheral/gpio/moder.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::{Register, Field};

#[derive(Copy, Clone)]
pub enum Mode {
  Input,
  Output,
  Alternate,
  Analog,
}

impl Field for Mode {
  fn mask(&self) -> u32 {
    match *self {
      Mode::Input => 0b00,
      Mode::Output => 0b01,
      Mode::Alternate => 0b10,
      Mode::Analog => 0b11,
    }
  }
}

impl Mode {
  fn from_mask(mask: u32) -> Self {
    match mask {
      0b00 => Mode::Input,
      0b01 => Mode::Output,
      0b10 => Mode::Alternate,
      0b11 => Mode::Analog,
      _ => panic!("Mode::from_mask - mask was not a valid value!"),
    }
  }
}

#[derive(Copy, Clone)]
pub struct MODER {
  base_addr: u32,
}

impl Register for MODER {
  fn new(base_addr: u32) -> Self {
    MODER { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x0
  }
}

impl MODER {
  /// Set the mode for the specified port, port must be a value between [0..15] or the kernel will
  /// panic
  pub fn set_mode(&self, mode: Mode, port: u8) {
    if port > 15 {
      panic!("MODER::set_mode - specified port must be a value between [0..15]!");
    }
    let mask = mode.mask();

    unsafe {
      let mut reg = self.addr();
      
      // Zero the field first
      *reg &= !(0b11 << (port * 2));
      *reg |= mask << (port * 2);
    }
  }
  
  /// Get the current mode for the specified port, port must be a value between [0..15] or the kernel
  /// will panic.
  pub fn get_mode(&self, port: u8) -> Mode {
    if port > 15 {
      panic!("MODER::get_mode - specified port must be a value between [0..15]!");
    }
    
    let mask = unsafe {
      let reg = self.addr();

      (*reg & (0b11 << (port * 2))) >> (port * 2)
    };
    Mode::from_mask(mask)
  }
}

/*
 * Copyright (C) 2017 AltOS-Rust Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

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
  base_addr: *const u32,
}

impl Register for MODER {
  fn new(base_addr: *const u32) -> Self {
    MODER { base_addr: base_addr }
  }

  fn base_addr(&self) -> *const u32 {
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

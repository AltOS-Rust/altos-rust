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
  base_addr: *const u32,
}

impl Register for PUPDR {
  fn new(base_addr: *const u32) -> Self {
    PUPDR { base_addr: base_addr }
  }

  fn base_addr(&self) -> *const u32 {
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

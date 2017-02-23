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

// peripheral/gpio/bsrr.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::Register;

#[derive(Copy, Clone)]
pub struct BSRR {
  base_addr: *const u32,
}

impl Register for BSRR {
  fn new(base_addr: *const u32) -> Self {
    BSRR { base_addr: base_addr }
  }

  fn base_addr(&self) -> *const u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x18
  }
}

impl BSRR {
  /// Set the bit high for the specified port, port must be a value between [0..15] or the kernel
  /// will panic.
  pub fn set(&self, port: u8) {
    if port > 15 {
      panic!("BSRR::set - specified port must be between [0..15]!");
    }

    unsafe {
      let mut reg = self.addr();

      *reg |= 0b1 << port;
    }
  }

  pub fn reset(&self, port: u8) {
    if port > 15 {
      panic!("BSRR::reset - specified port must be between [0..15]!");
    }

    unsafe {
      let mut reg = self.addr();

      *reg |= 0b1 << (port + 16);
    }
  }
}

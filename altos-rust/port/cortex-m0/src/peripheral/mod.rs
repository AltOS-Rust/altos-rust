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

// peripheral/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! This module handles the memory mapped peripherals that are a part of the Cortex-M0. Submodules
//! will handle the more specific details of each peripheral.

pub mod rcc;
pub mod gpio;
pub mod systick;
#[cfg(feature="serial")]
pub mod usart;

use volatile::Volatile;

pub trait Control {
  unsafe fn mem_addr(&self) -> Volatile<u32>;
}

pub trait Register {
  fn new(base_addr: *const u32) -> Self;

  fn base_addr(&self) -> *const u32;
  // FIXME: Return isize...????
  fn mem_offset(&self) -> u32;
  unsafe fn addr(&self) -> Volatile<u32> {
    // We cast to a u8 so the pointer offset is not multiplied
    let addr = self.base_addr() as *const u8;
    Volatile::new(addr.offset(self.mem_offset() as isize) as *const u32)
  }
}

pub trait Field {
  fn mask(&self) -> u32;
}

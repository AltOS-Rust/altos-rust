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

// system_control/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use ::volatile::Volatile;
use ::peripheral::{Control, Register};

mod icsr;

pub fn scb() -> SCB {
  SCB::scb()
}

/// System Control Block
#[derive(Copy, Clone)]
pub struct SCB {
  mem_addr: *const u32,
  icsr: icsr::ICSR,
}

impl Control for SCB {
  unsafe fn mem_addr(&self) -> Volatile<u32> {
    Volatile::new(self.mem_addr)
  }
}

impl SCB {
  fn scb() -> Self {
    const SCB_ADDR: *const u32 = 0xE000_ED00 as *const _;
    SCB {
      mem_addr: SCB_ADDR,
      icsr: icsr::ICSR::new(SCB_ADDR),
    }
  }

  pub fn set_pend_sv(&self) {
    self.icsr.set_pend_sv();
  }

  pub fn clear_pend_sv(&self) {
    self.icsr.clear_pend_sv();
  }
}

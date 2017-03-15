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

//! This module provides system implementation information and allows
//! configuration control and reporting of system exceptions.

mod icsr;
mod defs;

use ::volatile::Volatile;
use ::peripheral::{Control, Register};
use self::defs::*;

/// Returns instance of the System Control Block.
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
        SCB {
            mem_addr: SCB_ADDR,
            icsr: icsr::ICSR::new(SCB_ADDR),
        }
    }

    /// Trigger a pend_sv exception.
    ///
    /// PendSV signals to the operating system that a context
    /// switch should occur.
    pub fn set_pend_sv(&mut self) {
        self.icsr.set_pend_sv();
    }

    /// Clear the pend_sv exception.
    pub fn clear_pend_sv(&mut self) {
        self.icsr.clear_pend_sv();
    }
}

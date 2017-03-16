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

use super::super::Register;
use super::defs::*;

pub enum ClockSource {
    Reference,
    Processor,
}

/// The control and status register for the SysTick timer
#[derive(Copy, Clone)]
pub struct CSR {
    base_addr: *const u32,
}

impl Register for CSR {
    fn new(base_addr: *const u32) -> Self {
        CSR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CSR_OFFSET
    }
}

impl CSR {
    pub fn set_enable(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            if enable {
                *reg |= ENABLE;
            }
            else {
                *reg &= !ENABLE;
            }
        }
    }

    pub fn set_interrupt(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            if enable {
                *reg |= TICKINT;
            }
            else {
                *reg &= !TICKINT;
            }
        }
    }

    pub fn set_source(&mut self, source: ClockSource) {
        unsafe {
            let mut reg = self.addr();
            match source {
                ClockSource::Reference => *reg &= !CLKSOURCE,
                ClockSource::Processor => *reg |= CLKSOURCE,
            };
        }
    }

    /// Returns true if the counter has reached zero since the last time it was checked.
    pub fn did_underflow(&self) -> bool {
        unsafe {
            let reg = self.addr();
            *reg & COUNTFLAG != 0
        }
    }
}

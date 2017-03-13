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

#[derive(Copy, Clone)]
pub struct CVR {
    base_addr: *const u32,
}

impl Register for CVR {
    fn new(base_addr: *const u32) -> Self {
        CVR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CVR_OFFSET
    }
}

impl CVR {
    pub fn get_current_value(&self) -> u32 {
        unsafe {
            let reg = self.addr();
            *reg & CURRENT
        }
    }

    pub fn clear_current_value(&mut self) {
        // A write to this register clears its value to 0
        unsafe {
            let mut reg = self.addr();
            reg.store(1);
        }
    }
}

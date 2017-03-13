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

use peripheral::Register;
use interrupt::defs::Hardware;

#[derive(Copy, Clone)]
pub struct PendingControl {
    ispr: ISPR,
    icpr: ICPR,
}

impl PendingControl {
    pub fn new(base_addr: *const u32) -> Self {
        PendingControl {
            ispr: ISPR::new(base_addr),
            icpr: ICPR::new(base_addr),
        }
    }

    pub fn set_pending(&mut self, hardware: Hardware) {
        self.ispr.set_pending(hardware);
    }

    pub fn clear_pending(&mut self, hardware: Hardware) {
        self.icpr.clear_pending(hardware);
    }

    pub fn interrupt_is_pending(&self, hardware: Hardware) -> bool {
        self.ispr.interrupt_is_pending(hardware)
    }
}

#[derive(Copy, Clone)]
struct ISPR {
    base_addr: *const u32,
}

impl Register for ISPR {
    fn new(base_addr: *const u32) -> Self {
        ISPR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x100
    }
}

impl ISPR {
    fn set_pending(&mut self, hardware: Hardware) {
        let interrupt = hardware as u8;
        unsafe {
            let mut reg = self.addr();
            *reg |= 0b1 << interrupt;
        }
    }

    fn interrupt_is_pending(&self, hardware: Hardware) -> bool {
        let interrupt = hardware as u8;
        unsafe {
            let reg = self.addr();
            (*reg & (0b1 << interrupt)) != 0
        }
    }
}

#[derive(Copy, Clone)]
struct ICPR {
    base_addr: *const u32,
}

impl Register for ICPR {
    fn new(base_addr: *const u32) -> Self {
        ICPR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x180
    }
}

impl ICPR {
    fn clear_pending(&mut self, hardware: Hardware) {
        let interrupt = hardware as u8;
        unsafe {
            let mut reg = self.addr();
            *reg |= 0b1 << interrupt;
        }
    }
}

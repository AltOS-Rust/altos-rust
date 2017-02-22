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
pub struct EnableControl {
    iser: ISER,
    icer: ICER,
}

impl EnableControl {
    pub fn new(base_addr: *const u32) -> Self {
        EnableControl {
            iser: ISER::new(base_addr),
            icer: ICER::new(base_addr),
        }
    }

    pub fn enable_interrupt(&self, hardware: Hardware) {
            self.iser.enable_interrupt(hardware);
    }

    pub fn disable_interrupt(&self, hardware: Hardware) {
        self.icer.disable_interrupt(hardware);
    }

    pub fn interrupt_is_enabled(&self, hardware: Hardware) -> bool {
        self.iser.interrupt_is_enabled(hardware)
    }
}

#[derive(Copy, Clone)]
struct ISER {
    base_addr: *const u32,
}

impl Register for ISER {
    fn new(base_addr: *const u32) -> Self {
        ISER { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x0
    }
}

impl ISER {
    fn enable_interrupt(&self, hardware: Hardware) {
        let interrupt = hardware as u8;

        unsafe {
            let mut reg = self.addr();
            *reg |= 0b1 << interrupt;
        }
    }

    fn interrupt_is_enabled(&self, hardware: Hardware) -> bool {

        let interrupt = hardware as u8;
        unsafe {
            let reg = self.addr();
            (*reg & (0b1 << interrupt)) != 0
        }
    }
}

#[derive(Copy, Clone)]
struct ICER {
    base_addr: *const u32,
}

impl Register for ICER {
    fn new(base_addr: *const u32) -> Self {
        ICER { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x80
    }
}

impl ICER {
    fn disable_interrupt(&self, hardware: Hardware) {
        let interrupt = hardware as u8;
        unsafe {
            let mut reg = self.addr();
            *reg |= 0b1 << interrupt;
        }
    }
}


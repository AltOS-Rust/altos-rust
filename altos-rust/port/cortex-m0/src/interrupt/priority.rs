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

use peripheral::{Register, Field};
use interrupt::defs::Hardware;

/// The priority of the interrupt.
///
/// If in the interrupt handler and another interrupt with a
/// higher priority is generated, the CPU will handle the higher
/// priority interrupt before it finishes handling the lower priority interrupt.
#[derive(Copy, Clone)]
pub enum Priority {
    Highest,
    High,
    Low,
    Lowest,
}

impl Field for Priority {
    fn mask(&self) -> u32 {
        match *self {
            Priority::Highest => 0b00 << 6,
            Priority::High => 0b01 << 6,
            Priority::Low => 0b10 << 6,
            Priority::Lowest => 0b11 << 6,
        }
    }
}

impl Priority {
    fn from_mask(mask: u32) -> Self {
        match mask >> 6 {
            0b00 => Priority::Highest,
            0b01 => Priority::High,
            0b10 => Priority::Low,
            0b11 => Priority::Lowest,
            _ => panic!("Priority::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct PriorityControl {
    ipr_registers: [IPR; 8],
}

impl PriorityControl {
    pub fn new(base_addr: *const u32) -> Self {
        PriorityControl {
            ipr_registers: [
            IPR::new(base_addr, 0x00),
            IPR::new(base_addr, 0x04),
            IPR::new(base_addr, 0x08),
            IPR::new(base_addr, 0x0C),
            IPR::new(base_addr, 0x10),
            IPR::new(base_addr, 0x14),
            IPR::new(base_addr, 0x18),
            IPR::new(base_addr, 0x1C)],
        }
    }

    pub fn set_priority(&mut self, priority: Priority, hardware: Hardware) {
        let interrupt = hardware as u8;
        let mut ipr = self.ipr_registers[(interrupt / 4) as usize];
        ipr.set_priority(priority, interrupt % 4);
    }

    pub fn get_priority(&self, hardware: Hardware) -> Priority {
        let interrupt = hardware as u8;
        let ipr = self.ipr_registers[(interrupt / 4) as usize];
        ipr.get_priority(interrupt % 4)
    }
}

#[derive(Copy, Clone)]
struct IPR {
    base_addr: *const u32,
    mem_offset: u32,
}

impl Register for IPR {
    fn new(_base_addr: *const u32) -> Self {
        unimplemented!();
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        self.mem_offset
    }
}

impl IPR {
    fn new(base_addr: *const u32, offset: u32) -> Self {
        IPR {
            base_addr: base_addr,
            mem_offset: offset,
        }
    }

    fn set_priority(&mut self, priority: Priority, interrupt: u8) {

        let mask = priority.mask();
        unsafe {
            let mut reg = self.addr();

            // Clear top bits first
            *reg &= !((0b11 << 6) << (interrupt * 8));
            *reg |= mask << (interrupt * 8);
        }
    }

    fn get_priority(&self, interrupt:u8) -> Priority {
        let mask = unsafe {
            let reg = self.addr();

            (*reg & (0b11 << 6) << (interrupt * 8)) >> (interrupt * 8)
        };
        Priority::from_mask(mask)
    }
}

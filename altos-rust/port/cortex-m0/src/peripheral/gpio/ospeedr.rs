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

/// Defines available GPIO speeds.
#[derive(Copy, Clone)]
pub enum Speed {
    Low,
    Medium,
    High,
}

impl Field for Speed {
    fn mask(&self) -> u32 {
        match *self {
            Speed::Low => 0b00,
            Speed::Medium => 0b01,
            Speed::High => 0b11,
        }
    }
}

impl Speed {
    fn from_mask(mask: u32) -> Self {
        match mask {
            0b00 | 0b10 => Speed::Low,
            0b01 => Speed::Medium,
            0b11 => Speed::High,
            _ => panic!("Speed::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct OSPEEDR {
    base_addr: *const u32,
}

impl Register for OSPEEDR {
    fn new(base_addr: *const u32) -> Self {
        OSPEEDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        0x08
    }
}

impl OSPEEDR {
    pub fn set_speed(&mut self, speed: Speed, port: u8) {
        if port > 15 {
            panic!("OSPEEDR::set_speed - specified port must be between [0..15]!");
        }
        let mask = speed.mask();

        unsafe {
            let mut reg = self.addr();
            *reg &= !(0b11 << (port * 2));
            *reg |= mask << (port * 2);
        }
    }

    pub fn get_speed(&self, port: u8) -> Speed {
        if port > 15 {
            panic!("OSPEEDR::get_speed - specified port must be between [0..15]!");
        }

        let mask = unsafe {
            let reg = self.addr();
            (*reg & (0b11 << (port * 2))) >> (port * 2)
        };
        Speed::from_mask(mask)
    }
}

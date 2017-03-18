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
use super::defs::*;

/// Defines the behavior of the GPIO pin when not asserted.
#[derive(Copy, Clone)]
pub enum Pull {
    /// No behavior.
    Neither,
    /// Pull toward high voltage.
    Up,
    /// Pull toward low voltage.
    Down,
}

impl Field for Pull {
    fn mask(&self) -> u32 {
        match *self {
            Pull::Neither => PUPD_NEITHER,
            Pull::Up => PUPD_UP,
            Pull::Down => PUPD_DOWN,
        }
    }
}

impl Pull {
    fn from_mask(mask: u32) -> Self {
        match mask {
            PUPD_NEITHER => Pull::Neither,
            PUPD_UP => Pull::Up,
            PUPD_DOWN => Pull::Down,
            _ => panic!("Pull::from_mask - mask was an invalid value!"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct PUPDR {
    base_addr: *const u32,
}

impl Register for PUPDR {
    fn new(base_addr: *const u32) -> Self {
        PUPDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        PUPDR_OFFSET
    }
}

impl PUPDR {
    pub fn set_pull(&mut self, pull: Pull, port: u8) {
        if port > 15 {
            panic!("PUPDR::set_pull - specified port must be between [0..15]!");
        }
        let mask = pull.mask();

        unsafe {
            let mut reg = self.addr();
            *reg &= !(PUPD_MASK << (port * 2));
            *reg |= mask << (port * 2);
        }
    }

    pub fn get_pull(&self, port: u8) -> Pull {
        if port > 15 {
            panic!("PUPDR::get_pull - specified port must be between [0..15]!");
        }

        let mask = unsafe {
            let reg = self.addr();
            (*reg & (PUPD_MASK << (port * 2))) >> (port * 2)
        };
        Pull::from_mask(mask)
    }
}

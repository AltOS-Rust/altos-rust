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

/// Available GPIO pin types.
#[derive(Copy, Clone)]
pub enum Type {
    /// Actively drives the output to High.
    PushPull,
    /// Passively drives the output to High by an internal or external pull-up resistor.
    OpenDrain,
}

impl Field for Type {
    fn mask(&self) -> u32 {
        match *self {
            Type::PushPull => TYPE_PUSHPULL,
            Type::OpenDrain => TYPE_OPENDRAIN,
        }
    }
}

impl Type {
    fn from_mask(mask: u32) -> Self {
        match mask {
            TYPE_PUSHPULL => Type::PushPull,
            TYPE_OPENDRAIN => Type::OpenDrain,
            _ => panic!("Type::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct OTYPER {
    base_addr: *const u32,
}

impl Register for OTYPER {
    fn new(base_addr: *const u32) -> Self {
        OTYPER { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        OTYPER_OFFSET
    }
}

impl OTYPER {
    pub fn set_type(&mut self, new_type: Type, port: u8) {
        if port > 15 {
            panic!("OTYPER::set_type - specified port must be between [0..15]!");
        }

        unsafe {
            let mut reg = self.addr();
            match new_type {
                Type::PushPull => *reg &= !(0b1 << port),
                Type::OpenDrain => *reg |= 0b1 << port,
            }
        }
    }

    pub fn get_type(&self, port: u8) -> Type {
        if port > 15 {
            panic!("OTYPER::get_type - specified port must be between [0..15]!");
        }

        let mask = unsafe {
            let reg = self.addr();

            (*reg & (0b1 << port)) >> port
        };
        Type::from_mask(mask)
    }
}

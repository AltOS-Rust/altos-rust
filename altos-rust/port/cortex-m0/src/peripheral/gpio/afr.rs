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

/// Set the functionality of a port.
///
/// See data sheet for port mappings.
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub enum AlternateFunction {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl Field for AlternateFunction {
    fn mask(&self) -> u32 {
        match *self {
            AlternateFunction::Zero => AF0,
            AlternateFunction::One => AF1,
            AlternateFunction::Two => AF2,
            AlternateFunction::Three => AF3,
            AlternateFunction::Four => AF4,
            AlternateFunction::Five => AF5,
            AlternateFunction::Six => AF6,
            AlternateFunction::Seven => AF7,
        }
    }
}

impl AlternateFunction {
    fn from_mask(mask: u32) -> Self {
        match mask {
            AF0 => AlternateFunction::Zero,
            AF1 => AlternateFunction::One,
            AF2 => AlternateFunction::Two,
            AF3 => AlternateFunction::Three,
            AF4 => AlternateFunction::Four,
            AF5 => AlternateFunction::Five,
            AF6 => AlternateFunction::Six,
            AF7 => AlternateFunction::Seven,
            _ => panic!("AlternateFunction::from_mask - mask was not a valid value!"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct AlternateFunctionControl {
    afrl: AFRL,
    afrh: AFRH,
}

impl AlternateFunctionControl {
    pub fn new(base_addr: *const u32) -> Self {
        AlternateFunctionControl {
            afrl: AFRL::new(base_addr),
            afrh: AFRH::new(base_addr),
        }
    }

    pub fn set_function(&mut self, function: AlternateFunction, port: u8) {
        if port < 8 {
            self.afrl.set_function(function, port);
        }
        else {
            self.afrh.set_function(function, port);
        }
    }

    pub fn get_function(&self, port: u8) -> AlternateFunction {
        if port < 8 {
            self.afrl.get_function(port)
        }
        else {
            self.afrh.get_function(port)
        }
    }
}

#[derive(Copy, Clone)]
struct AFRL {
    base_addr: *const u32,
}

impl Register for AFRL {
    fn new(base_addr: *const u32) -> Self {
        AFRL { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        AFRL_OFFSET
    }
}

impl AFRL {
    fn set_function(&mut self, function: AlternateFunction, port: u8) {
        if port > 8 {
            panic!("AFRL::set_function - specified port must be between [0..7]!");
        }
        let mask = function.mask();

        unsafe {
            let mut reg = self.addr();
            *reg &= !(AFR_MASK << (port * 4));
            *reg |= mask << (port * 4);
        }
    }

    fn get_function(&self, port: u8) -> AlternateFunction {
        if port > 8 {
            panic!("AFRL::get_function - specified port must be between [0..7]!");
        }

        let mask = unsafe {
            let reg = self.addr();

            *reg & (AFR_MASK << (port * 4))
        };
        AlternateFunction::from_mask(mask)
    }
}

#[derive(Copy, Clone)]
struct AFRH {
    base_addr: *const u32,
}

impl Register for AFRH {
    fn new(base_addr: *const u32) -> Self {
        AFRH { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        AFRH_OFFSET
    }
}

impl AFRH {
    fn set_function(&mut self, function: AlternateFunction, port: u8) {
        if port > 15 || port < 8 {
            panic!("AFRH::set_function - specified port must be between [8..15]!");
        }
        let mask = function.mask();

        // #9: Port needs to be subtracted by 8 since afr registers are split into high and low
        // for 0-7 and 8-15. i.e. port 9 is actually offset 1 * 4 in the afrh register
        // (rather than offset 9 * 4)
        let port = port - 8;
        unsafe {
            let mut reg = self.addr();

            *reg &= !(AFR_MASK << (port * 4));
            *reg |= mask << (port * 4);
        }
    }

    fn get_function(&self, port: u8) -> AlternateFunction {
        if port > 15 || port < 8 {
            panic!("AFRL::get_function - specified port must be between [8..15]!");
        }

        // #9: See comment in `set_function`
        let port = port - 8;
        let mask = unsafe {
            let reg = self.addr();

            *reg & (AFR_MASK << (port * 4))
        };
        AlternateFunction::from_mask(mask)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_afrh_set_function() {
        let mut afrh = test::create_register::<AFRH>();
        afrh.set_function(AlternateFunction::Five, 8);

        assert_eq!(afrh.register_value(), 0x5);
    }

    #[test]
    #[should_panic]
    fn test_afrh_set_port_out_of_bounds_panics() {
        let mut afrh = test::create_register::<AFRH>();
        afrh.set_function(AlternateFunction::Seven, 2);
    }

    #[test]
    fn test_afrl_set_function() {
        let mut afrl = test::create_register::<AFRL>();
        afrl.set_function(AlternateFunction::Two, 3);

        assert_eq!(afrl.register_value(), 0x2000);
    }

    #[test]
    #[should_panic]
    fn test_afrl_set_port_out_of_bounds_panics() {
        let mut afrl = test::create_register::<AFRL>();
        afrl.set_function(AlternateFunction::Two, 10);
    }
}

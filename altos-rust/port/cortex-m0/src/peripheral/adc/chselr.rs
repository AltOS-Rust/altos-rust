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

use super::super::{Field, Register};
use super::defs::*;

// TODO: Comments
#[derive(Copy, Clone)]
pub enum Channel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
}

impl Field for Channel {
    fn mask(&self) -> u32 {
        match *self {
            Channel::Zero => CHSELR_0,
            Channel::One => CHSELR_1,
            Channel::Two => CHSELR_2,
            Channel::Three => CHSELR_3,
            Channel::Four => CHSELR_4,
            Channel::Five => CHSELR_5,
            Channel::Six => CHSELR_6,
            Channel::Seven => CHSELR_7,
            Channel::Eight => CHSELR_8,
            Channel::Nine => CHSELR_9,
            Channel::Ten => CHSELR_10,
            Channel::Eleven => CHSELR_11,
            Channel::Twelve => CHSELR_12,
            Channel::Thirteen => CHSELR_13,
            Channel::Fourteen => CHSELR_14,
            Channel::Fifteen => CHSELR_15,
            Channel::Sixteen => CHSELR_16,
            Channel::Seventeen => CHSELR_17,
            Channel::Eighteen => CHSELR_18,
        }
    }
}

impl Channel {
    fn from_mask(mask: u32) -> Self {
        match mask {
            CHSELR_0 => Channel::Zero,
            CHSELR_1 => Channel::One,
            CHSELR_2 => Channel::Two,
            CHSELR_3 => Channel::Three,
            CHSELR_4 => Channel::Four,
            CHSELR_5 => Channel::Five,
            CHSELR_6 => Channel::Six,
            CHSELR_7 => Channel::Seven,
            CHSELR_8 => Channel::Eight,
            CHSELR_9 => Channel::Nine,
            CHSELR_10 => Channel::Ten,
            CHSELR_11 => Channel::Eleven,
            CHSELR_12 => Channel::Twelve,
            CHSELR_13 => Channel::Thirteen,
            CHSELR_14 => Channel::Fourteen,
            CHSELR_15 => Channel::Fifteen,
            CHSELR_16 => Channel::Sixteen,
            CHSELR_17 => Channel::Seventeen,
            CHSELR_18 => Channel::Eighteen,
            _ => panic!("Channel::from_mask - mask was not a valid value!"),
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub struct CHSELR {
    base_addr: *const u32,
}

impl Register for CHSELR {
    fn new( base_addr: *const u32 ) -> Self {
        CHSELR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CHSELR_OFFSET
    }
}

impl CHSELR {
    pub fn select_channel(&mut self, channel: Channel) {
        unsafe {
            *self.addr() |= channel.mask();
        }
    }

    pub fn unselect_channel(&mut self, channel: Channel) {
        unsafe {
            *self.addr() &= !channel.mask();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;
}

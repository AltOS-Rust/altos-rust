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
    pub fn select_channel(&mut self, channel: usize) {
        match channel {
            0...18 => {
                let chelsr_bit: u32 = 0b1 << channel;
                unsafe { *self.addr() |= chelsr_bit }
            },
            _ => panic!{"CHSELR::select_channel - invalid channel"},
        }
    }

    pub fn unselect_channel(&mut self, channel: usize) {
        match channel {
            0...18 => {
                let chelsr_bit: u32 = 0b0 << channel;
                unsafe { *self.addr() &= chelsr_bit; }
            },
            _ => panic!{"CHSELSR::select_channel - invalid channel"},
        }
    }

    pub fn select_multiple_channels(&mut self, channel_array: [bool; 19]) {
        let chelsr_bit: u32 = 0b1;
        for channel in channel_array.iter() {
            if *channel {
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;
}

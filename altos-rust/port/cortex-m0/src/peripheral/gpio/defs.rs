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

pub const GROUPA_ADDR: *const u32 = 0x4800_0000 as *const _;
pub const GROUPB_ADDR: *const u32 = 0x4800_0400 as *const _;
pub const GROUPC_ADDR: *const u32 = 0x4800_0800 as *const _;
pub const GROUPF_ADDR: *const u32 = 0x4800_1400 as *const _;

pub const OTYPER_OFFSET: u32 = 0x04;
pub const TYPE_PUSHPULL: u32 = 0b0;
pub const TYPE_OPENDRAIN: u32 = 0b1;

pub const OSPEEDR_OFFSET: u32 = 0x08;
pub const SPEED_MASK: u32 = 0b11;
pub const SPEED_LOW: u32 = 0b00;
pub const SPEED_LOW_ALT: u32 = 0b10;
pub const SPEED_MEDIUM: u32 = 0b01;
pub const SPEED_HIGH: u32 = 0b11;

pub const PUPDR_OFFSET: u32 = 0x0C;
pub const PUPD_MASK: u32 = 0b11;
pub const PUPD_NEITHER: u32 = 0b00;
pub const PUPD_UP: u32 = 0b01;
pub const PUPD_DOWN: u32 = 0b10;


pub const BSRR_OFFSET: u32 = 0x18;
pub const BSRR_RESET_OFFSET: u8 = 16;

pub const AFRL_OFFSET: u32 = 0x20;
pub const AFR_MASK: u32 = 0b1111;
pub const AF0: u32 = 0b0000;
pub const AF1: u32 = 0b0001;
pub const AF2: u32 = 0b0010;
pub const AF3: u32 = 0b0011;
pub const AF4: u32 = 0b0100;
pub const AF5: u32 = 0b0101;
pub const AF6: u32 = 0b0110;
pub const AF7: u32 = 0b0111;

pub const AFRH_OFFSET: u32 = 0x24;


pub const MODER_OFFSET: u32 = 0x00;
pub const MODE_MASK: u32 = 0b11;
pub const MODE_INPUT: u32 = 0b00;
pub const MODE_OUTPUT: u32 = 0b01;
pub const MODE_ALTERNATE: u32 = 0b10;
pub const MODE_ANALOG: u32 = 0b11;

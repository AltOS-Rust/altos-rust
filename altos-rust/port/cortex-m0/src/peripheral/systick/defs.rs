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

pub const SYSTICK_ADDR: *const u32 = 0xE000E010 as *const _;

// Control Status Register
pub const CSR_OFFSET: u32 = 0x00;
pub const ENABLE: u32 = 0b1 << 0;
pub const TICKINT: u32 = 0b1 << 1;
pub const CLKSOURCE: u32 = 0b1 << 2;
pub const COUNTFLAG: u32 = 0b1 << 16;

// Reload Value Register
pub const RVR_OFFSET: u32 = 0x04;
pub const RELOAD: u32 = 0xFFFFFF;

// Current Value Register
pub const CVR_OFFSET: u32 = 0x08;
pub const CURRENT: u32 = 0xFFFFFF;

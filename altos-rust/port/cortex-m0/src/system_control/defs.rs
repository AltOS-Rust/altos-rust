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

pub const SCB_ADDR: *const u32 = 0xE000_ED00 as *const _;

pub const ICSR_OFFSET: u32 = 0x04;
pub const ICSR_PENDSVCLR: u32 = 0b1 << 27;
pub const ICSR_PENDSVSET: u32 = 0b1 << 28;

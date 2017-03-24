/*
* Copyright (C) 2017  AltOS-Rust Team
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

#![no_std]
#![feature(cfg_target_has_atomic)]
#![feature(const_fn)]

#[cfg(all(target_arch="arm", not(target_has_atomic="ptr")))]
extern crate cm0_atomic as atomic;

#[cfg(target_has_atomic="ptr")]
use core::sync::atomic as atomic;

pub mod spin;

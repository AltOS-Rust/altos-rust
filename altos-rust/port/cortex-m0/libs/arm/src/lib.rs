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

#![no_std]
#![feature(asm)]
#![feature(naked_functions)]

// NOTE: A lot of these functions are taken from other sources, one very useful resource is
// https://github.com/rust-lang-nursery/compiler-builtins
// The reason we're not just depending on this library is that
//  1. It uses some extra stuff that we don't neccessarily need, and so it increases the binary
//     size unneccessarily
//  2. It causes some weird interaction with the linker between debug and release modes, so some of
//     the symbols aren't getting exported when compiling for release
//
// The functions there are very useful as a resource however.

pub mod asm;
pub mod math;
pub mod mem;

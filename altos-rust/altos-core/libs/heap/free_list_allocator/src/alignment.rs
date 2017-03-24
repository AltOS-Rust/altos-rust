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

//! This file contains some functions for dealing with alignment for memory allocation.
//!
//! These functions are mainly intended to be used for adjusting the alignment of addresses,
//! but they can also be used to adjust the size of the heap and memory allocations.

// TODO: This file should probably be moved so that it can be utilized by both allocators

/// Returns whichever alignment is larger, BlockHeader's or the requested one.
pub fn use_align(align: usize, block_hdr_align: usize) -> usize {
    if align > block_hdr_align { align } else { block_hdr_align }
}

/// Align downwards. Returns the greatest x with alignment `align` so that x <= addr.
/// The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    }
    else if align == 0 {
        addr
    }
    else {
        panic!("align_down - `align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align` so that x >= addr.
/// The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    if align == 0 {
        addr
    }
    else {
        align_down(addr + align - 1, align)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_align_picks_larger_alignment() {
        assert_eq!(use_align(8, 8), 8);
        assert_eq!(use_align(8, 16), 16);
        assert_eq!(use_align(16, 8), 16);
    }

    #[test]
    fn align_up_zero_alignment() {
        assert_eq!(align_up(8, 0), 8);
        assert_eq!(align_up(12, 0), 12);
    }

    #[test]
    fn align_up_already_correctly_aligned() {
        assert_eq!(align_up(8, 1), 8);
        assert_eq!(align_up(8, 8), 8);
    }

    #[test]
    fn align_up_changes_misaligned_address() {
        assert_eq!(align_up(2, 8), 8);
        assert_eq!(align_up(4, 16), 16);
        assert_eq!(align_up(12, 16), 16);
    }

    #[test]
    fn align_down_zero_alignment() {
        assert_eq!(align_down(8, 0), 8);
        assert_eq!(align_down(11, 0), 11);
    }

    #[test]
    fn align_down_already_correctly_aligned() {
        assert_eq!(align_down(8, 1), 8);
        assert_eq!(align_down(11, 1), 11);
        assert_eq!(align_down(16, 8), 16);
    }

    #[test]
    fn align_down_changes_misaligned_address() {
        assert_eq!(align_down(12, 8), 8);
        assert_eq!(align_down(8, 16), 0);
        assert_eq!(align_down(17, 16), 16);
    }
}

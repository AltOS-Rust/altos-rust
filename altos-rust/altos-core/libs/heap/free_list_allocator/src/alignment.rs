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

/*
 * This file contains some functions for dealing with alignment for memory allocation
 */

// TODO: This file should probably be moved so that it can be utilized by both allocators

// This ensures the block size actually allocated is a multiple of the BlockHeader size.
// Actual allocation size >= requested size (obviously)
pub fn use_size(request_size: usize, block_hdr_size: usize) -> usize {
    align_up(request_size, block_hdr_size)
}

// Bumps block header alignment up to the nearest power of 2. Assumes the passed align is a
// power of 2 already (screening is done in free_list.allocate()).
// Returns whichever alignment is larger, BlockHeader's or the requested one.
pub fn use_align(align: usize, mut block_hdr_align: usize) -> usize {
    // This is a little inelegant maybe we can change it later.
    while !block_hdr_align.is_power_of_two() {
        block_hdr_align += 1;
    }

    if (block_hdr_align % align) == 0 {
        block_hdr_align
    }
    else if (align % block_hdr_align) == 0 {
        align
    } else {
        panic!("use_align - 'cannot align'")
    }
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
    align_down(addr + align - 1, align)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn use_align_returns_common_multiple_of_request_size_and_block_header_size() {
        let block_hdr_align = 8;
        let mut alloc_align = use_align(1, block_hdr_align);

        assert!(alloc_align.is_power_of_two());
        assert!(alloc_align % block_hdr_align == 0);

        alloc_align = use_align(2, block_hdr_align);

        assert!(alloc_align.is_power_of_two());
        assert!(alloc_align % block_hdr_align == 0);
        assert!(alloc_align % 2 == 0);

        use_align(3, block_hdr_align); // should panic
    }

    #[test]
    fn use_size_returns_multiple_of_block_header_size() {
        let block_hdr_size: usize = 8;
        let mut request_size: usize = 11;
        let mut alloc_size = use_size(request_size, block_hdr_size);

        assert!(request_size % block_hdr_size != 0);
        assert!(alloc_size % block_hdr_size == 0);

        request_size = block_hdr_size + 1;
        alloc_size = use_size(request_size, block_hdr_size);

        assert!(request_size % block_hdr_size != 0);
        assert!(alloc_size == 2 * block_hdr_size);
    }
}

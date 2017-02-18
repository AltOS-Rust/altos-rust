/*
* Copyright © 2017 AltOS-Rust Team
*
* This program is free software; you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation; either version 2 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful, but
* WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
* General Public License for more details.
*
* You should have received a copy of the GNU General Public License along
* with this program; if not, write to the Free Software Foundation, Inc.,
* 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/

// Linked list code for memory allocator
// This is intended for use by the free_list_allocator functionality

use core::{mem, ptr};

// Should not need to call function every time
// static mut MIN_ALLOC_SIZE : usize = 8;

// TODO: Add option to next_block
#[repr(C)]
pub struct BlockHeader {
    block_size: usize,
    next_block: *mut BlockHeader,
}

impl BlockHeader {
    const fn new(size: usize) -> Self {
        BlockHeader {
            block_size: size,
            next_block: ptr::null_mut(),
        }
    }
}

pub struct FreeList {
    head: *mut BlockHeader,
}

impl FreeList {
    pub const fn new() -> Self {
        FreeList {
            head: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        let block_position = heap_start as *mut BlockHeader;
        unsafe {
            ptr::write(&mut *block_position, BlockHeader::new(heap_size));
        }
        self.head = block_position;
    }

    // This will only relocate blocks to higher addresses in memory
    // but that's all we're using it for
    fn relocate_block(&self, current_pos: *mut BlockHeader, offset_val: usize) -> *mut BlockHeader {
        unsafe {
            // If we don't convert this, offset does not work correctly
            let current_ptr = current_pos as *mut u8;
            let new_pos = current_ptr.offset(offset_val as isize) as *mut BlockHeader;
            let current_block = ptr::read(current_pos);
            mem::forget(mem::replace(&mut *new_pos, current_block));
            new_pos as *mut BlockHeader
        }
    }

    // Allocate memory using first fit strategy
    // TODO: This probably needs to deal with alignment in someway
    pub fn allocate(&mut self, request_size: usize, request_align: usize) -> *mut u8 {
        let mut alloc_location: *mut u8 = ptr::null_mut();
        let using_size = use_size(request_size);
        let using_align = use_align(request_align);
        unsafe {
            let (mut previous, mut current) = (ptr::null_mut(), self.head);
            while !current.is_null() {
                let current_size = (*current).block_size;
                // Due to alignment, we should never get a case
                // where 0 < remaining_size < block_size

                // BlockHeader does not have enough space to satisfy requirement
                if current_size < using_size {
                    previous = current;
                    // If current is null, this will not work!
                    current = (*current).next_block;
                    continue;
                }
                // There is no block space remaining
                else if current_size == using_size {
                    // If at head, there is no previous to adjust
                    if self.head == current {
                        self.head = (*self.head).next_block;
                    }
                    else {
                        (*previous).next_block = (*current).next_block;
                    }
                }
                // BlockHeader has enough space and a block can be maintained
                else {
                    (*current).block_size -= using_size;
                    if self.head == current {
                        self.head = self.relocate_block(current, using_size);
                    }
                    else {
                        (*previous).next_block = self.relocate_block(current, using_size);
                    }
                }
                alloc_location = current as *mut u8;
                break;
            }
        }

        if alloc_location.is_null() {
            panic!("Out of memory.");
        }
        alloc_location
    }

    pub fn deallocate(&mut self, alloc_ptr: *mut u8, size: usize) {
        unsafe {
            // We can immediately add the block at the deallocated position
            let alloc_block_ptr = alloc_ptr as *mut BlockHeader;
            let used_memory = use_size(size);
            ptr::write(&mut *alloc_block_ptr, BlockHeader::new(used_memory));

            let (mut previous, mut current) = (ptr::null_mut(), self.head);
            // Memory location to be added at head of the list
            if alloc_block_ptr < current {
                (*alloc_block_ptr).next_block = current;
                self.head = alloc_block_ptr;
                return;
            }
            while !current.is_null() {
                // Will this comparison work the way I expect? Comparing pointer positions.
                if previous < alloc_block_ptr && alloc_block_ptr < current {
                    (*previous).next_block = alloc_block_ptr;
                    (*alloc_block_ptr).next_block = current;
                    return;
                }
                previous = current;
                current = (*current).next_block;
            }
            // At this point, we know that it needs to be added at the end
            (*previous).next_block = alloc_block_ptr;
        }
        /*
        TODO: We can still implement merging to deal with fragmentation
        - Nothing adjacent: Make new block, connect to closest blocks
        - Adjacent at tail: Merge with tail block, move block, switch leading ptr
        - Adjacent at lead: Merge with lead, no additional changes
        - Adjacent at both: Merge two with lead (add sizes, switch lead ptr)
        */
    }
    // fn traverse() {}
    // fn reallocate_inplace() {}
    // fn reallocate() {}
}

// This ensures the block size actually allocated is a multiple of the BlockHeader size.
// Actual allocation size >= requested size (obviously)
fn use_size(request_size: usize) -> usize {
    align_up(request_size, mem::size_of::<BlockHeader>())
}

// Returns whichever alignment is larger, BlockHeader's or the requested one.
// Assumes that both BlockHeader and the requested alignment are powers of 2
fn use_align(align: usize) -> usize {
    let block_hdr_align = mem::align_of::<BlockHeader>();

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

    const HEAP_SIZE: usize = 2048;

    // TODO: Can we claim an arbitrary amount of memory to use for testing?

    #[test]
    fn empty_free_list() {
        let free_list = FreeList::new();
        assert!(free_list.head.is_null());
    }

    #[test]
    fn free_list_init() {
        let mut heap: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
        let heap_start = &heap[0] as *const u8;
        let mut free_list = FreeList::new();
        free_list.init(heap_start as usize, HEAP_SIZE);

        assert_eq!(free_list.head, heap_start as *mut BlockHeader);
        unsafe {
            assert_eq!((*free_list.head).block_size, HEAP_SIZE);
        }
    }

    #[test]
    #[should_panic]
    fn use_align_returns_common_multiple_of_request_size_and_block_header_size() {
        let block_hdr_align = mem::align_of::<BlockHeader>();
        let mut alloc_align = use_align(1);

        assert!(alloc_align % block_hdr_align == 0);

        alloc_align = use_align(2);

        assert!(alloc_align % block_hdr_align == 0);
        assert!(alloc_align % 2 == 0);

        alloc_align = use_align(3); // should panic
    }

    #[test]
    fn use_size_returns_multiple_of_block_header_size() {
        let block_hdr_size: usize = mem::size_of::<BlockHeader>();
        let mut request_size: usize = 11; // prime number
        let mut alloc_size = use_size(request_size);

        assert!(request_size % block_hdr_size != 0);
        assert!(alloc_size % block_hdr_size == 0);

        request_size = block_hdr_size + 1;
        alloc_size = use_size(request_size);

        assert!(request_size % block_hdr_size != 0);
        assert!(alloc_size == 2 * block_hdr_size);
    }
}

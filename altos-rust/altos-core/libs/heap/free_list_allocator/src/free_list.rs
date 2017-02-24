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
 * Linked list code for memory allocator
 * This is intended for use by the free_list_allocator functionality
 */

use core::{mem, ptr};

use alignment;

// BlockHeader keeps track of a free block of memory
#[cfg(not(test))]
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

// FreeList is a linked list which keeps track of free blocks of memory
// Free blocks are embedded in the free memory itself
#[cfg(not(test))]
pub struct FreeList {
    block_hdr_size: usize,
    head: *mut BlockHeader,
}

// These are (trivially) implemented so FreeList objects can be passed
// between threads.
unsafe impl Send for FreeList {}
unsafe impl Sync for FreeList {}

impl FreeList {
    pub const fn new() -> Self {
        FreeList {
            block_hdr_size: 0,
            head: ptr::null_mut(),
        }
    }

    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.block_hdr_size = mem::size_of::<BlockHeader>();
        // Forcing BlockHeader size to be a power of two avoids weird issues with alignment
        // and extra logic we would have to include elsewhere
        if !self.block_hdr_size.is_power_of_two() {
            panic!("block header size is not power of two");
        }

        let start_position = heap_start as *mut BlockHeader;
        let use_heap_size = alignment::align_down(heap_size, self.block_hdr_size);
        unsafe {
            ptr::write(&mut *start_position, BlockHeader::new(use_heap_size));
        }

        self.head = start_position;
    }

    pub fn get_block_hdr_size(&self) -> usize {
        self.block_hdr_size
    }

    // Traverses the free list, looking for two sequential nodes which return true
    // when passed into match_condition.
    fn find_block(&mut self, match_condition: &Fn(*mut BlockHeader, *mut BlockHeader) -> bool)
        -> (*mut BlockHeader, *mut BlockHeader) {

        let (mut previous, mut current) = (ptr::null_mut(), self.head);
        while !current.is_null() {
            if !match_condition(previous, current) {
                previous = current;
                current = unsafe { (*current).next_block };
                continue;
            }
            break;
        }
        (previous, current)
    }

    // Allocate memory using first fit strategy
    // Returns pointer to allocated memory, or null if no memory is remaining
    pub fn allocate(&mut self, request_size: usize, request_align: usize) -> *mut u8 {
        let using_size = alignment::align_up(request_size, self.block_hdr_size);
        let using_align = alignment::use_align(request_align, self.block_hdr_size);

        // Will return true if current size is large enough to accomodate request
        let acceptable_block = |_, current: *mut BlockHeader| {
            let alloc_start = alignment::align_up(current as usize, using_align);
            let align_diff = alloc_start - current as usize;
            let current_size = unsafe { (*current).block_size };

            if current_size - align_diff < using_size { false } else { true }
        };

        let (previous, current) = self.find_block(&acceptable_block);
        if current.is_null() {
            return current as *mut u8;
        }

        let alloc_start = alignment::align_up(current as usize, using_align);
        let align_diff = alloc_start - current as usize;
        let current_size = unsafe { (*current).block_size };

        // Block is correct size and alignment.
        if current_size == using_size && align_diff == 0 {
            // If at head, there is no previous to adjust
            if self.head == current {
                self.head = unsafe { (*self.head).next_block };
            }
            else {
                unsafe { (*previous).next_block = (*current).next_block; }
            }
        }
        // Current block is larger than required and has the correct alignment
        else if current_size > using_size && align_diff == 0 {
            unsafe { (*current).block_size -= using_size; }
            if self.head == current {
                self.head = self.shift_block_forward(current, using_size);
            }
            else {
                unsafe {
                    (*previous).next_block = self.shift_block_forward(current, using_size);
                }
            }
        }
        // Current block is larger than required but has exactly the right size to
        // accomodate the alignment and size requirements
        else if current_size == using_size + align_diff {
            unsafe { (*current).block_size = align_diff; }
        }
        // Current block is larger than required and because of alignment, the allocation
        // divides it in two.
        else {
            unsafe {
                (*current).block_size = current_size - using_size - align_diff;
                let upper_block = self.shift_block_forward(current, using_size + align_diff);
                ptr::write(current as *mut BlockHeader, BlockHeader::new(align_diff));
                (*current).next_block = upper_block;
            }
        }
        alloc_start as *mut u8
    }

    /*
    TODO: We can still implement merging to deal with fragmentation
    - Nothing adjacent: Make new block, connect to closest blocks
    - Adjacent at tail: Merge with tail block, move block, switch leading ptr
    - Adjacent at lead: Merge with lead, no additional changes
    - Adjacent at both: Merge two with lead (add sizes, switch lead ptr)
    */

    pub fn deallocate(&mut self, alloc_ptr: *mut u8, size: usize, _align: usize) {
        // We can immediately add the block at the deallocated position
        let alloc_block_ptr = alloc_ptr as *mut BlockHeader;
        let used_memory = alignment::align_up(size, self.block_hdr_size);
        unsafe {
            ptr::write(&mut *alloc_block_ptr, BlockHeader::new(used_memory));
        }

        // Memory location to be added at head of the list
        if self.head.is_null() || alloc_block_ptr < self.head {
            unsafe { (*alloc_block_ptr).next_block = self.head; }
            self.head = alloc_block_ptr;
            return;
        }
        let (mut previous, current) = {
            let free_mem_between = |previous, current| {
                previous < alloc_block_ptr && alloc_block_ptr < current
            };
            self.find_block(&free_mem_between)
        };
        unsafe {
            (*previous).next_block = alloc_block_ptr;
            (*alloc_block_ptr).next_block = current;
        }
    }

    // This relocates BlockHeaders in memory, used when we do allocations.
    fn shift_block_forward(&self, current_pos: *mut BlockHeader, offset_val: usize) -> *mut BlockHeader {
        // If we don't convert this, offset does not work correctly
        let current_ptr = current_pos as *mut u8;
        unsafe {
            let new_pos = current_ptr.offset(offset_val as isize) as *mut BlockHeader;
            let current_block = ptr::read(current_pos);
            *new_pos = current_block;
            new_pos as *mut BlockHeader
        }
    }
}

// For testing purposes, we make versions of the list structures with public data
#[cfg(test)]
#[repr(C)]
pub struct BlockHeader {
    pub block_size: usize,
    pub next_block: *mut BlockHeader,
}

#[cfg(test)]
pub struct FreeList {
    pub block_hdr_size: usize,
    pub head: *mut BlockHeader,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    // Check each of the node merging cases
    // Make sure tests hit every case in allocate

    // Free list starts out with head set to null on creation
    #[test]
    fn new_free_list_is_empty() {
        let free_list = FreeList::new();
        assert!(free_list.head.is_null());
    }

    // List initialization creates single block with entire size
    #[test]
    fn free_list_init() {
        let heap_size: usize = 2048;
        let tfl = test::get_free_list_with_size(heap_size);
        let heap_start = tfl.get_heap_start() as *mut BlockHeader;

        assert!(!tfl.free_list.head.is_null());
        assert_eq!(tfl.free_list.head, heap_start);
        assert_eq!(tfl.free_list.get_block_hdr_size(), mem::size_of::<BlockHeader>());
        unsafe {
            assert_eq!((*tfl.free_list.head).block_size, heap_size);
        }
    }

    // Initializing the free list adjusts first memory block so the adress and size
    // are multiples of block header size.
    #[test]
    fn free_list_init_adjusts_to_block_header_size() {
        let heap_size: usize = 2048 + 1;
        let tfl = test::get_free_list_with_size(heap_size);

        assert!(!tfl.free_list.head.is_null());
        unsafe {
            assert!((*tfl.free_list.head).block_size % tfl.free_list.get_block_hdr_size() == 0);
        }
    }

    // Multiple allocations without deallocations
    #[test]
    fn free_list_multiple_allocations() {
        let heap_size: usize = 2048;
        let mut tfl = test::get_free_list_with_size(heap_size);

        // Allocations we're using in these tests should be multiple of size_of::<BlockHeader>()
        // This is to avoid having to account for alignment with this test
        tfl.free_list.allocate(32, 1);
        tfl.free_list.allocate(128, 1);
        tfl.free_list.allocate(256, 1);
        unsafe {
            assert_eq!((*tfl.free_list.head).block_size, heap_size - (32 + 128 + 256));
            assert!((*tfl.free_list.head).next_block.is_null());
        }
    }

    // Does allocations which results in the elimination of a free block
    #[test]
    fn free_list_allocations_use_entire_free_block() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.free_list.allocate(256, 1);
        tfl.free_list.allocate(256, 1);

        tfl.free_list.deallocate(alloc_ptr, 256, 1);
        unsafe {
            assert_eq!((*tfl.free_list.head).block_size, 256);
            assert!(!(*tfl.free_list.head).next_block.is_null());
        }

        // New allocation should claim the entire first block
        tfl.free_list.allocate(256, 1);

        unsafe {
            assert_eq!((*tfl.free_list.head).block_size, heap_size - (256 + 256));
            assert!((*tfl.free_list.head).next_block.is_null());
        }
    }

    #[test]
    fn deallocate_at_start() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.free_list.allocate(256, 1);
        assert_eq!(unsafe { (*tfl.free_list.head).block_size }, 1024 - 256);
        let new_head_ptr = tfl.free_list.head;

        tfl.free_list.deallocate(alloc_ptr, 256, 1);

        assert_eq!(tfl.count_free_blocks(), 2);
        assert_eq!(tfl.sum_free_block_memory(), heap_size);
        assert_eq!(tfl.free_list.head, alloc_ptr as *mut BlockHeader);
        assert_eq!(unsafe { (*tfl.free_list.head).next_block }, new_head_ptr);
    }

    #[test]
    fn deallocate_at_end() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.free_list.allocate(512, 1);
        let alloc_ptr2 = tfl.free_list.allocate(256, 1);
        let alloc_ptr3 = tfl.free_list.allocate(256, 1);

        tfl.free_list.deallocate(alloc_ptr, 512, 1);
        // Both of these deallocations should create nodes at the end of the list
        tfl.free_list.deallocate(alloc_ptr2, 256, 1);
        tfl.free_list.deallocate(alloc_ptr3, 256, 1);

        assert_eq!(tfl.count_free_blocks(), 3);
        assert_eq!(tfl.sum_free_block_memory(), heap_size);
        assert_eq!(tfl.free_list.head, alloc_ptr as *mut BlockHeader);
        assert_eq!(unsafe { (*tfl.free_list.head).next_block }, alloc_ptr2 as *mut BlockHeader);
    }

    #[test]
    fn deallocate_in_middle() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.free_list.allocate(512, 1);
        let alloc_ptr2 = tfl.free_list.allocate(256, 1);
        let alloc_ptr3 = tfl.free_list.allocate(256, 1);
        tfl.free_list.deallocate(alloc_ptr, 512, 1);
        tfl.free_list.deallocate(alloc_ptr3, 256, 1);

        // This should get put in between the nodes created from alloc_ptr and alloc_ptr3
        tfl.free_list.deallocate(alloc_ptr2, 256, 1);

        assert_eq!(tfl.count_free_blocks(), 3);
        assert_eq!(tfl.sum_free_block_memory(), heap_size);
        assert_eq!(tfl.free_list.head, alloc_ptr as *mut BlockHeader);
        assert_eq!(unsafe { (*tfl.free_list.head).next_block }, alloc_ptr2 as *mut BlockHeader);
    }

    #[test]
    fn allocations_deal_with_alignment_correctly() {
        // Should get aligned down based on block header size
        // Should be 328 if block header size is 8, 320 if it's 16
        let heap_size: usize = 330;
        let block_hdr_size = mem::size_of::<BlockHeader>();
        let aligned_heap_size = alignment::align_down(heap_size, block_hdr_size);

        let mut tfl = test::get_free_list_with_size(heap_size);

        let test_sizes = (60, 122, 54);
        let test_size_alignments = (
            alignment::align_up(test_sizes.0, block_hdr_size),
            alignment::align_up(test_sizes.1, block_hdr_size),
            alignment::align_up(test_sizes.2, block_hdr_size),
        );

        assert_eq!(tfl.sum_free_block_memory(), aligned_heap_size);
        // This should be aligned to 64
        tfl.free_list.allocate(test_sizes.0, 1);
        let mut expected_remainder = aligned_heap_size - test_size_alignments.0;
        let mut remaining_size = tfl.sum_free_block_memory();
        assert_eq!(remaining_size, expected_remainder);
        assert_eq!(remaining_size % block_hdr_size, 0);
        // This should be aligned to 128
        tfl.free_list.allocate(test_sizes.1, 1);
        expected_remainder = expected_remainder - test_size_alignments.1;
        remaining_size = tfl.sum_free_block_memory();
        assert_eq!(remaining_size, expected_remainder);
        assert_eq!(remaining_size % block_hdr_size, 0);
        // This will depend on block header size. Either 56, or 64.
        tfl.free_list.allocate(test_sizes.2, 1);
        expected_remainder = expected_remainder - test_size_alignments.2;
        remaining_size = tfl.sum_free_block_memory();
        assert_eq!(remaining_size, expected_remainder);
        assert_eq!(remaining_size % block_hdr_size, 0);

        // Regardless of block header size, there should be more than 0 memory remaining
        match block_hdr_size {
            16 => assert_eq!(remaining_size, 320 - (128 + 64 + 64)),
            8 => assert_eq!(remaining_size, 328 - (128 + 64 + 56)),
            _ => assert!(remaining_size > 0),
        }
    }

    #[test]
    fn deallocations_do_not_leak_memory() {
        // Should get aligned down based on block header size
        // Should be 328 if block header size is 8, 320 if it's 16
        let heap_size: usize = 330;
        let block_hdr_size = mem::size_of::<BlockHeader>();
        let aligned_heap_size = alignment::align_down(heap_size, block_hdr_size);

        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc1 = tfl.free_list.allocate(60, 1);
        let alloc2 = tfl.free_list.allocate(122, 1);
        let alloc3 = tfl.free_list.allocate(54, 1);
        tfl.free_list.deallocate(alloc1, 60, 1);
        tfl.free_list.deallocate(alloc2, 122, 1);
        tfl.free_list.deallocate(alloc3, 54, 1);

        assert_eq!(tfl.sum_free_block_memory(), aligned_heap_size);
        assert!(
            tfl.each_free_block_satisfies(
                &|current| unsafe { (*current).block_size % block_hdr_size == 0 }
            )
        );
    }

    #[test]
    fn many_allocations_and_deallocations_do_not_leak_memory() {
        // Should get aligned down based on block header size
        // Should be 328 if block header size is 8, 320 if it's 16
        let heap_size: usize = 330;
        let block_hdr_size = mem::size_of::<BlockHeader>();
        let aligned_heap_size = alignment::align_down(heap_size, block_hdr_size);

        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc1 = tfl.free_list.allocate(60, 1);
        let alloc2 = tfl.free_list.allocate(122, 1);
        tfl.free_list.deallocate(alloc1, 60, 1);
        let alloc3 = tfl.free_list.allocate(54, 4);
        tfl.free_list.deallocate(alloc2, 122, 1);
        let alloc4 = tfl.free_list.allocate(36, 8);
        tfl.free_list.deallocate(alloc3, 54, 4);
        tfl.free_list.deallocate(alloc4, 36, 8);
        let alloc5 = tfl.free_list.allocate(8, 32);
        let alloc6 = tfl.free_list.allocate(4, 1);
        tfl.free_list.deallocate(alloc5, 8, 32);
        tfl.free_list.deallocate(alloc6, 4, 1);

        assert_eq!(tfl.sum_free_block_memory(), aligned_heap_size);
        assert!(
            tfl.each_free_block_satisfies(
                &|current| unsafe { (*current).block_size % block_hdr_size == 0 }
            )
        );
    }

    #[test]
    fn free_list_out_of_memory_returns_null() {
        let heap_size: usize = 512;
        let mut tfl = test::get_free_list_with_size(heap_size);

        tfl.free_list.allocate(512, 1);
        assert!(tfl.free_list.allocate(64, 1).is_null());
    }

    #[test]
    fn free_list_not_enough_memory_returns_null() {
        let heap_size: usize = 512;
        let mut tfl = test::get_free_list_with_size(heap_size);

        tfl.free_list.allocate(256, 1);
        assert!(tfl.free_list.allocate(512, 1).is_null());
    }
}

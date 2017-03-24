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

//! Linked list code for the memory allocator.
//!
//! This is intended for use by the free_list_allocator to keep track of free blocks of memory.

use core::{mem, ptr};

use alignment;

/// A link pointing to a BlockHeader.
///
/// This type is used to provide a safe interface for getting links in our FreeList.
#[derive(Copy, Clone)]
pub struct Link(*const BlockHeader);

impl Link {
    unsafe fn new(ptr: *const BlockHeader) -> Self {
        Link(ptr)
    }

    const fn null() -> Self {
        Link(ptr::null())
    }

    pub fn get_ref_mut(&mut self) -> Option<&'static mut BlockHeader> {
        if self.0.is_null() {
            None
        }
        else {
            unsafe { Some(&mut *(self.0 as *mut _)) }
        }
    }

    pub fn get_ref(&self) -> Option<&'static BlockHeader> {
        if self.0.is_null() {
            None
        }
        else {
            unsafe { Some(&*self.0) }
        }
    }

    fn as_ptr(&self) -> *const BlockHeader {
        self.0
    }
}

impl<'a> From<&'a BlockHeader> for Link {
    fn from(block: &'a BlockHeader) -> Self {
        unsafe { Link::new(block) }
    }
}

impl<'a> From<&'a mut BlockHeader> for Link {
    fn from(block: &'a mut BlockHeader) -> Self {
        unsafe { Link::new(block) }
    }
}

impl<'a> From<Option<&'a BlockHeader>> for Link {
    fn from(block: Option<&'a BlockHeader>) -> Self {
        match block {
            Some(block) => Link::from(block),
            None => Link::null(),
        }
    }
}

impl<'a> From<Option<&'a mut BlockHeader>> for Link {
    fn from(block: Option<&'a mut BlockHeader>) -> Self {
        match block {
            Some(block) => Link::from(block),
            None => Link::null(),
        }
    }
}

/// BlockHeader nodes keep track of a free block of memory.
pub struct BlockHeader {
    /// Size of block in bytes.
    pub block_size: usize,
    /// Next block in the list.
    pub next_block: Link,
}

impl BlockHeader {
    const fn new(size: usize) -> Self {
        BlockHeader {
            block_size: size,
            next_block: Link::null(),
        }
    }

    fn as_ptr(&self) -> *const Self {
        self as *const _
    }
}

/// `FreeList` is a linked list which keeps track of free blocks of memory. Free blocks are
/// embedded in the free memory itself so that the system does not require additional memory
/// overhead to keep track of free memory.
pub struct FreeList {
    pub head: Link,
}

// These are (trivially) implemented so `FreeList` objects can be passed between threads.
unsafe impl Send for FreeList {}
unsafe impl Sync for FreeList {}

impl FreeList {
    pub const fn new() -> Self {
        FreeList {
            head: Link::null(),
        }
    }

    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        // We adjust the heap starting position and size so that it will initially have
        // a starting position and size aligned to the block header size.
        // This potentially leaks a few bytes but it might cause errors if we didn't do it.
        let mut heap = unsafe {
            let start = alignment::align_up(heap_start, mem::size_of::<BlockHeader>());
            Link::new(start as *const BlockHeader)
        };
        let align_diff = heap.as_ptr() as usize - heap_start;

        // Adjust the heap size down based on alignment change to starting position
        // and then adjust it down again if it's not aligned to block header size.
        let use_heap_size =
            alignment::align_down(heap_size - align_diff, mem::size_of::<BlockHeader>());

        match heap.get_ref_mut() {
            Some(start) => *start = BlockHeader::new(use_heap_size),
            None => unreachable!(),
        }
        self.head = heap;
    }

    // Traverses the free list, looking for two sequential free blocks which return true
    // when passed into match_condition.
    fn find_block<F: Fn(Option<&BlockHeader>, &BlockHeader) -> bool>(&mut self, match_condition: F)
        -> (Option<&'static mut BlockHeader>, Option<&'static mut BlockHeader>) {

        let mut previous: Link = Link::null();
        let mut current = self.head;
        while let Some(curr) = current.get_ref().take() {
            if match_condition(previous.get_ref(), curr) {
                break;
            }
            previous = current;
            current = curr.next_block;
        }
        (previous.get_ref_mut(), current.get_ref_mut())
    }

    // This relocates BlockHeaders in memory, used when we do allocations.
    fn shift_block_forward(&self, current_pos: &BlockHeader, offset_val: usize) -> Link {
        // If we don't convert this, offset does not work correctly
        let current_ptr = current_pos.as_ptr() as *mut u8;
        unsafe {
            let new_pos = current_ptr.offset(offset_val as isize) as *mut BlockHeader;
            *new_pos = ptr::read(current_pos);
            Link::new(new_pos)
        }
    }

    // Allocate memory using the first fit strategy
    // Returns pointer to allocated memory, or null if no memory can be found
    pub fn allocate(&mut self, request_size: usize, request_align: usize) -> *mut u8 {
        // Every allocation and deallocation is forced to have a size which is a multiple
        // of the BlockHeader size. This allows us to avoid potential issues with blocks of
        // memory that are too small to accomodate a BlockHeader node, but it also potentially
        // allocates slightly more memory than is needed.
        let using_size = alignment::align_up(request_size, mem::size_of::<BlockHeader>());
        let using_align = alignment::use_align(request_align, mem::size_of::<BlockHeader>());

        // Will find a block with enough size to accomodate request
        let (previous, current) = self.find_block(|_, current| {
            let current_addr = current.as_ptr() as usize;
            let alloc_start = alignment::align_up(current_addr, using_align);
            let align_diff = alloc_start - current_addr;
            let current_size = current.block_size;

            !(current_size - align_diff < using_size)
        });

        // If current is null, that means no free block was found that's large enough.
        match current {
            Some(current) => {
                let current_addr = current.as_ptr() as usize;
                let alloc_start = alignment::align_up(current_addr, using_align) as *mut u8;
                let align_diff = alloc_start as usize - current_addr;
                let current_size = current.block_size;

                assert!(align_diff == 0 || align_diff >= mem::size_of::<BlockHeader>());

                // Block is correct size and alignment.
                if current_size == using_size && align_diff == 0 {
                    // If at head, there is no previous to adjust
                    match previous {
                        Some(previous) => previous.next_block = current.next_block,
                        None => self.head = current.next_block,
                    }
                }
                // Current block is larger than required and has the correct alignment
                else if current_size > using_size && align_diff == 0 {
                    current.block_size -= using_size;
                    match previous {
                        Some(previous) => previous.next_block = self.shift_block_forward(current, using_size),
                        None => self.head = self.shift_block_forward(current, using_size),
                    }
                }
                // Current block is larger than required but has exactly the right size to
                // accomodate the alignment and size requirements.
                else if current_size == using_size + align_diff {
                    current.block_size = align_diff;
                }
                // Current block is larger than required and because of alignment, the allocation
                // divides it in two.
                else {
                    current.block_size = current_size - using_size - align_diff;
                    let upper_block = self.shift_block_forward(current, using_size + align_diff);
                    current.block_size = align_diff;
                    current.next_block = upper_block;
                }
                alloc_start
            },
            None => ptr::null_mut(),
        }
    }

    // Deallocates memory, placing it back in the free list as a free block for later use.
    // Adds a free block to the list based on alloc_ptr so that the list remains
    // sorted based on memory position. Merges adjacent free blocks with the deallocated block.
    pub fn deallocate(&mut self, alloc_ptr: *mut u8, size: usize, _align: usize) {
        // Creates a free block, dealloc_block, with size adjusted to multiples of BlockHeader size
        let mut dealloc_block = unsafe { Link::new(alloc_ptr as *const BlockHeader) };
        let used_memory = alignment::align_up(size, mem::size_of::<BlockHeader>());

        match dealloc_block.get_ref_mut() {
            Some(block) => *block = BlockHeader::new(used_memory),
            None => panic!("deallocate - tried to deallocate a null pointer!"),
        }

        // Traverses the free list, locating neighboring blocks to dealloc_block based on alloc_ptr
        let (previous, current) = self.find_block(|previous, current| {
            if dealloc_block.as_ptr() == current.as_ptr() {
                panic!("deallocate - attempt to free memory that's already free");
            }
            match previous {
                None => dealloc_block.as_ptr() < current.as_ptr(),
                Some(previous) => {
                    previous.as_ptr() < dealloc_block.as_ptr()
                    && dealloc_block.as_ptr() < current.as_ptr()
                },
            }
        });

        // Determine if the deallocated block is adjacent to the previous (leading) free block.
        let merge_with_previous = match previous {
            Some(ref previous) => {
                previous.as_ptr() as usize + previous.block_size == dealloc_block.as_ptr() as usize
            },
            None => false,
        };

        // Determine if the deallocated block is adjacent to the current (following) free block.
        let merge_with_current = match current {
            Some(ref current) => {
                dealloc_block.as_ptr() as usize + used_memory == current.as_ptr() as usize
            },
            None => false,
        };

        // Merge with previous (leading) or current (following) free block or both.
        // Block with the lowest address (previous block) becomes super block.
        match (previous, current) {
            // Deallocation is between two free blocks
            (Some(previous), Some(current)) => {
                if merge_with_previous && merge_with_current {
                    previous.block_size += used_memory + current.block_size;
                    previous.next_block = current.next_block;
                }
                else if merge_with_previous {
                    previous.block_size += used_memory;
                }
                else if merge_with_current {
                    previous.next_block = dealloc_block;
                    dealloc_block.get_ref_mut().unwrap().block_size += current.block_size;
                    dealloc_block.get_ref_mut().unwrap().next_block = current.next_block;
                }
                else {
                    previous.next_block = dealloc_block;
                    dealloc_block.get_ref_mut().unwrap().next_block = Link::from(current);
                }
            },
            // Deallocation is at the end of the free list
            (Some(previous), None) => {
                if merge_with_previous {
                    previous.block_size += used_memory;
                }
                else {
                    previous.next_block = dealloc_block;
                    dealloc_block.get_ref_mut().unwrap().next_block = Link::null();
                }
            },
            // Deallocation is at the head of the free list
            (None, Some(current)) => {
                if merge_with_current {
                    dealloc_block.get_ref_mut().unwrap().block_size += current.block_size;
                    dealloc_block.get_ref_mut().unwrap().next_block = current.next_block;
                } else {
                    dealloc_block.get_ref_mut().unwrap().next_block = Link::from(current);
                }
                self.head = dealloc_block;
            }
            // Free list is empty, so the deallocation becomes the only block
            (None, None) => {
                dealloc_block.get_ref_mut().unwrap().next_block = self.head;
                self.head = dealloc_block;
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn new_free_list_is_empty() {
        let free_list = FreeList::new();
        assert!(free_list.head.get_ref().is_none());
    }

    #[test]
    fn free_list_init() {
        let heap_size: usize = 2048;
        let tfl = test::get_free_list_with_size(heap_size);
        let heap_start = tfl.get_heap_start() as *mut BlockHeader;

        assert!(!tfl.head.get_ref().is_none());
        assert_eq!(tfl.head.as_ptr(), heap_start);
        // The size of BlockHeader should always be a power of 2 to avoid potential memory issues.
        assert!(mem::size_of::<BlockHeader>().is_power_of_two());
        // List initialization creates single block with entire size
        assert_eq!(tfl.head.block_size, heap_size);
    }

    // Initializing the free list adjusts first memory block so the address and size
    // are multiples of block header size.
    #[test]
    fn free_list_init_adjusts_to_block_header_size() {
        let heap_size: usize = 2048 + 1;
        let tfl = test::get_free_list_with_size(heap_size);

        assert!(!tfl.head.get_ref().is_none());
        assert!(tfl.head.block_size % mem::size_of::<BlockHeader>() == 0);
    }

    #[test]
    fn multiple_allocations() {
        let heap_size: usize = 2048;
        let mut tfl = test::get_free_list_with_size(heap_size);

        // Allocations we're using in these tests should be multiple of size_of::<BlockHeader>()
        // This is to avoid having to account for alignment with this test
        tfl.allocate(32, 1);
        tfl.allocate(128, 1);
        tfl.allocate(256, 1);
        assert_eq!(tfl.head.block_size, heap_size - (32 + 128 + 256));
        assert!(tfl.head.next_block.get_ref().is_none());
    }

    // Does allocations which results in the elimination of a free block
    #[test]
    fn allocations_use_entire_free_block() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.allocate(256, 1);
        tfl.allocate(256, 1);

        tfl.deallocate(alloc_ptr, 256, 1);
        assert_eq!(tfl.head.block_size, 256);
        assert!(!tfl.head.next_block.get_ref().is_none());

        // New allocation should claim the entire first block
        tfl.allocate(256, 1);

        assert_eq!(tfl.head.block_size, heap_size - (256 + 256));
        assert!(tfl.head.next_block.get_ref().is_none());
    }

    #[test]
    fn deallocation_merge_none() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr1 = tfl.allocate(64, 1);
        tfl.allocate(64, 1);
        let alloc_ptr3 = tfl.allocate(64, 1);
        tfl.allocate(64, 1);

        assert_eq!(tfl.count_free_blocks(), 1);

        tfl.deallocate(alloc_ptr1, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 2);

        tfl.deallocate(alloc_ptr3, 64, 1);
        assert_eq!(tfl.count_free_blocks(),  3);
        assert_eq!(tfl.sum_free_block_memory(), heap_size - 64 - 64);
    }

    // The following functions test the different deallocation and free block merging scenarios.
    #[test]
    fn deallocate_merge_both_sides() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr1 = tfl.allocate(64, 1);
        let alloc_ptr2 = tfl.allocate(64, 1);

        assert_eq!(tfl.count_free_blocks(), 1);

        tfl.deallocate(alloc_ptr1, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 2);

        tfl.deallocate(alloc_ptr2, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.sum_free_block_memory(), heap_size);
    }

    #[test]
    fn deallocate_merge_with_previous() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr1 = tfl.allocate(64, 1);
        let alloc_ptr2 = tfl.allocate(64, 1);
        tfl.allocate(64, 1);

        assert_eq!(tfl.count_free_blocks(), 1);

        tfl.deallocate(alloc_ptr1, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 2);

        tfl.deallocate(alloc_ptr2, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 2);
        assert_eq!(tfl.sum_free_block_memory(), heap_size - 64);
    }

    #[test]
    fn deallocate_merge_with_following() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.allocate(64, 1);
        tfl.allocate(64, 1);
        let alloc_ptr2 = tfl.allocate(64, 1);

        assert_eq!(tfl.count_free_blocks(), 1);

        tfl.deallocate(alloc_ptr, 64, 1);
        tfl.deallocate(alloc_ptr2, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 2);
        assert_eq!(tfl.sum_free_block_memory(), heap_size - 64);
    }

    #[test]
    fn deallocate_at_end_no_merge() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.allocate(512, 1);
        tfl.allocate(256, 1);
        let alloc_ptr3 = tfl.allocate(256, 1);

        assert_eq!(tfl.count_free_blocks(), 0);
        tfl.deallocate(alloc_ptr, 512, 1);
        tfl.deallocate(alloc_ptr3, 256, 1);

        assert_eq!(tfl.count_free_blocks(), 2);
        assert_eq!(tfl.sum_free_block_memory(), heap_size - 256);
    }

    #[test]
    fn deallocate_at_end_merge_previous() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        tfl.allocate(512, 1);
        let alloc_ptr2 = tfl.allocate(256, 1);
        let alloc_ptr3 = tfl.allocate(256, 1);

        assert_eq!(tfl.count_free_blocks(), 0);

        tfl.deallocate(alloc_ptr2, 256, 1);
        assert_eq!(tfl.count_free_blocks(), 1);

        tfl.deallocate(alloc_ptr3, 256, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.sum_free_block_memory(), heap_size - 512);
    }

    #[test]
    fn deallocate_at_head_no_merge() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        tfl.allocate(64, 1);
        let alloc_ptr = tfl.allocate(64, 1);
        tfl.allocate(64, 1);

        assert_eq!(tfl.count_free_blocks(), 1);

        tfl.deallocate(alloc_ptr, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 2);
        assert_eq!(tfl.sum_free_block_memory(), heap_size - 64 - 64);
    }

    #[test]
    fn deallocate_at_head_merge_following() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        tfl.allocate(64, 1);
        let alloc_ptr = tfl.allocate(64, 1);

        assert_eq!(tfl.count_free_blocks(), 1);

        tfl.deallocate(alloc_ptr, 64, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.sum_free_block_memory(), heap_size - 64);
    }

    #[test]
    fn deallocate_at_start() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.allocate(256, 1);
        assert_eq!(tfl.head.block_size, 1024 - 256);
        assert_eq!(tfl.head.next_block.as_ptr(), ptr::null());

        tfl.deallocate(alloc_ptr, 256, 1);

        // Deallocation merges with following block
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.sum_free_block_memory(), heap_size);
        assert_eq!(tfl.head.as_ptr(), alloc_ptr as *mut BlockHeader);
    }

    #[test]
    fn deallocate_at_end() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.allocate(512, 1);
        let alloc_ptr2 = tfl.allocate(256, 1);
        let alloc_ptr3 = tfl.allocate(256, 1);

        // Deallocation starting from high addresses moving to lower addresses.
        // Blocks always merge to one and list head moves down with each deallocation.
        tfl.deallocate(alloc_ptr3, 256, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.head.as_ptr(), alloc_ptr3 as *mut BlockHeader);

        tfl.deallocate(alloc_ptr2, 256, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.head.as_ptr(), alloc_ptr2 as *mut BlockHeader);

        tfl.deallocate(alloc_ptr, 512, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.head.as_ptr(), alloc_ptr as *mut BlockHeader);

        assert_eq!(tfl.sum_free_block_memory(), heap_size);
    }

    #[test]
    fn deallocate_in_middle() {
        let heap_size: usize = 1024;
        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc_ptr = tfl.allocate(512, 1);
        let alloc_ptr2 = tfl.allocate(256, 1);
        let alloc_ptr3 = tfl.allocate(256, 1);

        // Everything's been allocated so no blocks are free.
        assert_eq!(tfl.count_free_blocks(), 0);

        // Last allocation becomes the first deallocation and the head of the free list
        tfl.deallocate(alloc_ptr3, 256, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.head.as_ptr(), alloc_ptr3 as *mut BlockHeader);

        tfl.deallocate(alloc_ptr, 512, 1);
        assert_eq!(tfl.count_free_blocks(), 2);
        assert_eq!(tfl.head.as_ptr(), alloc_ptr as *mut BlockHeader);

        // This should get put in between the blocks created from alloc_ptr and alloc_ptr3
        // and merge them all into 1 block
        tfl.deallocate(alloc_ptr2, 256, 1);
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.head.as_ptr(), alloc_ptr as *mut BlockHeader);

        assert_eq!(tfl.sum_free_block_memory(), heap_size);
        assert_eq!(tfl.head.next_block.as_ptr(), ptr::null());
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
        tfl.allocate(test_sizes.0, 1);
        let mut expected_remainder = aligned_heap_size - test_size_alignments.0;
        let mut remaining_size = tfl.sum_free_block_memory();
        assert_eq!(remaining_size, expected_remainder);
        assert_eq!(remaining_size % block_hdr_size, 0);
        // This should be aligned to 128
        tfl.allocate(test_sizes.1, 1);
        expected_remainder = expected_remainder - test_size_alignments.1;
        remaining_size = tfl.sum_free_block_memory();
        assert_eq!(remaining_size, expected_remainder);
        assert_eq!(remaining_size % block_hdr_size, 0);
        // This will depend on block header size. Either 56, or 64.
        tfl.allocate(test_sizes.2, 1);
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

        let alloc1 = tfl.allocate(60, 1);
        let alloc2 = tfl.allocate(122, 1);
        let alloc3 = tfl.allocate(54, 1);

        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );

        tfl.deallocate(alloc1, 60, 1);
        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );

        tfl.deallocate(alloc2, 122, 1);
        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );

        tfl.deallocate(alloc3, 54, 1);
        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );
        assert_eq!(tfl.count_free_blocks(), 1);
        assert_eq!(tfl.sum_free_block_memory(), aligned_heap_size);
    }

    #[test]
    fn many_allocations_and_deallocations_do_not_leak_memory() {
        // Should get aligned down based on block header size
        // Should be 328 if block header size is 8, 320 if it's 16
        let heap_size: usize = 330;
        let block_hdr_size = mem::size_of::<BlockHeader>();
        let aligned_heap_size = alignment::align_down(heap_size, block_hdr_size);

        let mut tfl = test::get_free_list_with_size(heap_size);

        let alloc1 = tfl.allocate(60, 1);
        let alloc2 = tfl.allocate(122, 1);
        tfl.deallocate(alloc1, 60, 1);

        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );

        let alloc3 = tfl.allocate(54, 4);
        tfl.deallocate(alloc2, 122, 1);

        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );

        let alloc4 = tfl.allocate(36, 8);
        tfl.deallocate(alloc3, 54, 4);
        tfl.deallocate(alloc4, 36, 8);

        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );

        let alloc5 = tfl.allocate(8, 32);
        let alloc6 = tfl.allocate(4, 1);
        tfl.deallocate(alloc5, 8, 32);
        tfl.deallocate(alloc6, 4, 1);

        assert!(
            tfl.each_free_block_satisfies(|current| current.block_size % block_hdr_size == 0)
        );

        assert_eq!(tfl.sum_free_block_memory(), aligned_heap_size);
    }

    #[test]
    fn out_of_memory_returns_null() {
        let heap_size: usize = 512;
        let mut tfl = test::get_free_list_with_size(heap_size);

        tfl.allocate(512, 1);
        assert!(tfl.allocate(64, 1).is_null());
    }

    #[test]
    fn not_enough_memory_returns_null() {
        let heap_size: usize = 512;
        let mut tfl = test::get_free_list_with_size(heap_size);

        tfl.allocate(256, 1);
        assert!(tfl.allocate(512, 1).is_null());
    }
}

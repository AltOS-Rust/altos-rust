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
 * Provides a test framework for testing the free list allocator.
 */

use std::vec::Vec;
use std::ops::{Deref, DerefMut};
use free_list;

impl Deref for free_list::Link {
    type Target = free_list::BlockHeader;
    fn deref(&self) -> &Self::Target {
        self.get_ref().expect("Dereferencing null BlockHeader Link!")
    }
}

impl DerefMut for free_list::Link {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_ref_mut().expect("Dereferencing null BlockHeader Link!")
    }
}

// TestMemory is a helper type which represents a chunk of memory used in testing the allocator
// It uses a vector to create the necessary memory and manages deallocation of the vector
pub struct TestMemory {
    heap: *const u8,
    vector_capacity: usize,
}

impl TestMemory {
    pub fn new(heap_size: usize) -> TestMemory {
        let heap: Vec<u8> = Vec::with_capacity(heap_size);
        let heap_start = heap.as_ptr();
        ::core::mem::forget(heap);
        TestMemory {
            heap: heap_start,
            vector_capacity: heap_size,
        }
    }

    pub fn get_heap(&self) -> *const u8 {
        self.heap
    }

    fn get_memory_size(&self) -> usize {
        self.vector_capacity
    }
}

impl Drop for TestMemory {
    fn drop(&mut self) {
        unsafe {
            // Reform the vector from component parts in order to make sure it's deallocated
            let heap = Vec::from_raw_parts(
                self.heap as *mut u8, self.vector_capacity, self.vector_capacity
            );
            drop(heap);
        }
    }
}

// TestFreeList is a helper type for free lists, which utilizes TestMemory
// Provides some functions which help make it easier to check certain properties of the free list
pub struct TestFreeList {
    pub free_list: free_list::FreeList,
    test_memory: TestMemory,
}

impl Deref for TestFreeList {
    type Target = free_list::FreeList;

    fn deref(&self) -> &free_list::FreeList {
        &self.free_list
    }
}

impl DerefMut for TestFreeList {
    fn deref_mut(&mut self) -> &mut free_list::FreeList {
        &mut self.free_list
    }
}

impl TestFreeList {
    fn new(heap_size: usize) -> TestFreeList {
        let test_memory = TestMemory::new(heap_size);
        let mut free_list = free_list::FreeList::new();
        free_list.init(test_memory.get_heap() as usize, test_memory.get_memory_size());
        TestFreeList {
            free_list: free_list,
            test_memory: test_memory,
        }
    }

    pub fn get_heap_start(&self) -> *const u8 {
        self.test_memory.get_heap()
    }

    // Helper function which returns the sum of the block_sizes for every BlockHeader in the list
    pub fn sum_free_block_memory(&self) -> usize {
        let mut current = self.free_list.head;
        let mut sum: usize = 0;
        while let Some(curr) = current.get_ref() {
            sum += curr.block_size;
            current = curr.next_block;
        }
        sum
    }

    // Helper function to get the number of BlockHeader nodes in the list
    pub fn count_free_blocks(&self) -> usize {
        let mut current = self.free_list.head;
        let mut num_blocks = 0;
        while let Some(curr) = current.get_ref() {
            num_blocks += 1;
            current = curr.next_block;
        }
        num_blocks
    }

    // Helper function to check that every block in the list satisfies some condition
    // Returns false if the condition returns false for any node
    pub fn each_free_block_satisfies<F>(&self, condition: F) -> bool
        where F: Fn(&free_list::BlockHeader) -> bool {

        let mut current = self.free_list.head;
        while let Some(curr) = current.get_ref() {
            if !condition(curr) {
                return false;
            }
            current = curr.next_block;
        }
        true
    }
}

// Gets an initialized free list with the requested memory for testing purposes
pub fn get_free_list_with_size(heap_size: usize) -> TestFreeList {
    TestFreeList::new(heap_size)
}

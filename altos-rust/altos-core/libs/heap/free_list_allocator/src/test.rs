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

use std::vec::Vec;

use free_list;

// TestMemory is a helper type which represents a chunk of memory used in testing the allocator
// It uses a vector to create the necessary memory and manages deallocation of the vector
pub struct TestMemory {
    heap: *mut u8,
    vector_length: usize,
    vector_capacity: usize,
}

impl TestMemory {
    pub fn new(heap_size: usize) -> TestMemory {
        let mut heap: Vec<u8> = vec![0; heap_size];
        let heap_start = &mut heap[0] as *mut u8;
        let heap_capacity = heap.capacity();
        ::core::mem::forget(heap);
        TestMemory {
            heap: heap_start,
            vector_length: heap_size,
            vector_capacity: heap_capacity,
        }
    }

    pub fn get_heap(&self) -> *mut u8 {
        self.heap
    }

    fn get_memory_size(&self) -> usize {
        self.vector_length
    }
}

impl Drop for TestMemory {
    fn drop(&mut self) {
        unsafe {
            // Reform the vector from component parts in order to make sure it's deallocated
            let heap = Vec::from_raw_parts(self.heap, self.vector_length, self.vector_capacity);
            drop(heap);
        }
    }
}

// TestFreeList is a helper type for free lists, which utilizes TestMemory
pub struct TestFreeList {
    pub free_list: free_list::FreeList,
    test_memory: TestMemory,
}

impl TestFreeList {
    fn new(test_memory: TestMemory) -> TestFreeList {
        let mut free_list = free_list::FreeList::new();
        free_list.init(test_memory.get_heap() as usize, test_memory.get_memory_size());
        TestFreeList {
            free_list: free_list,
            test_memory: test_memory,
        }
    }

    pub fn get_heap_start(&self) -> *mut u8 {
        self.test_memory.get_heap()
    }

    pub fn get_free_list(&mut self) -> &mut free_list::FreeList {
        &mut self.free_list
    }
}

// Returns a TestMemory with the given size
pub fn get_memory(heap_size: usize) -> TestMemory {
    TestMemory::new(heap_size)
}

// Gets an initialized free list with the requested memory for testing purposes
pub fn get_free_list_with_size(heap_size: usize) -> TestFreeList {
    let heap = get_memory(heap_size);
    TestFreeList::new(heap)
}

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
 * The free list allocator uses a linked list to keep track of blocks of free memory, allowing
 * for more efficient use of memory than the bump allocator. This allocator reclaims memory
 * on deallocations and allocates memory using the first fit strategy.
 */

#![feature(allocator)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(cfg_target_has_atomic)]
#![cfg_attr(not(test), allocator)]
#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(all(target_arch="arm", not(target_has_atomic="ptr")))]
extern crate cm0_atomic as atomic;

//#[cfg(all(target_arch="arm", not(target_has_atomic="ptr")))]
extern crate cm0_sync as sync;

#[cfg(target_has_atomic="ptr")]
use core::sync::atomic as atomic;

// Not sure if we need to be using this or not
//use atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

mod free_list;
mod alignment;

static mut FL_ALLOCATOR : FreeListAllocator = FreeListAllocator::new();
static SYNC_FL_ALLOCATOR : sync::spin::SpinMutex<FreeListAllocator> =
    sync::spin::SpinMutex::new(FreeListAllocator::new());

/// Initializes the free list with the given heap memory starting position and size
/// Call this before doing any heap allocation. This MUST only be called once
pub fn init_heap(heap_start: usize, heap_size: usize) {
    unsafe { FL_ALLOCATOR.init(heap_start, heap_size) };
}

pub struct FreeListAllocator {
    heap_list: free_list::FreeList,
}

impl FreeListAllocator {
    /// Creates a new free list allocator
    pub const fn new() -> Self {
        FreeListAllocator {
            heap_list: free_list::FreeList::new(),
        }
    }

    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        // List starts with a single block containing all the memory
        self.heap_list.init(heap_start, heap_size);
    }

    /// Allocates a block of memory with the given size and alignment.
    #[inline(never)]
    pub fn allocate(&mut self, size: usize, align: usize) -> *mut u8 {
        // Should we be using an option here?
        let alloc_ptr = self.heap_list.allocate(size, align);
        // Is this the right place for checking if we're out of memory?
        if alloc_ptr.is_null() {
            panic!("Out of memory")
        }
        alloc_ptr
    }

    /// Deallocates a block of memory with the given size and alignment.
    #[inline(never)]
    pub fn deallocate(&mut self, alloc_ptr: *mut u8, size: usize, align: usize) {
        // Do we even care about alignment variable?
        self.heap_list.deallocate(alloc_ptr, size);
    }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    unsafe {
        FL_ALLOCATOR.allocate(size, align)
    }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {
    unsafe {
        FL_ALLOCATOR.deallocate(_ptr, _size, _align)
    }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    // TODO: This actually needs to return result from minimum block alignment or value of _align
    // So if minimal block size is 16, align is 32, and size is 5, usable size is 32
    unsafe {
        alignment::use_size(size, FL_ALLOCATOR.heap_list.get_block_hdr_size())
    }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, size: usize, _new_size: usize, _align: usize) -> usize {
    // TODO: This could search the list and try to expand to _new_size
    size
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize, align: usize) -> *mut u8 {
    // TODO: Could call __rust_reallocate_inplace first before doing a normal reallocation
    use core::{ptr, cmp};

    let new_ptr = __rust_allocate(new_size, align);
    // I was having issues with ptr::copy earlier. Should we use that here?
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
    __rust_deallocate(ptr, size, align);
    new_ptr
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::vec::Vec;
    use core::mem::{size_of, align_of};
    use free_list::BlockHeader;

    const HEAP_SIZE: usize = 2048;

    // TODO: Implement more tests for this
    // What tests should go in here as opposed to free_list.rs?

    /*
    Test
    __rust_allocate
    __rust_deallocate
    __rust_usable_size
    __rust_reallocate_inplace
    __rust_reallocate
    */

    #[test]
    #[should_panic]
    fn basic_allocation() {
        let mut heap: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
        let heap_start = &heap[0] as *const u8;
        let mut allocator = FreeListAllocator::new();
        allocator.init(heap_start as usize, HEAP_SIZE);

        assert!(!allocator.allocate(512, 1).is_null());
        assert!(!allocator.allocate(512, 1).is_null());
        assert!(!allocator.allocate(512, 2).is_null());
        
        allocator.allocate(1024, 1); // should panic
    }

    /*
    #[test]
    #[should_panic]
    fn alignment() {
        let mut heap: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
        let mut allocator = FreeListAllocator::new();
        let heap_start = &heap[0] as *const u8;
        let mut allocator = FreeListAllocator::new();
        allocator.init(heap_start as usize, HEAP_SIZE);

        assert!(allocator.allocate(512, 2).is_some());
        // should panic
        assert!(allocator.allocate(512, align_of::<BlockHeader>()).is_some());
        assert!(allocator.allocate(512, 3).is_none());
    }
    */
}

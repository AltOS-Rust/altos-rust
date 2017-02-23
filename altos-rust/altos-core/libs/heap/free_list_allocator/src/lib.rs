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

extern crate cm0_sync as sync;

use sync::spin::SpinMutex;
use free_list::FreeList;

mod free_list;
mod alignment;

#[cfg(test)]
mod test;

static FL_ALLOCATOR : SpinMutex<FreeList> =
    SpinMutex::new(FreeList::new());


/// Initializes the free list with the given heap memory starting position and size
/// Call this before doing any heap allocation. This MUST only be called once
pub fn init_heap(heap_start: usize, heap_size: usize) {
    let mut guard = FL_ALLOCATOR.lock();
    guard.init(heap_start, heap_size);

}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    let mut guard = FL_ALLOCATOR.lock();
    guard.allocate(size, align)
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {
    // This ignores align currently
    // TODO: Deal with align

    let mut guard = FL_ALLOCATOR.lock();
    guard.deallocate(_ptr, _size, _align)
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    // TODO: This actually needs to return result from minimum block alignment or value of _align
    // So if minimal block size is 16, align is 32, and size is 5, usable size is 32
    let guard = FL_ALLOCATOR.lock();
    alignment::align_up(size, guard.get_block_hdr_size())
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
    /*
    use super::*;
    use std::sync::Arc;
    use std::vec::Vec;
    use core::mem::{size_of, align_of};
    use free_list::BlockHeader;
    use test;
    */

    // fn _get_test_fl_allocator_with_size(heap_size: usize) -> FreeListAllocator {
    //     FreeListAllocator {
    //         heap_list: test::get_free_list_with_size(heap_size),
    //     }
    // }

    // TODO: Implement more tests for this

    /*
    Test
        __rust_allocate
        __rust_deallocate
        __rust_usable_size
        __rust_reallocate_inplace
        __rust_reallocate
    */
}

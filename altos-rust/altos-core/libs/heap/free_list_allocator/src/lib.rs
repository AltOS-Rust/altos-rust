#![feature(allocator)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(cfg_target_has_atomic)]

#![cfg_attr(not(test), allocator)]
#![no_std]

#[cfg(test)]
extern crate std;

#[cfg(all(target_arch="arm", not(target_has_atomic="ptr")))]
extern crate cm0_atomic as atomic;

#[cfg(target_has_atomic="ptr")]
use core::sync::atomic as atomic;
use atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

mod free_list;

static mut FL_ALLOCATOR : FreeListAllocator = FreeListAllocator::new();

/// Call this before doing any heap allocation. This MUST only be called once
pub fn init_heap(heap_start: usize, heap_size: usize) {
  unsafe { FL_ALLOCATOR.init(heap_start, heap_size) };
}

pub struct FreeListAllocator {
  heap_start: usize,
  heap_size: usize,
  heap_list: free_list::LinkedList,
}

impl FreeListAllocator {
  /// Creates a new free list allocator
  pub const fn new() -> Self {
    FreeListAllocator {
      heap_start: 0,
      heap_size: 0,
      heap_list: free_list::LinkedList::new(),
    }
  }

  pub fn init(&mut self, heap_start: usize, heap_size: usize) {
    self.heap_start = heap_start;
    self.heap_size = heap_size;
    // Should initially populate list with single node containing all memory
    self.heap_list.init(heap_start, heap_size);
  }

  /// Allocates a block of memory with the given size and alignment.
  #[inline(never)]
  pub fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
    Some(self.heap_list.allocate(size))
  }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
  unsafe {
    FL_ALLOCATOR.allocate(size, align).expect("out of memory")
  }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {
  // TODO: This should actually be implemented
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
  // This must be at least size, but if we're giving it more memory, we can return that
  size
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, size: usize, _new_size: usize, _align: usize) -> usize {
  // Tries to expand the size in place, but returns old size if it can't
  size
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize, align: usize) -> *mut u8 {
  // If we can expand the space without moving, we do
  // Otherwise move to a new location
  use core::{ptr, cmp};

  let new_ptr = __rust_allocate(new_size, align);
  unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
  __rust_deallocate(ptr, size, align);
  new_ptr
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::Arc;
  use std::vec::Vec;

  // TODO: Implement tests for this
}

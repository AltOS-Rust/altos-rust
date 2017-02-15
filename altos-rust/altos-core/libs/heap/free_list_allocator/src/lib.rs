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
  heap_list: free_list::FreeList,
}

impl FreeListAllocator {
  /// Creates a new free list allocator
  pub const fn new() -> Self {
    FreeListAllocator {
      heap_start: 0,
      heap_size: 0,
      heap_list: free_list::FreeList::new(),
    }
  }

  pub fn init(&mut self, heap_start: usize, heap_size: usize) {
    self.heap_start = heap_start;
    self.heap_size = heap_size;
    // Should initially populate list with single block containing all memory
    self.heap_list.init(heap_start, heap_size);
  }

  /// Allocates a block of memory with the given size and alignment.
  #[inline(never)]
  pub fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
    // Do we even care about alignment variable?
    Some(self.heap_list.allocate(size))
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
    FL_ALLOCATOR.allocate(size, align).expect("out of memory")
  }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {
  // TODO: This should actually be implemented
  unsafe {
    FL_ALLOCATOR.deallocate(_ptr, _size, _align)
  }
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
  // This must be at least size, but if we're giving it more memory, we can return that
  // We should return result of align_up with alignment equal to node size
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
  use core::mem::{size_of, align_of};
  use free_list::BlockHeader;

  const HEAP_SIZE: usize = 2048;


  #[test]
  #[should_panic]
  fn basic_allocation() {
    let mut heap: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    let heap_start = &heap[0] as *const u8;
    let mut allocator = FreeListAllocator::new();
    let mut allocator = FreeListAllocator::new();
    allocator.init(heap_start as usize, HEAP_SIZE);

    assert!(allocator.allocate(512, 1).is_some());
    assert!(allocator.allocate(512, 1).is_some());
    assert!(allocator.allocate(512, 2).is_some());
    // should_panic
    assert!(allocator.allocate(1024, 1).is_none());
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
  // TODO: Implement tests for this
}

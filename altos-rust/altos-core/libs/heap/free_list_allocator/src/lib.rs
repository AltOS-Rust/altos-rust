// Leaving this stuff the same as in bump allocator
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
// Keeping this above stuff the same for now

mod dlinkedlist;

/*
- Need to put together doubly linked list data structure
- Figure out how it's going to work with allocating nodes in memory allocator
*/

// Changing this stuff ->
static mut FL_ALLOCATOR : FreeListAllocator = FreeListAllocator::new();

/// Call this before doing any heap allocation. This MUST only be called once
pub fn init_heap(heap_start: usize, heap_size: usize) {
  unsafe { FL_ALLOCATOR.init(heap_start, heap_size) };
}

pub struct FreeListAllocator {
  // Need ref to start of list (Head)
  // Keep track of total size?
  // TODO: Fill this in...
  heap_start: usize,
  heap_size: usize,
  list_head: dlinkedlist::LinkedList,
}

impl FreeListAllocator {
  /// Creates a new free list allocator
  pub const fn new() -> Self {
    FreeListAllocator {
      // TODO: Fill this information in
      heap_start: 0,
      heap_size: 0,
      list_head: dlinkedlist::LinkedList::new(),
    }
  }

  pub fn init(&mut self, heap_start: usize, heap_size: usize) {
    // TODO: Create initial list
    self.heap_start = heap_start;
    self.heap_size = heap_size;
    // Should initially populate list with single node containing all memory
    self.list_head.add(heap_start as *mut u8, heap_size);
  }

  /// Allocates a block of memory with the given size and alignment.
  #[inline(never)]
  pub fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
    Some(self.list_head.allocate(size))
    // TODO: Fill in allocate fn
    // loop {
    //   let old_next = self.next.load(Ordering::SeqCst);
    //   let alloc_start = align_up(old_next, align);
    //   let alloc_end = alloc_start.saturating_add(size);
    //
    //   if alloc_end <= self.heap_start + self.heap_size {
    //     if self.next.compare_and_swap(old_next, alloc_end, Ordering::SeqCst) == old_next {
    //       return Some(alloc_start as *mut u8)
    //     }
    //   }
    //   else {
    //     return None
    //   }
    // }
  }
}

/// Align downwards. Returns the greatest x with alignment `align` so that x <= addr.
/// The alignment must be a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
  // TODO: Any adjustments to make to this or align_up?
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
  size
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, size: usize, _new_size: usize, _align: usize) -> usize {
  // TODO: If next chunk is free with enough size, allow it to be taken?
  size
}

#[no_mangle]
#[cfg(not(test))]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize, align: usize) -> *mut u8 {
  // TODO: Does this need to change at all?
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

  // TODO: Need new tests and adjust these tests as necessary

  #[test]
  fn test_alloc_smoke() {
    let mut allocator = FreeListAllocator::new();
    allocator.init(0, 10 * 1024 * 1024);
    assert!(allocator.allocate(1024, 1).is_some());
    assert!(allocator.allocate(1024, 1).is_some());
    assert!(allocator.allocate(1024 * 1024, 1).is_some());
    // assert_eq!(allocator.next.load(Ordering::Relaxed), 1024 * 1024 + 2048);
  }

  // #[test]
  // fn test_thread_safety() {
  //   let mut allocator = FreeListAllocator::new();
  //   let mut handles = Vec::with_capacity(10);
  //   allocator.init(0, 10 * 1024 * 1024);
  //   let alloc_arc = Arc::new(allocator);
  //   for _ in 0..10 {
  //     let alloc = alloc_arc.clone();
  //     handles.push(std::thread::spawn(move|| {
  //       for _ in 0..1000 {
  //         alloc.allocate(1024, 1);
  //       }
  //     }));
  //   }
  //   for handle in handles {
  //     handle.join().unwrap();
  //   }
  //   assert_eq!(alloc_arc.next.load(Ordering::Relaxed), 10 * 1000 * 1024);
  // }

  #[test]
  fn test_oom() {
    let mut allocator = FreeListAllocator::new();
    allocator.init(0, 1024);
    assert!(allocator.allocate(1024, 1).is_some());
    assert!(allocator.allocate(1024, 1).is_none());
  }
}

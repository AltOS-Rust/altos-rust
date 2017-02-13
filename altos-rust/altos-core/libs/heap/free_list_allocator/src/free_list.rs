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

// Not sure about naming
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
  pub fn allocate(&mut self, needed_size: usize) -> *mut u8 {
    let mut alloc_location: *mut u8 = ptr::null_mut();
    let using_size = use_size(needed_size);
    unsafe {
      let (mut previous, mut current) = (self.head, self.head);
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

      let (mut previous, mut current) = (self.head, self.head);
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

fn use_size(needed_size: usize) -> usize {
  // We always need to align up to block size or we end up with with potential leaks
  align_up(needed_size, mem::size_of::<BlockHeader>())
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

  // TODO: Figure out what to do for testing this stuff
  // Can we claim an arbitrary amount of memory to use for testing?

  #[test]
  fn test_empty_ll() {
    let new_ll = FreeList::new();
    assert!(new_ll.head.is_null());
  }
}

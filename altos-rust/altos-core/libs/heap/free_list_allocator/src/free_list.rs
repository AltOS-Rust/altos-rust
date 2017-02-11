// Linked list code for memory allocator
// This is intended for use by the free_list_allocator functionality

use core::{mem, ptr};

// Should not need to call function every time
// static mut MIN_ALLOC_SIZE : usize = 8;

// TODO: Add option to next
#[repr(C)]
pub struct Node {
  data: usize,
  next: *mut Node,
}

impl Node {
  const fn new(node_data: usize) -> Self {
    Node {
      data: node_data,
      next: ptr::null_mut(),
    }
  }
}

// Not sure about naming
pub struct LinkedList {
  head: *mut Node,
}

impl LinkedList {
  pub const fn new() -> Self {
    LinkedList {
      head: ptr::null_mut(),
    }
  }

  pub fn init(&mut self, heap_start: usize, heap_size: usize) {
    let node_position = heap_start as *mut Node;
    unsafe {
      ptr::write(&mut *node_position, Node::new(heap_size));
    }
    self.head = node_position;
  }

  // For some reason, this isn't working
  fn relocate_node(&self, current_pos: *mut Node, offset_val: usize) -> *mut Node {
    unsafe {
      let new_pos = current_pos.offset(offset_val as isize);
      // For some reason, just using copy here causes a compiler error
      ptr::copy_nonoverlapping(current_pos, new_pos, mem::size_of::<Node>());
      new_pos as *mut Node
    }
  }

  // Allocate memory using first fit strategy
  pub fn allocate(&mut self, needed_size: usize) -> *mut u8 {
    let mut alloc_location: *mut u8 = ptr::null_mut();
    unsafe {
      let (mut previous, mut current) = (self.head, self.head);
      while !current.is_null() {
        let current_size = (*current).data;
        let using_size = use_size(needed_size);
        // let remaining_size = current_size - using_size;
        // Due to alignment, we should never get a case
        // where 0 < remaining_size < node_size

        // Node does not have enough space to satisfy requirement
        if current_size < using_size {
          previous = current;
          // If current is null, this will not work yo!
          current = (*previous).next;
          continue;
        }
        // There is no node space remaining
        else if current_size == using_size {
          // If at head, there is no previous to adjust
          if self.head == current {
            self.head = (*self.head).next;
          }
          else {
            (*previous).next = (*current).next;
          }
          alloc_location = current as *mut u8;
          break;
        }
        // Node has enough space and a node can be maintained
        else {
          (*current).data -= using_size;
          (*previous).next = self.relocate_node(current, using_size);
        }
      }
    }

    if alloc_location.is_null() {
      panic!("Out of memory.");
    }
    alloc_location
  }

  fn deallocate() {
    /*
    Cases:
    - Nothing adjacent: Make new node, connect to closest nodes
    - Adjacent at tail: Merge with tail node, move node, switch leading ptr
    - Adjacent at lead: Merge with lead, no additional changes
    - Adjacent at both: Merge two with lead (add sizes, switch lead ptr)

    - Simplified version: Always create new node, insert at proper location
    */
  }
  // fn reallocate_inplace() {}
  // fn reallocate() {}
}

fn use_size(needed_size: usize) -> usize {
  // We always need to align up to node size or we end up with with potential leaks
  align_up(needed_size, mem::size_of::<Node>())
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
  // Can we mock memory by claiming big chunk arbitrarily?

  #[test]
  fn test_empty_ll() {
    let new_ll = LinkedList::new();
    assert!(new_ll.head.is_null());
  }
}

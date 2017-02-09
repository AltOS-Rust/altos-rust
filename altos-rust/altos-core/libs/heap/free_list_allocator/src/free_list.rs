// Linked list code for memory allocator
// This is intended for use by the free_list_allocator functionality

use core::mem;
use core::ptr;

// Figure out how to make this work
//static mut MIN_ALLOC_SIZE : usize = 8;

// Why is repr(C) here?
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
    // Need to check for minimum size
    let node_position = heap_start as *mut Node;
    unsafe {
      ptr::write(&mut *node_position, Node::new(heap_size));
    }
    self.head = node_position;
  }

  // Currently just having this work like a bump allocator
  pub fn allocate(&mut self, needed_size: usize) -> *mut u8 {
    let node_size = mem::size_of::<Node>();
    // Need to force it to get at least enough memory for a node
    let using_size = align_up(needed_size, node_size);
    unsafe {
      // Need to also make sure we have enough space for node
      // Remove it if we don't
      if using_size > (*self.head).data {
        panic!("No more memory!");
      }
      (*self.head).data -= using_size;
    }
    // Need to copy node to new starting location
    // Allocate node at other end of free mem?
    let current_pos = self.head as *mut u8;
    unsafe {
      let new_pos = current_pos.offset(using_size as isize);
      // Shouldn't need to call size_of every time probably
      ptr::copy(current_pos, new_pos, node_size);
      self.head = new_pos as *mut Node;
    }
    current_pos
  }

  fn deallocate() {}
  fn reallocate_inplace() {}
  // fn reallocate() {}
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_ll() {
    let new_ll = LinkedList::new();
    assert!(new_ll.head.is_null());
  }

  // #[test]
  // fn test_add_to_ll() {
  //   let mut new_ll = LinkedList::new();
  //   //let test_num : usize = 3;
  //   new_ll.add(1);
  //   new_ll.add(2);
  //   //new_ll.add(test_num);
  //   assert!(!new_ll.head.is_null());
  //
  //   unsafe { assert_eq!((*new_ll.head).data, 2) };
  // }
  //
  // #[test]
  // fn remove_from_ll() {
  //   let mut new_ll = LinkedList::new();
  //   new_ll.add(1);
  //   new_ll.add(2);
  //   new_ll.add(3);
  //   new_ll.add(4);
  //   new_ll.remove();
  //   assert!(!new_ll.head.is_null());
  //
  //   // Does this just fail if it's missing?
  //   //let dll_head = new_dll.head.expect("Doubly Linked List missing head!");
  //   unsafe { assert_eq!((*new_ll.head).data, 3) };
  // }
}

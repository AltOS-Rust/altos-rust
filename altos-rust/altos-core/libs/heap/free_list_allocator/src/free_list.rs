// Linked list code for memory allocator
// This is intended for use by the free_list_allocator functionality

use core::mem;
use core::ptr;

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

  // This currently functions just like the bump allocator
  pub fn allocate(&mut self, needed_size: usize) -> *mut u8 {
    let node_size = mem::size_of::<Node>();
    let using_size = use_size(needed_size);
    unsafe {
      // Need to also make sure we have enough space for node
      // Remove node if we don't
      if using_size > (*self.head).data {
        panic!("No more memory!");
      }
      (*self.head).data -= using_size;
    }
    // Need to copy node to new starting location
    // Can we get around this by allocating node at other end of free mem?
    let current_pos = self.head as *mut u8;
    unsafe {
      let new_pos = current_pos.offset(using_size as isize);
      // Shouldn't need to call size_of every time
      ptr::copy(current_pos, new_pos, node_size);
      self.head = new_pos as *mut Node;
    }
    current_pos
  }

  /// Allocate memory using first fit strategy
  // pub fn ff_allocate(&mut self, needed_size: usize) {
  //   // Traverse until we find big enough node
  //   // If we get to end without finding node, panic!
  //   unsafe {
  //     let head_size = (*self.head).data;
  //     let using_size = use_size(needed_size);
  //     let remaining_size = head_size - using_size;
  //     match remaining_size {
  //       rs < 0 => ,
  //       rs < mem::size_of::<Node>() => ,
  //       // Greater than or equal to node size
  //       _ => ,
  //       // Less than zero -> Not enough, move to next node
  //       // Less than node size -> Not enough for node, give back all
  //       // Node size or greater -> Enough for new node
  //     }
  //   }
  //   /*
  //   Cases:
  //   - Node is entirely consumed (Destroy and move to next)
  //     - At head: move head to next
  //     - In middle: Take previous and connect to next
  //   - Node has enough space
  //     - At head: move head forward
  //     - In middle: Allocate the space, adjust the node, change leading node ptr
  //   */
  // }

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

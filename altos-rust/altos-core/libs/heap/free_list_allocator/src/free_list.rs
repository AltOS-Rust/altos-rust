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
        // Due to alignment, we should never get a case
        // where 0 < remaining_size < node_size

        // Node does not have enough space to satisfy requirement
        if current_size < using_size {
          previous = current;
          // If current is null, this will not work!
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
          break;
        }
      }
    }

    if alloc_location.is_null() {
      panic!("Out of memory.");
    }
    alloc_location
  }

  pub fn deallocate(&mut self, alloc_ptr: *mut u8, size: usize) {
    unsafe {
      // We can immediately add the node at the deallocated position
      let alloc_node_ptr = alloc_ptr as *mut Node;
      let used_memory = use_size(size);
      ptr::write(&mut *alloc_node_ptr, Node::new(used_memory));

      let (mut previous, mut current) = (self.head, self.head);
      // Memory location to be added at head of the list
      if alloc_node_ptr < current {
        (*alloc_node_ptr).next = current;
        self.head = alloc_node_ptr;
        return;
      }
      while !current.is_null() {
        // Will this comparison work the way I expect? Comparing pointer positions.
        if previous < alloc_node_ptr && alloc_node_ptr < current {
          (*previous).next = alloc_node_ptr;
          (*alloc_node_ptr).next = current;
          return;
        }
        previous = current;
        current = (*previous).next;
      }
      // At this point, we know that it needs to be added at the end
      (*previous).next = alloc_node_ptr;
    }
    /*
    TODO: We can still implement merging to deal with fragmentation
    - Nothing adjacent: Make new node, connect to closest nodes
    - Adjacent at tail: Merge with tail node, move node, switch leading ptr
    - Adjacent at lead: Merge with lead, no additional changes
    - Adjacent at both: Merge two with lead (add sizes, switch lead ptr)
    */
  }
  // fn traverse() {}
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
  // Can we claim an arbitrary amount of memory to use for testing?

  #[test]
  fn test_empty_ll() {
    let new_ll = LinkedList::new();
    assert!(new_ll.head.is_null());
  }
}

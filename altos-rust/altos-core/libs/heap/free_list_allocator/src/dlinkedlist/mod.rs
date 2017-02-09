// Experimenting with doubly linked list code
// This is intended for use by the free_list_allocator functionality

// TODO: File should probably be relocated but I'm not sure where yet

// Change: Make it a regular linked list, have add/remove functions
// How to create node at specific passed in memory position

use core::mem;
use core::ptr;

// Why is repr(C) here?
#[repr(C)]
pub struct Node {
  data: usize,
  next: *mut Node,
}

impl Node {
  fn new(position: *mut u8, node_data: usize) -> *mut Node {
    let node_position = position as *mut Node;
    unsafe {
      ptr::write(node_position, Node {
        data: node_data,
        next: ptr::null_mut(),
      });
    }
    node_position
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

  // Always add at head
  pub fn add(&mut self, position: *mut u8, node_data: usize) {
    if !self.head.is_null() {
      let new_node = Node::new(position, node_data);
      unsafe {
        (*new_node).next = self.head;
      }
      self.head = new_node;
    }
    else {
      self.head = Node::new(position, node_data);
    }
  }

  pub fn allocate(&mut self, needed_size: usize) -> *mut u8 {
    // Needs alignment as well
    // Not doing any checking yet for enough memory
    unsafe {
      (*self.head).data -= needed_size;
    }
    // Need to copy node to new starting location
    // Allocate node at other end of free mem?
    let current_pos = self.head as *mut u8;
    unsafe {
      let new_pos = current_pos.offset(needed_size as isize);
      // Shouldn't need to call size_of every time probably
      ptr::copy(current_pos, new_pos, mem::size_of::<Node>());
    }
    current_pos
  }

  // Currently just removes first element
  fn remove(&mut self) {
    if !self.head.is_null() {
      unsafe {
        let current_head = self.head;
        self.head = (*self.head).next;
        drop(current_head);
      }
    }
    else {
      panic!("Trying to remove from empty list");
    }
  }

  // Add and remove anywhere in list
  // Find memory amount from node
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

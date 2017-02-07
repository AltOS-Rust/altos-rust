// Experimenting with doubly linked list code
// This is intended for use by the free_list_allocator functionality

// TODO: File should probably be relocated but I'm not sure where yet

use core::mem;
use core::ptr;

// Why is repr(C) here?
#[repr(C)]
pub struct Node<T> {
  data: T,
  next: *mut Node<T>,
  prev: *mut Node<T>,
}

impl<T> Node<T> {
  const fn new(node_data: T) -> Self {
    Node {
      data: node_data,
      next: ptr::null_mut(),
      prev: ptr::null_mut(),
    }
  }
}

// Not sure about naming
pub struct DoublyLinkedList<T> {
  head: *mut Node<T>,
}

impl<T> DoublyLinkedList<T> {
  const fn new() -> Self {
    DoublyLinkedList {
      head: ptr::null_mut(),
    }
  }

  // Add to the doubly linked list at head
  fn add(&mut self, node_data: T) {
    if !self.head.is_null() {
      let new_node: *mut Node<T> = &mut Node::new(node_data);
      unsafe {
        //println!("setting next and prev");
        (*new_node).next = self.head;
        (*self.head).prev = new_node;
      }
      self.head = new_node;
    }
    else {
      self.head = &mut Node::new(node_data);
    }
  }

  // Start with remove first element
  fn remove(&mut self) {
    if !self.head.is_null() {
      unsafe {
        let next_node = (*self.head).next;
        //drop(self.head);
        self.head = next_node;
        (*self.head).prev = ptr::null_mut();
        //println!("Head data: {}", (*self.head).data);
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
  fn test_empty_dll() {
    let new_dll = DoublyLinkedList::<i32>::new();
    assert!(new_dll.head.is_null());
  }

  #[test]
  fn test_add_to_dll() {
    let mut new_dll = DoublyLinkedList::<i32>::new();
    new_dll.add(1);
    new_dll.add(2);
    new_dll.add(3);
    //new_dll.add(4);
    assert!(!new_dll.head.is_null());

    // Does this just fail if it's missing?
    //let dll_head = new_dll.head.expect("Doubly Linked List missing head!");
    unsafe { assert_eq!((*new_dll.head).data, 3) };
  }

  #[test]
  fn remove_from_dll() {
    let mut new_dll = DoublyLinkedList::<i32>::new();
    new_dll.add(1);
    new_dll.add(2);
    new_dll.add(3);
    new_dll.add(4);
    new_dll.remove();
    assert!(!new_dll.head.is_null());

    // Does this just fail if it's missing?
    //let dll_head = new_dll.head.expect("Doubly Linked List missing head!");
    unsafe { assert_eq!((*new_dll.head).data, 3) };
  }

}

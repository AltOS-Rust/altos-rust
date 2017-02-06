// Experimenting with doubly linked list code
// This is intended for use by the free_list_allocator functionality

// TODO: File should probably be relocated but I'm not sure where yet

use core::mem;
use core::ptr;

// Why is repr(C) here?
// #[repr(C)]
pub struct Node<T> {
  data: T,
  next: Option<*mut Node<T>>,
  prev: Option<*mut Node<T>>,
}

impl<T> Node<T> {
  const fn new(node_data: T) -> Self {
    Node {
      data: node_data,
      next: None,
      prev: None,
    }
  }
}

// Not sure about naming
pub struct DoublyLinkedList<T> {
  head: Option<*mut Node<T>>,
}

impl<T> DoublyLinkedList<T> {
  const fn new() -> Self {
    DoublyLinkedList {
      head: None,
    }
  }

  // Add to the doubly linked list at head
  fn add(&mut self, node_data: T) {
    match self.head {
      Some(head) => {
        println!("in Some(head)");
        let new_node: *mut Node<T> = &mut Node::new(node_data);
        unsafe {
          //println!("setting next and prev");
          (*new_node).next = Some(head);
          (*head).prev = Some(new_node);
        }
        self.head = Some(new_node)
      },
      None => {
        self.head = Some(&mut Node::new(node_data))
      }
    }
  }

  // Start with remove first element
  fn remove(&mut self) {
    match self.head {
      Some(head) => {
        let current_head = head;
        unsafe { self.head = (*current_head).next };
        match self.head {
          Some(head) => {
            unsafe { (*head).prev = None };
          },
          // Don't care if empty
          None => ()
        }
        drop(current_head);
      },
      None => {
        // Not sure what to do here
        panic!("Trying to remove from empty list");
      }
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
    assert!(new_dll.head.is_none());
  }

  #[test]
  fn test_add_to_dll() {
    let mut new_dll = DoublyLinkedList::<i32>::new();
    new_dll.add(1);
    new_dll.add(2);
    //new_dll.add(3);
    //new_dll.add(4);
    assert!(new_dll.head.is_some());

    // Does this just fail if it's missing?
    let dll_head = new_dll.head.expect("Doubly Linked List missing head!");
    unsafe { assert_eq!((*dll_head).data, 2) };
  }

  #[test]
  fn remove_from_dll() {
    let mut new_dll = DoublyLinkedList::<i32>::new();
    new_dll.add(1);
    new_dll.add(2);
    new_dll.add(3);
    new_dll.add(4);
    new_dll.remove();
    assert!(new_dll.head.is_some());

    // Does this just fail if it's missing?
    let dll_head = new_dll.head.expect("Doubly Linked List missing head!");
    unsafe { assert_eq!((*dll_head).data, 3) };
  }

}

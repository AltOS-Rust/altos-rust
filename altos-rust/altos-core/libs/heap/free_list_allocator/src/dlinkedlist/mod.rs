// Experimenting with doubly linked list for free list allocator
use alloc::boxed::Box;

// Why is repr(C) here?
#[repr(C)]
pub struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>,
  prev: Option<Box<Node<T>>>,
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
  head: Option<Node<T>>,
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
        let mut new_node = Some(Node::new(node_data));
        let mut current_head = self.head;
        current_head.prev = new_node;
        new_node.next = Some(current_head);
        self.head = new_node;
      }
      None => {
        self.head = Node::new(node_data);
      }
    }
  }

  // Start with remove first
  fn remove(&mut self) {
    let mut current_head = self.head;
    self.head = self.head.next;
    self.head.prev = None;
    drop(current_head);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use alloc::boxed::Box;

  #[test]
  fn test_empty_dll() {
    //let a : Node<usize> = Node { data: 1, next: None, prev: None };
    //let b : Node<usize> = Node { data: 2, next: None, prev: None };
    let new_dll = DoublyLinkedList::new();
    assert_eq!(new_dll.head, None);
  }

  #[test]
  fn test_add_to_dll() {
    let new_dll = DoublyLinkedList::new();
    new_dll.add(1);
    new_dll.add(2);
    match new_dll.head {
      Some(head_node) => {
        assert_eq!(head_node.data, 2);
      }
      None => {
        assert!(false);
      }
    }
  }

  #[test]
  fn remove_from_dll() {
    let new_dll = DoublyLinkedList::new();
    new_dll.add(1);
    new_dll.add(2);
    new_dll.remove();
    match new_dll.head {
      Some(head_node) => {
        assert_eq!(head_node.data, 1);
      }
      None => {
        assert!(false);
      }
    }
  }
}

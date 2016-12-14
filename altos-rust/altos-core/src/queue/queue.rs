// queue/queue.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/2/16

//! A collection of items that can be accessed through a FIFO interface.

use alloc::boxed::Box;
use super::Node;

/// A collection that provides FIFO queue functionality.
pub struct Queue<T> {
  head: Option<Box<Node<T>>>,
  tail: *mut Node<T>,
}

impl<T> Queue<T> {
  /// Creates an empty `Queue`.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::Queue;
  ///
  /// let queue = Queue::<usize>::new();
  /// ```
  pub const fn new() -> Self {
    Queue { 
      head: None,
      tail: ::core::ptr::null_mut(),
    }
  }

  /// Places a new item onto the end of the queue.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  ///
  /// queue.enqueue(Box::new(Node::new(0)));
  /// ```
  pub fn enqueue(&mut self, elem: Box<Node<T>>) {
    let mut new_tail = elem;
    // Probably not necessary...
    new_tail.next = None;

    let raw_tail: *mut _ = &mut *new_tail;

    if !self.tail.is_null() {
      unsafe {
        (*self.tail).next = Some(new_tail);
      }
    }
    else {
      self.head = Some(new_tail);
    }

    self.tail = raw_tail;
  }

  /// Takes an item off of the front of the queue, if there are no items in the queue returns None.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  ///
  /// queue.enqueue(Box::new(Node::new(0)));
  ///
  /// assert!(queue.dequeue().is_some());
  /// assert!(queue.dequeue().is_none());
  /// ```
  pub fn dequeue(&mut self) -> Option<Box<Node<T>>> {
    self.head.take().map(|mut head| {
      self.head = head.next.take();
      if self.head.is_none() {
        self.tail = ::core::ptr::null_mut();
      }

      head
    })
  }
  
  /// Removes all elements matching `predicate` and returns them in a new queue
  ///
  /// O(n) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  ///
  /// queue.enqueue(Box::new(Node::new(0)));
  /// queue.enqueue(Box::new(Node::new(1)));
  ///
  /// let removed = queue.remove(|n| *n == 0);
  ///
  /// assert!(!queue.is_empty());
  /// assert!(!removed.is_empty());
  /// ```
  pub fn remove<F: Fn(&T) -> bool>(&mut self, predicate: F) -> Queue<T> {
    let mut matching = Queue::new();
    let mut not_matching = Queue::new();

    while let Some(mut head) = self.head.take() {
      self.head = head.next.take();

      if predicate(&head) {
        matching.enqueue(head);
      }
      else {
        not_matching.enqueue(head);
      }
    }
    *self = not_matching;
    matching
  }
  
  /// Appends all the elements of `queue` onto `self`.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue1 = Queue::new();
  /// let mut queue2 = Queue::new();
  ///
  /// queue1.enqueue(Box::new(Node::new(0)));
  /// queue2.enqueue(Box::new(Node::new(1)));
  /// 
  /// queue1.append(queue2);
  ///
  /// assert!(!queue1.is_empty());
  /// ```
  pub fn append(&mut self, mut queue: Queue<T>) {
    if !self.tail.is_null() {
      unsafe {
        (*self.tail).next = queue.head.take();
      }
    }
    else {
      self.head = queue.head.take();
    }

    self.tail = queue.tail;
  }

  /// Modifies all the elements of the queue with the block passed in.
  ///
  /// O(n) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  ///
  /// queue.enqueue(Box::new(Node::new(0)));
  /// queue.enqueue(Box::new(Node::new(1)));
  /// 
  /// queue.modify_all(|n| *n = *n + 1);
  /// ```
  #[deprecated(since="0.1.0", note="Use `iter_mut()` instead")]
  pub fn modify_all<F: Fn(&mut T)>(&mut self, block: F) {
    let mut current = self.head.as_mut();
    while let Some(node) = current {
      block(&mut *node);
      current = node.next.as_mut();
    }
  }

  /// Removes all the elements from `self` and returns them in a new `Queue`.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  ///
  /// queue.enqueue(Box::new(Node::new(0)));
  /// queue.enqueue(Box::new(Node::new(1)));
  /// 
  /// let removed = queue.remove_all();
  ///
  /// assert!(queue.is_empty());
  /// assert!(!removed.is_empty());
  /// ```
  pub fn remove_all(&mut self) -> Queue<T> {
    ::core::mem::replace(self, Queue::new())
  }

  /// Checks if the queue is empty, returns true if it is empty, false otherwise.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::Queue;
  ///
  /// let queue = Queue::<usize>::new();
  ///
  /// assert!(queue.is_empty());
  /// ```
  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  /// Returns an iterator over the values of `self` consuming `self`.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  /// queue.enqueue(Box::new(Node::new(1)));
  /// queue.enqueue(Box::new(Node::new(2)));
  /// queue.enqueue(Box::new(Node::new(3)));
  ///
  /// let mut iter = queue.into_iter();
  /// assert_eq!(iter.next().map(|n| **n), Some(1));
  /// assert_eq!(iter.next().map(|n| **n), Some(2));
  /// assert_eq!(iter.next().map(|n| **n), Some(3));
  /// assert!(iter.next().is_none());
  /// ```
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  /// Returns an iterator over references to the values in `self`.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  /// queue.enqueue(Box::new(Node::new(1)));
  /// queue.enqueue(Box::new(Node::new(2)));
  /// queue.enqueue(Box::new(Node::new(3)));
  ///
  /// let mut iter = queue.iter();
  /// assert_eq!(iter.next(), Some(&1));
  /// assert_eq!(iter.next(), Some(&2));
  /// assert_eq!(iter.next(), Some(&3));
  /// assert!(iter.next().is_none());
  /// ```
  pub fn iter(&self) -> Iter<T> {
    Iter { next: self.head.as_ref().map(|node| &**node) }
  }

  /// Returns an iterator over mutable references to the values in `self`.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, Queue};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut queue = Queue::new();
  /// queue.enqueue(Box::new(Node::new(1)));
  /// queue.enqueue(Box::new(Node::new(2)));
  /// queue.enqueue(Box::new(Node::new(3)));
  ///
  /// let mut iter = queue.iter_mut();
  /// assert_eq!(iter.next(), Some(&mut 1));
  /// assert_eq!(iter.next(), Some(&mut 2));
  /// assert_eq!(iter.next(), Some(&mut 3));
  /// assert!(iter.next().is_none());
  /// ```
  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut { next: self.head.as_mut().map(|node| &mut **node) }
  }
}

impl<T> Drop for Queue<T> {
  fn drop(&mut self) {
    // Drop the queue in an iterative fashion to avoid recursive drop calls
    let mut current = self.head.take();
    while let Some(mut node) = current {
      current = node.next.take();
    }
    self.tail = ::core::ptr::null_mut();
  }
}

/// An iterator over `Queue` that consumes the values in the collection.
pub struct IntoIter<T>(Queue<T>);

impl<T> Iterator for IntoIter<T> {
  type Item = Box<Node<T>>;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.dequeue()
  }
}

/// An iterator over `Queue` that holds references to the values in the collection.
pub struct Iter<'a, T: 'a> {
  next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &**node);
      &node.data
    })
  }
}

/// An iterator over `Queue` that holds mutable references to the values in the collection.
pub struct IterMut<'a, T: 'a> {
  next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
  type Item = &'a mut T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.take().map(|node| {
      self.next = node.next.as_mut().map(|node| &mut **node);
      &mut node.data
    })
  }
}

#[cfg(test)]
mod tests {
  use super::Queue;
  use super::super::Node;
  use alloc::boxed::Box;

  #[test]
  fn empty_dequeue() {
    let mut list: Queue<Node<usize>> = Queue::new();

    assert!(list.dequeue().is_none());
  }

  #[test]
  fn smoke() {
    let mut list = Queue::new();

    // Populate list
    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));

    // Check normal removal
    assert_eq!(list.dequeue().map(|n| n.data), Some(1));
    assert_eq!(list.dequeue().map(|n| n.data), Some(2));

    // Push some more just to make sure nothing's corrupted
    list.enqueue(Box::new(Node::new(4)));
    list.enqueue(Box::new(Node::new(5)));

    // Check normal removal
    assert_eq!(list.dequeue().map(|n| n.data), Some(3));
    assert_eq!(list.dequeue().map(|n| n.data), Some(4));

    // Check exhaustion
    assert_eq!(list.dequeue().map(|n| n.data), Some(5));
    assert!(list.dequeue().is_none());

    // Check the exhaustion case fixed the pointer right
    list.enqueue(Box::new(Node::new(6)));
    list.enqueue(Box::new(Node::new(7)));

    // Check normal removal
    assert_eq!(list.dequeue().map(|n| n.data), Some(6));
    assert_eq!(list.dequeue().map(|n| n.data), Some(7));
    assert!(list.dequeue().is_none());
  }

  #[test]
  fn remove_predicate() {
    let mut list = Queue::new();

    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));
    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));

    let predicate = |data: &usize| *data == 1;

    let mut removed = list.remove(predicate);
    assert_eq!(removed.dequeue().map(|n| n.data), Some(1));
    assert_eq!(removed.dequeue().map(|n| n.data), Some(1));
    assert!(removed.dequeue().is_none());

    assert_eq!(list.dequeue().map(|n| n.data), Some(2));
    assert_eq!(list.dequeue().map(|n| n.data), Some(3));
    assert_eq!(list.dequeue().map(|n| n.data), Some(2));
    assert_eq!(list.dequeue().map(|n| n.data), Some(3));
    assert!(list.dequeue().is_none());
  }

  #[test]
  fn append_queue() {
    let mut list1 = Queue::new();
    let mut list2 = Queue::new();

    list1.enqueue(Box::new(Node::new(1)));
    list1.enqueue(Box::new(Node::new(2)));
    list2.enqueue(Box::new(Node::new(3)));
    list2.enqueue(Box::new(Node::new(4)));

    list1.append(list2);

    assert_eq!(list1.dequeue().map(|n| n.data), Some(1));
    assert_eq!(list1.dequeue().map(|n| n.data), Some(2));
    assert_eq!(list1.dequeue().map(|n| n.data), Some(3));
    assert_eq!(list1.dequeue().map(|n| n.data), Some(4));

    assert!(list1.dequeue().is_none());
  }

  #[test]
  fn append_empty() {
    let mut list1 = Queue::new();
    let list2 = Queue::new();

    list1.enqueue(Box::new(Node::new(1)));
    list1.enqueue(Box::new(Node::new(2)));

    list1.append(list2);

    assert_eq!(list1.dequeue().map(|n| n.data), Some(1));
    assert_eq!(list1.dequeue().map(|n| n.data), Some(2));

    assert!(list1.dequeue().is_none());
  }

  #[test]
  #[allow(deprecated)]
  fn modify_all() {
    let mut list = Queue::new();

    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));

    list.modify_all(|data: &mut usize| *data *= 10);

    assert_eq!(list.dequeue().map(|n| n.data), Some(10));
    assert_eq!(list.dequeue().map(|n| n.data), Some(20));
    assert_eq!(list.dequeue().map(|n| n.data), Some(30));

    assert!(list.dequeue().is_none());
  }

  #[test]
  fn remove_all() {
    let mut list = Queue::new();

    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));

    let mut old = list.remove_all();

    assert_eq!(old.dequeue().map(|n| n.data), Some(1));
    assert_eq!(old.dequeue().map(|n| n.data), Some(2));
    assert_eq!(old.dequeue().map(|n| n.data), Some(3));
    assert!(old.dequeue().is_none());

    assert!(list.dequeue().is_none());
  }

  #[test]
  fn is_empty() {
    let mut list = Queue::new();

    assert!(list.is_empty());

    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));

    assert!(!list.is_empty());

    assert_eq!(list.dequeue().map(|n| n.data), Some(1));
    assert!(!list.is_empty());
    assert_eq!(list.dequeue().map(|n| n.data), Some(2));
    assert!(list.is_empty());
    assert!(list.dequeue().is_none());
    assert!(list.is_empty());
  }

  #[test]
  fn into_iter() {
    let mut list = Queue::new();
    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));

    let mut iter = list.into_iter();
    assert_eq!(iter.next().map(|n| n.data), Some(1));
    assert_eq!(iter.next().map(|n| n.data), Some(2));
    assert_eq!(iter.next().map(|n| n.data), Some(3));
    assert!(iter.next().is_none());
  }

  #[test]
  fn iter() {
    let mut list = Queue::new();
    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));

    {
      let mut iter = list.iter();
      assert_eq!(iter.next(), Some(&1));
      assert_eq!(iter.next(), Some(&2));
      assert_eq!(iter.next(), Some(&3));
      assert!(iter.next().is_none());
    }

    assert_eq!(list.dequeue().map(|n| n.data), Some(1));
    assert_eq!(list.dequeue().map(|n| n.data), Some(2));
    assert_eq!(list.dequeue().map(|n| n.data), Some(3));
    assert!(list.dequeue().is_none());
  }

  #[test]
  fn iter_mut() {
    let mut list = Queue::new();
    list.enqueue(Box::new(Node::new(1)));
    list.enqueue(Box::new(Node::new(2)));
    list.enqueue(Box::new(Node::new(3)));

    {
      let mut iter = list.iter_mut();
      assert_eq!(iter.next(), Some(&mut 1));
      assert_eq!(iter.next(), Some(&mut 2));
      assert_eq!(iter.next(), Some(&mut 3));
      assert!(iter.next().is_none());
    }

    assert_eq!(list.dequeue().map(|n| n.data), Some(1));
    assert_eq!(list.dequeue().map(|n| n.data), Some(2));
    assert_eq!(list.dequeue().map(|n| n.data), Some(3));
    assert!(list.dequeue().is_none());
  }
}

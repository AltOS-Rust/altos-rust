// queue/sorted_list.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/4/16

//! Sorted collections
//!
//! These sorted collections provide a way to store data that must be sorted. If the extra utility
//! of having the collection sorted is not worth the overhead, think of using the unsorted
//! collections.

use super::Node;
use alloc::boxed::Box;

/// A list where every insertion is in sorted order.
///
/// The list will ensure that every item inserted into it goes in its proper place. This requires
/// that the generic type wrapped by the list is `PartialOrd` so the values can be compared.
pub struct SortedList<T: PartialOrd> {
  head: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> SortedList<T> {
  /// Creates an empty `SortedList`.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::SortedList;
  ///
  /// let list = SortedList::<usize>::new();
  /// ```
  pub const fn new() -> Self {
    SortedList {
      head: None,
    }
  }

  /// Places a new item onto the end of the queue.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut list = SortedList::new();
  ///
  /// list.insert(Box::new(Node::new(1)));
  /// list.insert(Box::new(Node::new(0)));
  /// ```
  pub fn insert(&mut self, mut elem: Box<Node<T>>) {
    if self.head.is_none() || **elem <= ***self.head.as_ref().unwrap() {
      elem.next = self.head.take();
      self.head = Some(elem);
      return;
    }
    let mut current = self.head.as_mut();
    while let Some(node) = current.take() {
      if node.next.is_none() || **elem <= ***node.next.as_ref().unwrap() {
        current = Some(node);
        break;
      }
      current = node.next.as_mut();
    }

    if let Some(node) = current.take() {
      elem.next = node.next.take();
      node.next = Some(elem);
    }
  }

  /// Takes an item off of the front of the list, if there are no items in the list returns None.
  ///
  /// This method returns the lowest value currently stored in the list.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut list = SortedList::new();
  ///
  /// list.insert(Box::new(Node::new(0)));
  ///
  /// assert_eq!(list.pop().map(|n| **n), Some(0));
  /// ```
  pub fn pop(&mut self) -> Option<Box<Node<T>>> {
    match self.head.take() {
      Some(mut head) => {
        self.head = head.next.take();
        Some(head)
      },
      None => None,
    }
  }

  /// Removes all elements matching `predicate` and returns them in a new list.
  ///
  /// O(n) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut list = SortedList::new();
  ///
  /// list.insert(Box::new(Node::new(0)));
  /// list.insert(Box::new(Node::new(1)));
  ///
  /// let removed = list.remove(|node| *node == 0);
  ///
  /// assert!(!list.is_empty());
  /// assert!(!removed.is_empty());
  /// ```
  pub fn remove<F: Fn(&T) -> bool>(&mut self, predicate: F) -> SortedList<T> {
    let mut matching = SortedList::new();
    let mut not_matching = SortedList::new();

    while let Some(mut head) = self.head.take() {
      self.head = head.next.take();

      if predicate(&head) {
        matching.insert(head);
      }
      else {
        not_matching.insert(head);
      }
    }
    *self = not_matching;
    matching
  }
  
  /// Inserts all the elements of `list` into `self` in the correct location.
  ///
  /// O(n) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  /// 
  /// let mut list1 = SortedList::new();
  /// let mut list2 = SortedList::new();
  /// 
  /// list1.insert(Box::new(Node::new(0)));
  /// list2.insert(Box::new(Node::new(1)));
  ///
  /// list1.merge(list2);
  /// ```
  pub fn merge(&mut self, list: SortedList<T>) {
    // TODO: Figure out a more efficient way to do this (the other list is in sorted order after
    // all...)
    for item in list.into_iter() {
      self.insert(item);
    }
  }

  /*
  /// Modify all the elements of the queue with the block passed in.
  /// O(n) algorithmic time
  pub fn modify_all<F: Fn(&mut T)>(&mut self, block: F) {
    let mut current = self.head.as_mut();
    while let Some(node) = current {
      block(&mut *node);
      current = node.next_mut();
    }
  }
  */

  /// Removes all the elements from `self` and returns it in a new `SortedList`.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut list = SortedList::new();
  /// 
  /// list.insert(Box::new(Node::new(0)));
  /// list.insert(Box::new(Node::new(1)));
  ///
  /// let removed = list.remove_all();
  ///
  /// assert!(list.is_empty());
  /// assert!(!removed.is_empty());
  /// ```
  pub fn remove_all(&mut self) -> SortedList<T> {
    ::core::mem::replace(self, SortedList::new())
  }

  /// Checks if the list is empty, returns true if it is, false otherwise.
  ///
  /// O(1) algorithmic time
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::SortedList;
  ///
  /// let list = SortedList::<usize>::new();
  ///
  /// // Returns true
  /// list.is_empty();
  /// ```
  pub fn is_empty(&self) -> bool {
    self.head.is_none()
  }

  /// Returns an iterator over the values of the list.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut list = SortedList::new();
  /// list.insert(Box::new(Node::new(1)));
  /// list.insert(Box::new(Node::new(2)));
  /// list.insert(Box::new(Node::new(3)));
  ///
  /// let mut iter = list.into_iter();
  /// assert_eq!(iter.next().map(|n| **n), Some(1));
  /// assert_eq!(iter.next().map(|n| **n), Some(2));
  /// assert_eq!(iter.next().map(|n| **n), Some(3));
  /// assert!(iter.next().is_none());
  /// ```
  pub fn into_iter(self) -> IntoIter<T> {
    IntoIter(self)
  }

  /// Returns an iterator of references over the list.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut list = SortedList::new();
  /// list.insert(Box::new(Node::new(1)));
  /// list.insert(Box::new(Node::new(2)));
  /// list.insert(Box::new(Node::new(3)));
  ///
  /// let mut iter = list.iter();
  /// assert_eq!(iter.next(), Some(&1));
  /// assert_eq!(iter.next(), Some(&2));
  /// assert_eq!(iter.next(), Some(&3));
  /// assert!(iter.next().is_none());
  /// ```
  pub fn iter(&self) -> Iter<T> {
    Iter { next: self.head.as_ref().map(|node| &**node) }
  }

  /// Returns an iterator of mutable references over the list.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::queue::{Node, SortedList};
  /// use altos_core::alloc::boxed::Box;
  ///
  /// let mut list = SortedList::new();
  /// list.insert(Box::new(Node::new(1)));
  /// list.insert(Box::new(Node::new(2)));
  /// list.insert(Box::new(Node::new(3)));
  ///
  /// let mut iter = list.iter_mut();
  /// assert_eq!(iter.next(), Some(&mut 1));
  /// assert_eq!(iter.next(), Some(&mut 2));
  /// assert_eq!(iter.next(), Some(&mut 3));
  /// assert!(iter.next().is_none());
  /// ```
  pub fn iter_mut(&mut self) -> IterMut<T> {
    IterMut { next: self.head.as_mut().map(|node| &mut **node) }
  }
}

impl<T: PartialOrd> Drop for SortedList<T> {
  fn drop(&mut self) {
    // Drop the queue in an iterative fashion to avoid recursive drop calls
    let mut current = self.head.take();
    while let Some(mut node) = current {
      current = node.next.take();
    }
  }
}

pub struct IntoIter<T: PartialOrd>(SortedList<T>);

impl<T: PartialOrd> Iterator for IntoIter<T> {
  type Item = Box<Node<T>>;
  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub struct Iter<'a, T: PartialOrd + 'a> {
  next: Option<&'a Node<T>>,
}

impl<'a, T: PartialOrd> Iterator for Iter<'a, T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    self.next.map(|node| {
      self.next = node.next.as_ref().map(|node| &**node);
      &node.data
    })
  }
}

pub struct IterMut<'a, T: PartialOrd + 'a> {
  next: Option<&'a mut Node<T>>,
}

impl<'a, T: PartialOrd> Iterator for IterMut<'a, T> {
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
  use super::SortedList;
  use super::super::Node;
  use alloc::boxed::Box;

  #[test]
  fn sorted_insert_unsorted() {
    let mut list = SortedList::new();

    list.insert(Box::new(Node::new(4)));
    list.insert(Box::new(Node::new(1)));
    list.insert(Box::new(Node::new(2)));
    list.insert(Box::new(Node::new(3)));

    assert_eq!(list.pop().map(|n| n.data), Some(1));
    assert_eq!(list.pop().map(|n| n.data), Some(2));
    assert_eq!(list.pop().map(|n| n.data), Some(3));
    assert_eq!(list.pop().map(|n| n.data), Some(4));
    assert!(list.pop().is_none());
  }

  #[test]
  fn sorted_insert_unsorted_2() {
    let mut list = SortedList::new();

    list.insert(Box::new(Node::new(3)));
    list.insert(Box::new(Node::new(4)));
    list.insert(Box::new(Node::new(2)));
    list.insert(Box::new(Node::new(1)));

    assert_eq!(list.pop().map(|n| n.data), Some(1));
    assert_eq!(list.pop().map(|n| n.data), Some(2));
    assert_eq!(list.pop().map(|n| n.data), Some(3));
    assert_eq!(list.pop().map(|n| n.data), Some(4));
    assert!(list.pop().is_none());
  }

  #[test]
  fn sorted_insert_reverse_sorted() {
    let mut list = SortedList::new();

    list.insert(Box::new(Node::new(4)));
    list.insert(Box::new(Node::new(3)));
    list.insert(Box::new(Node::new(2)));
    list.insert(Box::new(Node::new(1)));

    assert_eq!(list.pop().map(|n| n.data), Some(1));
    assert_eq!(list.pop().map(|n| n.data), Some(2));
    assert_eq!(list.pop().map(|n| n.data), Some(3));
    assert_eq!(list.pop().map(|n| n.data), Some(4));
    assert!(list.pop().is_none());
  }

  #[test]
  fn sorted_insert_sorted() {
    let mut list = SortedList::new();

    list.insert(Box::new(Node::new(1)));
    list.insert(Box::new(Node::new(2)));
    list.insert(Box::new(Node::new(3)));
    list.insert(Box::new(Node::new(4)));

    assert_eq!(list.pop().map(|n| n.data), Some(1));
    assert_eq!(list.pop().map(|n| n.data), Some(2));
    assert_eq!(list.pop().map(|n| n.data), Some(3));
    assert_eq!(list.pop().map(|n| n.data), Some(4));
    assert!(list.pop().is_none());
  }

  #[test]
  fn merge_1() {
    let mut list1 = SortedList::new();
    let mut list2 = SortedList::new();

    list1.insert(Box::new(Node::new(4)));
    list1.insert(Box::new(Node::new(3)));
    list2.insert(Box::new(Node::new(2)));
    list2.insert(Box::new(Node::new(1)));

    list1.merge(list2);

    assert_eq!(list1.pop().map(|n| n.data), Some(1));
    assert_eq!(list1.pop().map(|n| n.data), Some(2));
    assert_eq!(list1.pop().map(|n| n.data), Some(3));
    assert_eq!(list1.pop().map(|n| n.data), Some(4));
    assert!(list1.pop().is_none());
  }

  #[test]
  fn merge_2() {
    let mut list1 = SortedList::new();
    let mut list2 = SortedList::new();

    list1.insert(Box::new(Node::new(2)));
    list1.insert(Box::new(Node::new(1)));
    list2.insert(Box::new(Node::new(4)));
    list2.insert(Box::new(Node::new(3)));

    list1.merge(list2);

    assert_eq!(list1.pop().map(|n| n.data), Some(1));
    assert_eq!(list1.pop().map(|n| n.data), Some(2));
    assert_eq!(list1.pop().map(|n| n.data), Some(3));
    assert_eq!(list1.pop().map(|n| n.data), Some(4));
    assert!(list1.pop().is_none());
  }

  #[test]
  fn into_iter() {
    let mut list = SortedList::new();
    list.insert(Box::new(Node::new(1)));
    list.insert(Box::new(Node::new(2)));
    list.insert(Box::new(Node::new(3)));

    let mut iter = list.into_iter();
    assert_eq!(iter.next().map(|n| n.data), Some(1));
    assert_eq!(iter.next().map(|n| n.data), Some(2));
    assert_eq!(iter.next().map(|n| n.data), Some(3));
    assert!(iter.next().is_none());
  }

  #[test]
  fn iter() {
    let mut list = SortedList::new();
    list.insert(Box::new(Node::new(1)));
    list.insert(Box::new(Node::new(2)));
    list.insert(Box::new(Node::new(3)));

    {
      let mut iter = list.iter();
      assert_eq!(iter.next(), Some(&1));
      assert_eq!(iter.next(), Some(&2));
      assert_eq!(iter.next(), Some(&3));
      assert!(iter.next().is_none());
    }

    assert_eq!(list.pop().map(|n| n.data), Some(1));
    assert_eq!(list.pop().map(|n| n.data), Some(2));
    assert_eq!(list.pop().map(|n| n.data), Some(3));
    assert!(list.pop().is_none());
  }

  #[test]
  fn iter_mut() {
    let mut list = SortedList::new();
    list.insert(Box::new(Node::new(1)));
    list.insert(Box::new(Node::new(2)));
    list.insert(Box::new(Node::new(3)));

    {
      let mut iter = list.iter_mut();
      assert_eq!(iter.next(), Some(&mut 1));
      assert_eq!(iter.next(), Some(&mut 2));
      assert_eq!(iter.next(), Some(&mut 3));
      assert!(iter.next().is_none());
    }

    assert_eq!(list.pop().map(|n| n.data), Some(1));
    assert_eq!(list.pop().map(|n| n.data), Some(2));
    assert_eq!(list.pop().map(|n| n.data), Some(3));
    assert!(list.pop().is_none());
  }
}

/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

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
        // self.head is a box, in a node, as a reference, so all layers need to be dereferenced.
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

    /// Takes an item off of the front of the list. If there are no items in the list,
    /// it returns None.
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
        // TODO: Figure out a more efficient way to do this (the other list is in sorted order
        // after all...)
        for item in list.into_iter() {
            self.insert(item);
        }
    }

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

    /// Checks if the list is empty, returning true if it is, and false otherwise.
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

#[cfg(test)]
mod tests {
    use super::SortedList;
    use super::super::Node;
    use alloc::boxed::Box;

    #[test]
    fn test_list_properly_sorts_when_given_unsorted_data() {
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
    fn test_list_properly_sorts_when_given_unsorted_data_part2() {
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
    fn test_list_properly_sorts_when_given_reverse_sorted_data() {
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
    fn test_list_properly_sorts_when_given_already_sorted_data() {
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
    fn test_merging_two_lists_creates_one_sorted_list() {
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
    fn test_merging_two_lists_creates_one_sorted_list_part2() {
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
    fn test_list_with_into_iter_takes_ownership_of_list_elements() {
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
    fn test_iter_correctly_iterates_through_list_elements() {
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
}

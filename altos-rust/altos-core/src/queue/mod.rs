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

//! Implementations for different collections used throughout the kernel.

mod queue;
mod atomic_queue;
mod sorted_list;
mod ringbuffer;

pub use self::queue::*;
pub use self::atomic_queue::*;
pub use self::sorted_list::*;
pub use self::ringbuffer::*;

use alloc::boxed::Box;
use core::ops::{Deref, DerefMut};

/// A wrapper struct that is used in AltOS-Rust collections.
///
/// This type is used to provide a common way of passing allocated objects between collections
/// without doing reallocations. This is because of tight memory constraints, so it's best to avoid
/// reallocating an object if possible.
#[repr(C)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    /// Creates a new `Node<T>` wrapping an object of type `T`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use altos_core::queue::Node;
    ///
    /// let usize_node = Node::new(0usize);
    /// let isize_node = Node::new(0isize);
    /// ```
    pub fn new(data: T) -> Self {
        Node {
            data: data,
            next: None,
        }
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    /// Gives a reference to the wrapped data.
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Node<T> {
    /// Gives a mutable reference to the wrapped data.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

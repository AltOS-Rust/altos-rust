// queue/atomic_queue.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/3/16
//! A synchronized wrapper around the Queue struct.
use queue::{Queue, Node};
use alloc::boxed::Box;
use sync::{SpinMutex, SpinGuard};

/// A queue that is wrapped in a mutex lock.
///
/// This collection is meant for use in the kernel, so it is implemented with a spin lock rather
/// than a mutex lock. This type should not be used outside of the kernel. The `SyncQueue` just
/// acts as a wrapper around the `Queue` collection, with all of its methods behaving the same way.
pub struct SyncQueue<T> {
  lock: SpinMutex<Queue<T>>,
}

unsafe impl<T: Send> Sync for SyncQueue<T> {}
unsafe impl<T: Send> Send for SyncQueue<T> {}

impl<T> SyncQueue<T> {
  /// Creates a new `SyncQueue` with an empty queue.
  pub const fn new() -> Self {
    SyncQueue { lock: SpinMutex::new(Queue::new()) }
  }

  /// Creates a `SyncQueue` from a `Queue`.
  pub fn from(queue: Queue<T>) -> Self {
    SyncQueue { lock: SpinMutex::new(queue) }
  }

  /// Places an item onto the back of the queue.
  pub fn enqueue(&self, elem: Box<Node<T>>) {
    let mut queue = self.lock();
    queue.enqueue(elem);
  }

  /// Takes an item off of the front of the queue.
  pub fn dequeue(&self) -> Option<Box<Node<T>>> {
    let mut queue = self.lock();
    queue.dequeue()
  }

  /// Removes all items from the queue matching `predicate`.
  pub fn remove<F: Fn(&T) -> bool>(&self, predicate: F) -> Queue<T> {
    let mut queue = self.lock();
    queue.remove(predicate)
  }

  /// Appends `queue` onto the end of `self`.
  pub fn append(&self, to_append: Queue<T>) {
    let mut queue = self.lock();
    queue.append(to_append);
  }

  /// Modifies all items in the queue with `block`.
  ///
  /// This is used over an `iter_mut()` function because returning an iterator over mutable
  /// references would break the synchronization guarantee.
  #[allow(deprecated)]
  pub fn modify_all<F: Fn(&mut T)>(&self, block: F) {
    let mut queue = self.lock();
    queue.modify_all(block);
  }

  /// Removes all items from `self` and returns it as a new `Queue`.
  pub fn remove_all(&self) -> Queue<T> {
    let mut queue = self.lock();
    queue.remove_all()
  }

  /// Checks if `self` is empty, returns true if it is, false otherwise.
  pub fn is_empty(&self) -> bool {
    let queue = self.lock();
    queue.is_empty()
  }

  fn lock(&self) -> SpinGuard<Queue<T>> {
    self.lock.lock()
  }
}

impl<T> Default for SyncQueue<T> {
  /// Creates an empty `SyncQueue`.
  fn default() -> Self {
    SyncQueue::new()
  }
}

// queue/ringbuffer.rs
// AltOS Rust
//
// Created by Daniel Seitz on 2/5/17

// TODO: Make variable sized ring buffers?
/// The size of any ring buffers
pub const RING_BUFFER_SIZE: usize = 8;

/// A ring buffer to keep track of some data
pub struct RingBuffer {
  data: [u8; RING_BUFFER_SIZE],
  // TODO: u8?
  start: usize,
  end: usize,
  full: bool,
}

impl RingBuffer {
  /// Create a new RingBuffer that is empty
  pub const fn new() -> Self {
    //debug_assert!(size > 0);
    RingBuffer {
      data: [0; RING_BUFFER_SIZE],
      start: 0,
      end: 0,
      full: false,
    }
  }

  /// Insert a byte into the buffer
  ///
  /// Return true if the byte was successfully inserted into the buffer
  pub fn insert(&mut self, byte: u8) -> bool {
    if !self.full {
      self.data[self.end] = byte;
      self.end += 1;
      if self.end >= self.data.len() {
        self.end = 0;
      }
      if self.end == self.start {
        self.full = true;
      }
      true
    }
    else {
      false
    }
  }

  /// Remove a byte from the buffer if there is one available
  pub fn remove(&mut self) -> Option<u8> {
    if self.start != self.end || self.full {
      let byte = self.data[self.start];
      self.start += 1;
      if self.start >= self.data.len() {
        self.start = 0;
      }
      self.full = false;
      Some(byte)
    }
    else {
      None
    }
  }

  /// Check if the buffer is empty
  pub fn is_empty(&self) -> bool {
    self.start == self.end && !self.full
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_ring_buffer_insert() {
    let mut buffer = RingBuffer::new();

    buffer.insert(0);
    buffer.insert(1);
    buffer.insert(2);

    assert_eq!(buffer.data[0], 0);
    assert_eq!(buffer.data[1], 1);
    assert_eq!(buffer.data[2], 2);
  }

  #[test]
  fn test_ring_buffer_remove() {
    let mut buffer = RingBuffer::new();

    assert_eq!(buffer.remove(), None);

    buffer.insert(0);
    buffer.insert(1);

    assert_eq!(buffer.remove(), Some(0));
    assert_eq!(buffer.remove(), Some(1));
    assert_eq!(buffer.remove(), None);
  }

  #[test]
  fn test_ring_buffer_overflow_drops_insertion_byte() {
    let mut buffer = RingBuffer::new();

    for i in 0..RING_BUFFER_SIZE {
      buffer.insert(i as u8);
    }

    // Overflow the buffer
    buffer.insert(!0);
    // First item should still be 0, not !0
    assert_eq!(buffer.data[0], 0);
  }

  #[test]
  fn test_ring_buffer_is_empty() {
    let mut buffer = RingBuffer::new();

    assert!(buffer.is_empty());

    buffer.insert(0);

    assert!(!buffer.is_empty());
  }
}

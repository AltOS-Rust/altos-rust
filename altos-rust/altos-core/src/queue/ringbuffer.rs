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

// NOTE: Make variable sized ring buffers? On further research, this doesn't seem to be possible at
// this time. From the documentation on the array primitive type:
// "...Rust does not yet support code that is generic over the size of an array type."
//
// So... I guess we can make multiple types each with a different size, like RingBuffer8,
// RingBuffer16, etc. If we need larger buffers.

/// The size of any ring buffers.
const RING_BUFFER_SIZE: usize = 8;

/// A queue to store bytes of data.
pub struct RingBuffer {
    data: [u8; RING_BUFFER_SIZE],
    start: usize,
    end: usize,
    full: bool,
}

impl RingBuffer {
    /// Create a new, empty RingBuffer.
    pub const fn new() -> Self {
        RingBuffer {
            data: [0; RING_BUFFER_SIZE],
            start: 0,
            end: 0,
            full: false,
        }
    }

    /// Insert a byte into the buffer.
    ///
    /// If the buffer is full, the byte being inserted will be dropped rather than overwriting
    /// data in the queue. Return true if the byte was successfully inserted into the buffer.
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

    /// Remove a byte from the buffer if there is one available.
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

    /// Check if the buffer is empty, returning true if it is, and false otherwise.
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

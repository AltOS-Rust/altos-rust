/*
 * Copyright © 2017 AltOS-Rust Team
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
 */

use altos_core::syscall::sleep;
use altos_core::sync::Mutex;
use altos_core::queue::RingBuffer;
use core::fmt::{self, Write, Arguments};
use peripheral::usart::{UsartX, Usart, USART2_TX_BUFFER_FULL_CHAN, USART2_TC_CHAN};

/// Buffer for transmitting bytes
pub static mut TX_BUFFER: RingBuffer = RingBuffer::new();

/// Buffer for receiving bytes
pub static mut RX_BUFFER: RingBuffer = RingBuffer::new();

// Mutex to ensure transmitted data is not jumbled.
static WRITE_LOCK: Mutex<()> = Mutex::new(());

#[macro_export]
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::write_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
#[cfg(not(test))]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

struct Serial {
    usart: Usart,
}

impl Serial {
    // Creates a new Serial object that is initialized to the Usart variable.
    fn new(usart: Usart) -> Self {
        Serial { usart: usart }
    }

    // Inserts a byte into the TX buffer, if byte cannot be inserted loops
    // and sleeps on the TX_BUFFER_FULL channel.
    fn buffer_byte(&mut self, byte: u8) {
        unsafe {
            while !TX_BUFFER.insert(byte) {
                // FIXME?: Might need to put this in a critical section?
                //let _g = CriticalSection::begin();
                self.usart.enable_transmit_interrupt();
                sleep(USART2_TX_BUFFER_FULL_CHAN);
            }
        }
    }
}

// Implementing the Write trait for the Serial struct.
// Allows for use of print and println.
impl Write for Serial {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.as_bytes() {
            if *byte == b'\n' {
                self.buffer_byte(b'\r');
            }
            self.buffer_byte(*byte);
        }
        self.usart.enable_transmit_complete_interrupt();
        self.usart.enable_transmit_interrupt();
        sleep(USART2_TC_CHAN);
        Ok(())
    }
}

struct DebugSerial {
    usart: Usart,
}

impl DebugSerial {
    fn new(usart: Usart) -> Self {
        DebugSerial { usart: usart }
    }

    fn write_byte(&mut self, byte: u8) {
        while !self.usart.is_tx_reg_empty() {}
        self.usart.transmit_byte(byte);
    }
}

impl Write for DebugSerial {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.as_bytes() {
            if *byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(*byte);
        }
        Ok(())
    }
}

pub fn write_fmt(args: Arguments) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut serial = Serial::new(usart2);

    let _g = WRITE_LOCK.lock();
    serial.write_fmt(args).ok();
}

pub fn write_str(s: &str) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut serial = Serial::new(usart2);

    let _g = WRITE_LOCK.lock();
    serial.write_str(s).ok();
}

// NOTE: debug assumes interrupts are turned off, so does not need lock.
#[no_mangle]
pub fn debug_fmt(args: Arguments) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut serial = DebugSerial::new(usart2);

    serial.write_fmt(args).ok();
}

// NOTE: debug assumes interrupts are turned off, so does not need lock.
#[no_mangle]
pub fn debug_str(s: &str) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut serial = DebugSerial::new(usart2);

    serial.write_str(s).ok();
}

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

//! This module handles input and output through the serial port.
//!
//! It implements print formatting for debug and for non-debug purposes.
//! Serial and DebugSerial types provide interfaces for printing characters
//! to the serial port.
//!
//! This module contains implementations of helper macros for print and println.

pub use self::imp::*;

#[cfg(not(feature="serial"))]
mod imp {
    use core::fmt::Arguments;
    #[no_mangle]
    #[doc(hidden)]
    pub fn debug_fmt(_args: Arguments) {
        // Stub
    }
}

#[cfg(feature="serial")]
mod imp {
    use altos_core::volatile::Volatile;
    use altos_core::syscall::sleep;
    use altos_core::sync::{Mutex, CriticalSection};
    use altos_core::queue::RingBuffer;
    use core::fmt::{self, Write, Arguments};
    use peripheral::usart::{UsartX, Usart, USART2_TX_CHAN, USART2_RX_CHAN};

    /// A buffer for transmitting bytes.
    pub static mut TX_BUFFER: RingBuffer = RingBuffer::new();

    /// A buffer for receiving bytes.
    pub static mut RX_BUFFER: RingBuffer = RingBuffer::new();

    // Mutex to ensure transmitted data is not jumbled.
    static WRITE_LOCK: Mutex<()> = Mutex::new(());
    static READ_LOCK: Mutex<()> = Mutex::new(());

    /// Print a formatted string to the serial port. This macro is intended for
    /// user code and should not be used to print within the kernel code.
    #[macro_export]
    #[cfg(not(test))]
    macro_rules! print {
        ($($arg:tt)*) => ({
            $crate::io::write_fmt(format_args!($($arg)*));
        });
    }

    /// Print a formatted string, with a new line appended to it, to the serial port.
    /// This macro is intended for user code and should not be used to print within
    /// the kernel code.
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
        fn new(usart: Usart) -> Self {
            Serial { usart: usart }
        }

        fn buffer_byte(&mut self, byte: u8) {
            unsafe {
                while !TX_BUFFER.insert(byte) {
                    //let _g = CriticalSection::begin();
                    self.usart.enable_transmit_interrupt();
                    sleep(USART2_TX_CHAN);
                }
            }
        }

        fn read(&mut self, buf: &mut [u8]) -> usize {
            // UNSAFE: Accessing mutable static
            while unsafe {
                let _g = CriticalSection::begin();
                Volatile::new(&RX_BUFFER).is_empty()
            } {
                sleep(USART2_RX_CHAN);
            }
            let mut read = 0;
            while read < buf.len() {
                let g = CriticalSection::begin();
                // UNSAFE: Accessing mutable static
                let byte = unsafe { RX_BUFFER.remove() };
                drop(g);
                match byte {
                    Some(byte) => {
                        buf[read] = byte;
                        read += 1;
                    },
                    None => break,
                }
            }
            read
        }
    }

    impl Write for Serial {
        fn write_str(&mut self, string: &str) -> fmt::Result {
            for byte in string.as_bytes() {
                if *byte == b'\n' {
                    self.buffer_byte(b'\r');
                }
                #[cfg(feature="minicom")]
                {
                    if *byte == b'\r' {
                        self.buffer_byte(b'\n');
                    }
                }
                self.buffer_byte(*byte);
            }
            //let g = CriticalSection::begin();
            self.usart.enable_transmit_interrupt();
            sleep(USART2_TX_CHAN);
            //drop(g);
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
                // If at end of line, write a carriage return because
                // minicom doesn't go to beginning of line on its own.
                if *byte == b'\n' {
                    self.write_byte(b'\r');
                }
                self.write_byte(*byte);
            }
            Ok(())
        }
    }

    #[doc(hidden)]
    pub fn write_fmt(args: Arguments) {
        let usart2 = Usart::new(UsartX::Usart2);
        let mut serial = Serial::new(usart2);

        let _g = WRITE_LOCK.lock();
        serial.write_fmt(args).ok();
    }

    #[doc(hidden)]
    pub fn write_str(s: &str) {
        let usart2 = Usart::new(UsartX::Usart2);
        let mut serial = Serial::new(usart2);

        let _g = WRITE_LOCK.lock();
        serial.write_str(s).ok();
    }

    // NOTE: debug assumes interrupts are turned off, so does not need lock.
    #[no_mangle]
    #[doc(hidden)]
    pub fn debug_fmt(args: Arguments) {
        let usart2 = Usart::new(UsartX::Usart2);
        let mut serial = DebugSerial::new(usart2);

        serial.write_fmt(args).ok();
    }

    // NOTE: debug assumes interrupts are turned off, so does not need lock.
    #[no_mangle]
    #[doc(hidden)]
    pub fn debug_str(s: &str) {
        let usart2 = Usart::new(UsartX::Usart2);
        let mut serial = DebugSerial::new(usart2);

        serial.write_str(s).ok();
    }

    #[doc(hidden)]
    pub fn poll_char() -> Option<u8> {
        let usart2 = Usart::new(UsartX::Usart2);
        let mut serial = Serial::new(usart2);
        let mut buf: [u8; 1] = [0];
        let _g = READ_LOCK.lock();
        match serial.read(&mut buf) {
            0 => None,
            _ => Some(buf[0]),
        }
    }
}

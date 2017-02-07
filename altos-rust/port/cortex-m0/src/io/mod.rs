
use altos_core::syscall::sleep;
//use altos_core::sync::CriticalSection;
use altos_core::queue::RingBuffer;
use core::fmt::{self, Write, Arguments};
use peripheral::usart::{UsartX, Usart, USART2_CHAN};

pub static mut TX_BUFFER: RingBuffer = RingBuffer::new();
pub static mut RX_BUFFER: RingBuffer = RingBuffer::new();

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        $crate::io::debug_fmt(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

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
    fn new(usart: Usart) -> Self {
        Serial { usart: usart }
    }

    fn buffer_byte(&mut self, byte: u8) {
        unsafe {
            while !TX_BUFFER.insert(byte) {
                // FIXME?: Might need to put this in a critical section?
                //let _g = CriticalSection::begin();
                self.usart.enable_transmit_interrupt();
                sleep(USART2_CHAN);
            }
        }
    }
}

// TODO: Need to lock this to avoid data race
impl Write for Serial {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.as_bytes() {
            if *byte == b'\n' {
              self.buffer_byte(b'\r');
            }
            self.buffer_byte(*byte);
        }
        self.usart.enable_transmit_interrupt();
        sleep(USART2_CHAN);
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
        while !self.usart.get_txe() {}
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

    serial.write_fmt(args).ok();
}

pub fn write_str(s: &str) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut serial = Serial::new(usart2);

    serial.write_str(s).ok();
}

pub fn debug_fmt(args: Arguments) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut serial = DebugSerial::new(usart2);

    serial.write_fmt(args).ok();
}

pub fn debug_str(s: &str) {
    let usart2 = Usart::new(UsartX::Usart2);
    let mut serial = DebugSerial::new(usart2);

    serial.write_str(s).ok();
}

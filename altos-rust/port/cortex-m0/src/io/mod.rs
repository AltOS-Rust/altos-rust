
use altos_core::syscall::sleep;
use altos_core::sync::Mutex;
use altos_core::queue::RingBuffer;
use core::fmt::{self, Write, Arguments};
use peripheral::usart::{UsartX, Usart, USART2_TX_BUFFER_FULL_CHAN, USART2_TC_CHAN};

pub static mut TX_BUFFER: RingBuffer = RingBuffer::new();
pub static mut RX_BUFFER: RingBuffer = RingBuffer::new();

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
    fn new(usart: Usart) -> Self {
        Serial { usart: usart }
    }

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


use altos_core::syscall::sleep;
use core::fmt::{self, Write, Arguments};
use peripheral::usart::{UsartX, Usart, USART2_CHAN};

// TODO: Make kernel print macros
#[cfg(not(test))]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::write_fmt(format_args!($($arg)*));
    });
}

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

    fn write_byte(&mut self, byte: u8) {
        while !self.usart.get_txe() { sleep(USART2_CHAN); }
        self.usart.transmit_byte(byte);
    }
}

// TODO: Need to lock this to avoid data race
impl Write for Serial {
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

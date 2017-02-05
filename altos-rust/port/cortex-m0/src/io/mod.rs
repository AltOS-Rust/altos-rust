
use core::fmt::{self, Write, Arguments};
use peripheral::usart::{UsartX, Usart};

struct Serial {
    usart: Usart,
}

impl Serial {
    fn new(usart: Usart) -> Self {
        Serial { usart: usart }
    }

    fn write_byte(&mut self, byte: u8) {
        while !self.usart.get_txe() {}
        self.usart.transmit_byte(byte);
    }
}
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

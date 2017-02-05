
use core::fmt::{self, Write};

struct Serial;

impl Write for Serial {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        let usart2 = Usart::new(UsartX::Usart2);
        for byte in string.as_bytes() {
            while !usart2.get_txe() {}
            usart2.transmit_byte(*byte);
        }
        Ok(())
    }
}

pub fn write_fmt(args: Arguments) {
    Serial.write_fmt(args).ok();
}

pub fn write_str(s: &str) {
    Serial.write_str(s).ok();
}

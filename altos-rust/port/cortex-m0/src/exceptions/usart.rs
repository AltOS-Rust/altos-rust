
use peripheral::usart::{Usart, USART2_CHAN};
use altos_core::syscall;
use io::TX_BUFFER;
use io::RX_BUFFER;

pub fn usart_tx(mut usart: Usart) {
    unsafe {
        if usart.get_txe() {
            if let Some(byte) = TX_BUFFER.remove() {
                usart.transmit_byte(byte);
            }
            else {
                usart.disable_transmit_interrupt();
                syscall::wake(USART2_CHAN);
            }
        }
    }
}

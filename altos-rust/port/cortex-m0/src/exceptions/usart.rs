
use peripheral::usart::{Usart, USART2_TX_BUFFER_FULL_CHAN, USART2_TC_CHAN};
use altos_core::syscall;
use io::TX_BUFFER;
use io::RX_BUFFER;

pub fn usart_tx(mut usart: Usart) {
    if usart.get_txe() {
        if let Some(byte) = unsafe { TX_BUFFER.remove() } {
            usart.transmit_byte(byte);
        }
        else {
            usart.disable_transmit_interrupt();
            syscall::wake(USART2_TX_BUFFER_FULL_CHAN);
        }
    }

    if usart.get_tc() {
        usart.disable_transmit_complete_interrupt();
        syscall::wake(USART2_TC_CHAN);
        usart.clear_tc();
    }
}

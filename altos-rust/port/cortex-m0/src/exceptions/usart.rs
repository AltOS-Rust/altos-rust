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

use peripheral::usart::{Usart, USART2_TX_BUFFER_FULL_CHAN, USART2_TC_CHAN};
use altos_core::syscall;
use io::TX_BUFFER;
use io::RX_BUFFER;

/// Handles transmitting any bytes when an interrupt is generated
pub fn usart_tx(mut usart: Usart) {
    if usart.is_tx_reg_empty() {
        if let Some(byte) = unsafe { TX_BUFFER.remove() } {
            usart.transmit_byte(byte);
        }
        else {
            usart.disable_transmit_interrupt();
            syscall::wake(USART2_TX_BUFFER_FULL_CHAN);
        }
    }

    if usart.is_transmission_complete() {
        usart.disable_transmit_complete_interrupt();
        syscall::wake(USART2_TC_CHAN);
        usart.clear_tc_flag();
    }
}

/// Handles receiving any bytes when an interrupt is generated
pub fn usart_rx(mut usart: Usart) {
    kprintln!("{}", usart.load_byte());
    if usart.is_rx_reg_full() {
        unsafe { ::arm::asm::bkpt(); }
        usart.load_byte();
    }
}

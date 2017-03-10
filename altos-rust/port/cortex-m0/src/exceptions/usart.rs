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
extern crate arm;

use peripheral::usart::{Usart, USART2_TX_CHAN, USART2_RX_CHAN};
use altos_core::syscall;
use io::{TX_BUFFER, RX_BUFFER};

// Handles transmitting any bytes when an interrupt is generated
pub fn usart_tx(mut usart: Usart) {
    if usart.is_tx_reg_empty() {
        if let Some(byte) = unsafe { TX_BUFFER.remove() } {
            usart.transmit_byte(byte);
        }
        else {
            usart.disable_transmit_interrupt();
            syscall::wake(USART2_TX_CHAN);
        }
    }

    if usart.is_transmission_complete() {
        usart.disable_transmit_complete_interrupt();
        syscall::wake(USART2_TX_CHAN);
        usart.clear_tc_flag();
    }
}

/// Handles receiving any bytes when an interrupt is generated
pub fn usart_rx(mut usart: Usart) {
    usart.clear_ore_flag();
    if usart.is_rx_reg_full() {
        let byte = usart.load_byte();
        unsafe { RX_BUFFER.insert(byte) };
        syscall::wake(USART2_RX_CHAN);
    }
}

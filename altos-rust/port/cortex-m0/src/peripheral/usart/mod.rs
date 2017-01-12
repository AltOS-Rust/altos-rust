// peripheral/serial/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16
mod usart;
mod control;
mod defs;
mod baudr;
mod gpio;

use self::usart::USART;
use self::usart::USARTx;

// TODO
//pub fn init() {
//    let usart1: USART = USART::new(USARTx::USART1);
//}

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

/* This submodule contains the function implementations for the Usartx_TDR.
 * The TDR is the transmit data register and is responsible for transmitting
 * data through the serial bus.
 */

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct TDR {
    base_addr: *const u32,
}

impl Register for TDR {
    fn new(base_addr: *const u32) -> Self {
        TDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        TDR_OFFSET
    }
}

impl TDR {
    /* Bits 31:9 Reserved, must be kept at reset value.
     * Bits 8:0 TDR[8:0]: Transmit data value
     *   Contains the data character to be transmitted.
     * The TDR register provides the parallel interface between the internal
     * bus and the output shift register.
     * When transmitting with the parity enabled (PCE bit set to 1 in the
     * USARTx_CR1 register), the value written in the MSB (bit 7 or bit 8
     * depending on the data length) has no effect because it is replaced by
     * the parity.
     */
    pub fn store(&mut self, byte: u8) {
        unsafe {
            let mut reg = self.addr();
            reg.store(byte as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_tdr_has_ascii_value_of_97_on_store_of_char_a() {
        let mut tdr = test::create_register::<TDR>();
        tdr.store(b'a');
        assert_eq!(tdr.register_value(), 97);
    }

    #[test]
    fn test_tdr_has_ascii_value_of_98_on_last_store_of_char_b() {
        let mut tdr = test::create_register::<TDR>();
        tdr.store(b'i');
        tdr.store(b'z');
        tdr.store(b'b');
        assert_eq!(tdr.register_value(), 98);
    }
}

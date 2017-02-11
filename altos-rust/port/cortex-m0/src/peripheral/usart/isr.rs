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

/// This submodule contains the function implementations for the Usartx_ISR.
/// The ISR is the interrupt service register and is responsible for generating
/// the interrupts for the Usart.
///
/// The bit definitions used for the bit operations are located in: defs.rs

use super::super::Register;
use super::defs::*;

/// Stores base address of the ISR, which is the address
/// of the Usart being used to access this register.
#[derive(Copy, Clone, Debug)]
pub struct ISR {
    base_addr: *const u32,
}

/// Implements the Register trait for Usartx_ISR.
/// Stores base address for the interrupt service register, which is the address
/// of the Usart being used to access this register. Uses the base address
/// combined with the register offset to calculate the register address.
impl Register for ISR {
    /* Sets the base address as the Usart address.
     * Returns itself to the calling routine.
     */
    fn new(base_addr: *const u32) -> Self {
        ISR { base_addr: base_addr }
    }

    /* Helper function to calcluate the address of ISR.
     * Supplies the base address to the `addr()` Register routine.
     * Used in conjunction with the 'mem_offset' function below.
     */
    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    /* Helper function to calculate the address of ISR.
     * Supplies the ISR offset address back to the 'addr()' Register routine.
     * Used in conjuction with the 'base_addr' function above.
     */
    fn mem_offset(&self) -> u32 {
        ISR_OFFSET
    }
}

/// Function implementations for the Usartx_ISR.
/// These functions are called from the wrapper functions defined
/// for the Usart struct.
impl ISR {

    /// Returns true if the RXNE flag is set, false otherwise.
    /* Bit 5 RXNE: Read data register not empty
     *   This bit is set by hardware when the content of the RDR shift register
     *   has been transferred to the USARTx_RDR. It is cleared by a
     *   read to the USARTx_RDR.
     *   The RXNE flag can also be cleared by writing 1 to the RXFRQ in the
     *   USARTx_RQR.
     *   An interrupt is generated if RXNEIE=1 in the USARTx_CR1.
     *      0: data is not received
     *      1: Received data is ready to be read.
     */
    pub fn get_rxne(&self) -> bool {
        unsafe {
            *self.addr() & ISR_RXNE != 0
        }
    }

    /// Returns true if the transmit complete flag is set, false otherwise.
    /* Bit 6 - TC: Transmission Complete
     *   This bit is set by hardware if the transmission of a frame containing
     *   data is complete and if TXE is set. An interrupt is generated if TCIE=1
     *   in the USARTx_CR1. It is cleared by software, writing 1 to the
     *   TCCF in the USARTx_ICR or by a write to the USARTx_TDR.
     *   An interrupt is generated if TCIE=1 in the USARTx_CR1.
     *       0: Transmission is not complete
     *       1: Transmission is complete
     *   Note: If TE bit is reset and no transmission is on going, the TC bit
     *   will be set immediately.
     */
    pub fn get_tc(&self) -> bool {
        unsafe {
            *self.addr() & ISR_TC != 0
        }
    }

    /// Returns true of the transmit data register is empty, false otherwise.
    /* Bit 7 - TXE: Transmit data register empty
     *   This bit is set by hardware when the content of the USARTx_TDR
     *   has been transferred into the shift register.
     *   It is cleared by a write to the USARTx_TDR. The TXE flag
     *   can also be cleared by writing 1 to the TXFRQ in the USARTx_RQR,
     *   in order to discard the data (only in smartcard T=0 mode,
     *   in case of transmission failure).  An interrupt is generated if the
     *   TXEIE bit=1 in the USARTx_CR1.
     *        0: data is not transferred to the shift register
     *        1: data is transferred to the shift register
     *   Note: This bit is used during single buffer transmission.
    */
    pub fn get_txe(&self) -> bool {
        unsafe {
            *self.addr() & ISR_TXE != 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_isr_get_rxne_returns_false_when_bit_not_set() {
        let isr = test::create_register::<ISR>();
        assert_eq!(isr.get_rxne(), false);
    }

    #[test]
    fn test_isr_get_rxne_returns_true_when_bit_is_set() {
        let isr = test::create_initialized_register::<ISR>(0b1 << 5);
        assert_eq!(isr.get_rxne(), true);
    }

    #[test]
    fn test_isr_get_tc_returns_false_when_bit_not_set() {
        let isr = test::create_register::<ISR>();
        assert_eq!(isr.get_tc(), false);
    }

    #[test]
    fn test_isr_get_tc_returns_true_when_bit_is_set() {
        let isr = test::create_initialized_register::<ISR>(0b1 << 6);
        assert_eq!(isr.get_tc(), true);
    }

    #[test]
    fn test_isr_get_txe_returns_false_when_bit_not_set() {
        let isr = test::create_register::<ISR>();
        assert_eq!(isr.get_txe(), false);
    }

    #[test]
    fn test_isr_get_txe_returns_true_when_bit_is_set() {
        let isr = test::create_initialized_register::<ISR>(1 << 7);
        assert_eq!(isr.get_txe(), true);
    }
}

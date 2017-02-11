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

/// This submodule contains the function implementations for the ICR.
/// The ICR is the interrupt clear register and is responsible for
/// clearing various interrupt flags that are generated in the ISR.
/// It does so by writing a 1 to specific bits in this register.
///
/// The bit definitions used for the bit operations are located in: defs.rs

use super::super::Register;
use super::defs::*;

/// Stores base address of the ICR, which is the address
/// of the Usart being used to access this register.
#[derive(Copy, Clone, Debug)]
pub struct ICR {
    base_addr: *const u32,
}

/// Implements the Register trait for Usartx_ICR.
/// Stores base address for the interrupt clear register, which is the address
/// of the Usart being used to access this register. Uses the base address
/// combined with the register offset to calculate the register address.
impl Register for ICR {
    // Sets the base address as the Usart address.
    // Returns itself to the calling routine.
    fn new(base_addr: *const u32) -> Self {
        ICR { base_addr: base_addr }
    }

    // Helper function to calcluate the address of the ICR.
    // Supplies the base address to the `addr()` Register routine.
    // Used in conjunction with the 'mem_offset' function below.
    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    // Helper function to calculate the address of the ICR.
    // Supplies the ICR offset address back to the 'addr()' Register routine.
    // Used in conjuction with the 'base_addr' function above.
    fn mem_offset(&self) -> u32 {
        ICR_OFFSET
    }
}

/// Function implementations for the ICR.
/// These functions are called from the wrapper functions defined
/// for the Usart struct.
impl ICR {
    /// Clears the ORE flag in the Usartx_ISR.
    /*  Bit 3 ORECF: Overrun error clear flag
     *  Writing 1 to this bit clears the ORE flag in the USARTx_ISR.
     */
    pub fn clear_ore(&self) {
        unsafe {
            *self.addr() |= ICR_ORECF;
        }
    }

    /// Clears the IDLE flag in the Usartx_ISR.
    /* Bit 4 IDLECF: Idle line detected clear flag
     * Writing 1 to this bit clears the IDLE flag in the USARTx_ISR.
     */
    pub fn clear_idle(&self) {
        unsafe {
            *self.addr() |= ICR_IDLECF;
        }
    }

    /// Clears the TC flag for the Usartx_ISR.
    /* Bit 6 TCCF: Transmission complete clear flag
     * Writing 1 to this bit clears the TC flag in the USARTx_ISR.
     */
    pub fn clear_tc(&self) {
        unsafe {
            *self.addr() |= ICR_TCCF;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

#[test]
    fn test_icr_clear_ore() {
        let mut icr = test::create_register::<ICR>();
        icr.clear_ore();

        assert_eq!(icr.register_value(), 0b1 << 3);
    }

#[test]
    fn test_icr_clear_tc() {
        let mut icr = test::create_register::<ICR>();
        icr.clear_tc();

        assert_eq!(icr.register_value(), 0b1 << 6);
    }
}

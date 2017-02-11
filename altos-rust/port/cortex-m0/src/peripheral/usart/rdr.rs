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

/// This submodule contains the function implementations for the Usartx_RDR.
/// The RDR is the read data register and is responsible for receiving data
/// through the serial bus.
///
/// The bit definitions used for the bit operations are located in: defs.rs

use super::super::Register;
use super::defs::*;

/// Stores base address of the RDR, which is the address
/// of the Usart being used to access this register.
#[derive(Copy, Clone, Debug)]
pub struct RDR {
    base_addr: *const u32,
}

/// Implements the Register trait for Usartx_RDR.
/// Stores base address for the read data register, which is the address
/// of the Usart being used to access this register. Uses the base address
/// combined with the register offset to calculate the register address.
impl Register for RDR {
    /* Sets the base address as the Usart address.
     * Returns itself to the calling routine.
     */
    fn new(base_addr: *const u32) -> Self {
        RDR { base_addr: base_addr }
    }

    /* Helper function to calcluate the address of RDR.
     * Supplies the base address to the `addr()` Register routine.
     * Used in conjunction with the 'mem_offset' function below.
     */
    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    /* Helper function to calculate the address of RDR.
     * Supplies the RDR offset address back to the 'addr()' Register routine.
     * Used in conjuction with the 'base_addr' function above.
     */
    fn mem_offset(&self) -> u32 {
        RDR_OFFSET
    }
}

/// Function implementations for the Usartx_RDR.
/// These functions are called from the wrapper functions defined
/// for the Usart struct.
impl RDR {
    /// Loads the received byte into the RDR
    /* Bits 31:9 Reserved, must be kept at reset value.
     * Bits 8:0 RDR[8:0]: Receive data value
     *   Contains the received data character.
     * The RDR register provides the parallel interface between the input
     * shift register and the internal bus.
     *
     * When receiving with the parity enabled, the value read in the MSB bit
     * is the received parity bit.
     */
    pub fn load(&self) -> u8 {
        unsafe {
            self.addr().load() as u8
        }
    }
}

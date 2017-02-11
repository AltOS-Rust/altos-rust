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

/// This submodule contains the function implementations for the Usartx_BRR.
/// The BRR is the baud rate register and is responsible for setting the
/// baud rate based on what the user needs.
///
/// The bit definitions used for the bit operations are located in: defs.rs

use super::super::Register;
use super::defs::*;

/// Five most common baud rates available.
#[derive(Copy, Clone)]
pub enum BaudRate {
    Hz4800,
    Hz9600,
    Hz19200,
    Hz57600,
    Hz115200,
}

/// Stores base address of the BRR, which is the address
/// of the Usart being used to access this register.
#[derive(Copy, Clone, Debug)]
pub struct BRR {
    base_addr: *const u32,
}

/// Implements the Register trait for Usartx_BRR.
/// Stores base address for the baud rate register, which is the address
/// of the Usart being used to access this register. Uses the base address
/// combined with the register offset to calculate the register address.
impl Register for BRR {
    /* Sets the base address as the Usart address.
     * Returns itself to the calling routine.
     */
    fn new(base_addr: *const u32) -> Self {
        BRR { base_addr: base_addr }
    }

    /* Helper function to calcluate the address of BRR.
     * Supplies the base address to the `addr()` Register routine.
     * Used in conjunction with the 'mem_offset' function below.
     */
    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    /* Helper function to calculate the address of BRR.
     * Supplies the BRR offset address back to the 'addr()' Register routine.
     * Used in conjuction with the 'base_addr' function above.
     */
    fn mem_offset(&self) -> u32 {
        BRR_OFFSET
    }
}

/// Function implementations for the Usartx_BRR.
/// These functions are called from the wrapper functions defined
/// for the Usart struct.
impl BRR {
    /// Sets the baud rate for receiving and transmitting data. Accounts
    /// for oversampling by 8 or 16. Stores the final calculated rate necessary
    /// to acheive the intended baud rate in the BRR.
    /* Bits 31:16 Reserved, must be kept at reset value.
     * Bits 15:4 BRR[15:4]
     *   BRR[15:4] = USARTDIV[15:4]
     * Bits 3:0 BRR[3:0]
     *   When OVER8 = 0, BRR[3:0] = USARTDIV[3:0].
     *   When OVER8 = 1:
     *   BRR[2:0] = USARTDIV[3:0] shifted 1 bit to the right.
     *   BRR[3] must be kept cleared.
     */
    pub fn set_baud_rate(&mut self, baud_rate: BaudRate,
                         clock_rate: u32, over8: bool) {
        // Calculates rate by dividing the clock rate by the intended baud rate.
        let mut rate = match baud_rate {
            BaudRate::Hz4800 => clock_rate/4_800,
            BaudRate::Hz9600 => clock_rate/9_600,
            BaudRate::Hz19200 => clock_rate/19_200,
            BaudRate::Hz57600 => clock_rate/57_600,
            BaudRate::Hz115200 => clock_rate/115_200,
        };

        // If over8 bit in CR1 is enabled, shifts the lower four bits right 1.
        if over8 {
            // Mask off the lower 4 bits.
            let mut low_bits = rate & DIV_MASK;
            low_bits = low_bits >> 1;
            // Clear the lower 4 bits of the rate value
            rate &= !(DIV_MASK);
            // replace them with the calculated over8 value.
            rate |= low_bits;
        }

        // Store the resulting rate value in the BRR.
        unsafe {
            let mut reg = self.addr();
            reg.store(rate);
        }
    }
}

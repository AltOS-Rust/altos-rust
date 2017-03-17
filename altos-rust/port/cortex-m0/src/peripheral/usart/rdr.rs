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

/* This submodule contains the function implementations for the Usartx_RDR.
 * The RDR is the read data register and is responsible for receiving data
 * through the serial bus.
 */

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct RDR {
    base_addr: *const u32,
}

impl Register for RDR {
    fn new(base_addr: *const u32) -> Self {
        RDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        RDR_OFFSET
    }
}

impl RDR {
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

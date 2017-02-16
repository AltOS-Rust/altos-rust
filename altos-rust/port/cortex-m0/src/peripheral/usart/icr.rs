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

/* This submodule contains the function implementations for the ICR.
 * The ICR is the interrupt clear register and is responsible for
 * clearing various interrupt flags that are generated in the ISR.
 * It does so by writing a 1 to specific bits in this register.
 */

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct ICR {
    base_addr: *const u32,
}

impl Register for ICR {
    fn new(base_addr: *const u32) -> Self {
        ICR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        ICR_OFFSET
    }
}

impl ICR {
    /*  Bit 3 ORECF: Overrun error clear flag
     *  Writing 1 to this bit clears the ORE flag in the USARTx_ISR.
     */
    pub fn clear_ore(&mut self) {
        unsafe {
            *self.addr() |= ICR_ORECF;
        }
    }

    /* Bit 4 IDLECF: Idle line detected clear flag
     * Writing 1 to this bit clears the IDLE flag in the USARTx_ISR.
     */
    pub fn clear_idle(&mut self) {
        unsafe {
            *self.addr() |= ICR_IDLECF;
        }
    }

    /* Bit 6 TCCF: Transmission complete clear flag
     * Writing 1 to this bit clears the TC flag in the USARTx_ISR.
     */
    pub fn clear_tc(&mut self) {
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

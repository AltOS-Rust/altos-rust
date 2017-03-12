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

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct ISR {
    base_addr: *const u32,
}

impl Register for ISR {
    fn new(base_addr: *const u32) -> Self {
        ISR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        ISR_OFFSET
    }
}

impl ISR {
    /*
    OVR: ADC Overrun
    Set by hardware when new conversion has happened while EOC flag is already set
    0: No overrun occurred (Or acknowledged and cleared by software)
    1: Overrun has occurred
    */
    pub fn overrun_occurred(&self) -> bool {
        unsafe {
            *self.addr() & ISR_OVR != 0
        }
    }

    /*
    EOC: End of conversion
    Set by hardware at end of each conversion of a channel when a new data
    result is available in ADC_DR register.
    0: Channel conversion not complete
    1: Channel conversion complete
    */
    pub fn get_eoc(&self) -> bool {
        unsafe {
            *self.addr() & ISR_EOC != 0
        }
    }

    /*
    EOSMP: End of sampling flag
    Set by hardware during conversion, at the end of the sampling phase.
    0: Not at end of sampling phase
    1: End of sampling phase reached
    */
    pub fn get_eosmp(&self) -> bool {
        unsafe {
            *self.addr() & ISR_EOSMP != 0
        }
    }

    /*
    ADRDY: ADC ready
    Set by hardware after ADC has been enabled (ADEN = 1) and reached ready state.
    0: ADC not yet ready to start conversion (Or already acknowledged)
    1: ADC is ready to start conversion
    */
    pub fn adc_ready(&self) -> bool {
        unsafe {
            *self.addr() & ISR_ADRDY != 0
        }
    }

    /*
    EOSEQ: End of sequence flag
    Set by hardware at end of conversion of a sequence of channels selected by CHSEL bits.
    Cleared by software.
    0: Conversion sequence not complete (or cleared by software)
    1: Conversion sequence complete
    */
    pub fn get_eoseq(&self) -> bool {
        unsafe {
            *self.addr() & ISR_EOSEQ != 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    // Register tests...
}

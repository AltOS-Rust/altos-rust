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
pub struct CR {
    base_addr: *const u32,
}

impl Register for CR {
    fn new(base_addr: *const u32) -> Self {
        CR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CR_OFFSET
    }
}

impl CR {
    /*
    ADCAL: ADC calibration
    Set by software to start the calibration of the ADC. Cleared by hardware after calibration is complete.
    0: Calibration complete
    1: Write 1 to calibrate the ADC. Read at 1 means calibration is in progress.
    Note: Software may set ADCAL only when ADC is disabled (ADCAL=0, ADSTART=0, ADSTP=0, ADDIS=0, ADEN=0)
    */
    pub fn start_adc_calibration(&mut self) {
        unsafe {
            let mut reg = self.addr();
            *reg |= CR_ADCAL;
        }
    }

    pub fn is_adc_calibrating(&mut self) -> bool {
        unsafe {
            *self.addr() & CR_ADCAL != 0
        }
    }
    /*
    ADSTP: ADC stop conversion command
    Set by software to stop and discard an ongoing conversion.
    Cleared by hardware when conversion is effectively discarded.
    0: No ADC stop conversion command going
    1: Write 1 to stop the ADC. Read 1 means that an ADSTP command is in progress.
    Note: Software allowed to set ADSTP only when ADSTART=1 and ADDIS=0
    */
    pub fn stop_adc_conversion(&mut self) {
        unsafe {
            let mut reg = self.addr();
            *reg |= CR_ADSTP;
        }
    }

    pub fn is_adc_stopping_conversion(&mut self) -> bool {
        unsafe {
            *self.addr() & CR_ADSTP != 0
        }
    }
    /*
    ADSTART: ADC start conversion command
    Set by software to start ADC conversion. Depends on EXTEN[1:0] configuration bits.
    It is cleared by hardware (depends on EXTEN)
    0: No ADC conversion is going
    1: Write 1 to start the ADC. Read 1 means the ADC is operating and may be converting.
    Note: Software allowed to set ADSTART only when ADEN=1 and ADDIS=0
    */
    pub fn start_adc_conversion(&mut self) {
        unsafe {
            let mut reg = self.addr();
            *reg |= CR_ADSTART;
        }
    }

    pub fn is_adc_conversion_started(&mut self) -> bool {
        unsafe {
            *self.addr() & CR_ADSTART != 0
        }
    }
    /*
    ADDIS: ADC disable command
    Set by software to disabled the ADC and put it in power-down state.
    Cleared by hardware once the ADC is effectively disabled.
    0: No ADDIS command going
    1: Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress.
    Note: Software allowed to set ADDIS only when ADEN=1 and ADSTART=0
    */
    pub fn disable_adc(&mut self) {
        unsafe {
            let mut reg = self.addr();
            *reg |= CR_ADDIS;
        }
    }

    /*
    ADEN: ADC enable command
    This bit is set by software to enable the ADC. The ADC will be ready to operate once ADRDY flag
    has been set. Cleared by hardware when the ADC is disabled, after execution of ADDIS command.
    0: ADC is disabled (OFF state)
    1: Write 1 to enable the ADC
    Note: Software allowed to set ADEN only when all bits of ADC_CR registers are 0
    */
    pub fn enable_adc(&mut self) {
        unsafe {
            let mut reg = self.addr();
            *reg |= CR_ADEN;
        }
    }
}

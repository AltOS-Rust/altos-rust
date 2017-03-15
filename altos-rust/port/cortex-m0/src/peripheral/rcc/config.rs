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

//! This module handles the CFGR register which deals with clock configuration

use super::super::Register;
use super::Clock;
use super::defs::*;

#[derive(Copy, Clone)]
pub struct ConfigControl {
    cfgr: CFGR,
    cfgr2: CFGR2,
}

impl ConfigControl {
    pub fn new(base_addr: *const u32) -> Self {
        ConfigControl {
            cfgr: CFGR::new(base_addr),
            cfgr2: CFGR2::new(base_addr),
        }
    }

    /// Set the system clock source. The system clock can only be run off of the HSI, HSE, PLL or
    /// HSI48 clocks, if another clock is specified the kernel will panic
    pub fn set_system_clock_source(&mut self, clock: Clock) {
        self.cfgr.set_system_clock_source(clock);
    }

    /// Return the system clock source
    pub fn get_system_clock_source(&self) -> Clock {
        self.cfgr.get_system_clock_source()
    }

    /// Set the specified clock to drive the PLL, only the HSI, HSE or HSI48 can drive the PLL, if
    /// another clock is specified the kernel will panic.
    pub fn set_pll_source(&mut self, clock: Clock) {
        self.cfgr.set_pll_source(clock);
    }

    /// Return the clock that is driving the PLL.
    pub fn get_pll_source(&self) -> Clock {
        self.cfgr.get_pll_source()
    }

    /// Set the PLL multiplier, the multiplier specified MUST be within the range of [2..16], if it
    /// is outside of that range the kernel will panic.
    pub fn set_pll_multiplier(&mut self, mul: u8) {
        self.cfgr.set_pll_multiplier(mul);
    }

    /// Get the current multiplier for the PLL, the multiplier is in a range of [2..16].
    pub fn get_pll_multiplier(&self) -> u8 {
        self.cfgr.get_pll_multiplier()
    }

    /// Set the PLL prediv factor, the factor specified MUST be within the range of [1..16],
    /// if it is outside that range the kernel will panic.
    pub fn set_pll_prediv_factor(&mut self, factor: u8) {
        self.cfgr2.set_pll_prediv_factor(factor);
    }

    /// Get the current prediv factor for the PLL, the factor is in a range of [1..16].
    pub fn get_pll_prediv_factor(&self) -> u8 {
        self.cfgr2.get_pll_prediv_factor()
    }
}

/// Clock Configuration Register
#[derive(Copy, Clone)]
struct CFGR {
    base_addr: *const u32,
}

impl Register for CFGR {
    fn new(base_addr: *const u32) -> Self {
        CFGR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CFGR_OFFSET
    }
}

impl CFGR {
    fn get_system_clock_source(&self) -> Clock {
        let set_bits = unsafe {
            let reg = self.addr();
            *reg & CFGR_SWS_MASK
        };
        match set_bits {
            CFGR_SWS_HSI => Clock::HSI,
            CFGR_SWS_HSE => Clock::HSE,
            CFGR_SWS_PLL => Clock::PLL,
            CFGR_SWS_HSI48 => Clock::HSI48,
            _    => panic!("CFGR::get_system_clock_source - set bits gave an unknown value!"),
        }
    }

    fn set_system_clock_source(&mut self, clock: Clock) {
        let mask = match clock {
            Clock::HSI => CFGR_CLOCK_HSI,
            Clock::HSE => CFGR_CLOCK_HSE,
            Clock::PLL => CFGR_CLOCK_PLL,
            Clock::HSI48 => CFGR_CLOCK_HSI48,
            _ => panic!("CFGR::set_system_clock_source - the clock argument cannot be used as a source!"),
        };

        unsafe {
            let mut reg = self.addr();
            // Zero the selection first (does this have any side effects)?
            *reg &= !CFGR_SW_CLEAR_MASK;
            *reg |= mask;
        }
    }

    fn get_pll_source(&self) -> Clock {
        let set_bits = unsafe {
            let reg = self.addr();
            *reg & CFGR_PLLSRC_MASK
        };

        match set_bits {
            CFGR_PLLSRC_HSI_2 | CFGR_PLLSRC_HSI_PREDIV => Clock::HSI,
            CFGR_PLLSRC_HSE_PREDIV => Clock::HSE,
            CFGR_PLLSRC_HSI48_PREDIV => Clock::HSI48,
            _ => panic!("CFGR::get_pll_source - set bits gave an unknown value!"),
        }
    }

    fn set_pll_source(&mut self, clock: Clock) {
        let mask = match clock {
            Clock::HSI   => CFGR_PLLSRC_HSI_2,
            Clock::HSE   => CFGR_PLLSRC_HSE_PREDIV,
            Clock::HSI48 => CFGR_PLLSRC_HSI48_PREDIV,
            _ => panic!("CFGR::set_pll_source - the clock argument cannot be used as a source!"),
        };

        unsafe {
            let mut reg = self.addr();
            // Zero the register first
            *reg &= !CFGR_PLLSRC_MASK;
            *reg |= mask;
        }
    }

    fn get_pll_multiplier(&self) -> u8 {
        let set_bits = unsafe {
            let reg = self.addr();
            (*reg & CFGR_PLLMUL_MASK) >> 18
        };

        // Just the way the multiplier is set up...
        let mut mul = set_bits + 2;
        if mul > 16 {
            mul = 16
        }
        mul as u8
    }

    fn set_pll_multiplier(&mut self, mul: u8) {
        if mul < 2 || mul > 16 {
            panic!("CFGR::set_pll_multiplier - the multiplier must be between 2..16!");
        }
        let mask = ((mul - 2) as u32) << 18;

        unsafe {
            let mut reg = self.addr();
            // Zero the register field
            *reg &= !CFGR_PLLMUL_MASK;
            *reg |= mask;
        }
    }
}

#[derive(Copy, Clone)]
struct CFGR2 {
    base_addr: *const u32,
}

impl Register for CFGR2 {
    fn new(base_addr: *const u32) -> Self {
        CFGR2 { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CFGR2_OFFSET
    }
}

impl CFGR2 {
    fn get_pll_prediv_factor(&self) -> u8 {
        let set_bits = unsafe {
            let reg = self.addr();
            *reg & CFGR2_PREDIV_MASK
        };

        // Division factor is 1 greater than the value of the bits set
        (set_bits + 1) as u8
    }

    fn set_pll_prediv_factor(&mut self, factor: u8) {
        if factor < 1 || factor > 16 {
            panic!("CFGR2::set_pll_prediv_factor - the division factor must be between 1..16!");
        }
        let mask = (factor - 1) as u32;

        unsafe {
            let mut reg = self.addr();
            // Zero the register field
            *reg &= !CFGR2_PREDIV_MASK;
            *reg |= mask;
        }
    }
}

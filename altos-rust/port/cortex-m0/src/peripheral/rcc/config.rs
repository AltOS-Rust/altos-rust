// peripheral/rcc/config.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! This module handles the CFGR register which deals with clock configuration

use super::super::Register;
use super::Clock;

#[derive(Copy, Clone)]
pub struct ConfigControl {
  cfgr: CFGR,
  cfgr2: CFGR2,
}

impl ConfigControl {
  pub fn new(base_addr: u32) -> Self {
    ConfigControl {
      cfgr: CFGR::new(base_addr),
      cfgr2: CFGR2::new(base_addr),
    }
  }

  /// Return the system clock source
  pub fn get_system_clock_source(&self) -> Clock {
    self.cfgr.get_system_clock_source()
  }

  /// Set the system clock source. The system clock can only be run off of the HSI, HSE, PLL or
  /// HSI48 clocks, if another clock is specified the kernel will panic
  pub fn set_system_clock_source(&self, clock: Clock) {
    self.cfgr.set_system_clock_source(clock);
  }

  /// Return the clock that is driving the PLL.
  pub fn get_pll_source(&self) -> Clock {
    self.cfgr.get_pll_source()
  }

  /// Set the specified clock to drive the PLL, only the HSI, HSE or HSI48 can drive the PLL, if
  /// another clock is specified the kernel will panic.
  pub fn set_pll_source(&self, clock: Clock) {
    self.cfgr.set_pll_source(clock);
  }

  /// Get the current multiplier for the PLL, the multiplier is in a range of [2..16]. 
  pub fn get_pll_multiplier(&self) -> u8 {
    self.cfgr.get_pll_multiplier()
  }

  /// Set the PLL multiplier, the multiplier specified MUST be within the range of [2..16], if it
  /// is outside of that range the kernel will panic.
  pub fn set_pll_multiplier(&self, mul: u8) {
    self.cfgr.set_pll_multiplier(mul);
  }

  /// Get the current prediv factor for the PLL, the factor is in a range of [1..16].
  pub fn get_pll_prediv_factor(&self) -> u8 {
    self.cfgr2.get_pll_prediv_factor()
  }

  /// Set the PLL prediv factor, the factor specified MUST be within the range of [1..16], if it is
  /// outside that range the kernel will panic.
  pub fn set_pll_prediv_factor(&self, factor: u8) {
    self.cfgr2.set_pll_prediv_factor(factor);
  }
}

/// Clock Configuration Register
#[derive(Copy, Clone)]
struct CFGR {
  base_addr: u32,
}

impl Register for CFGR {
  fn new(base_addr: u32) -> Self {
    CFGR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x04
  }
}

impl CFGR {
  fn get_system_clock_source(&self) -> Clock {
    let set_bits = unsafe {
      let reg = self.addr();
      (*reg & (0b11 << 2)) >> 2
    };

    match set_bits {
      0b00 => Clock::HSI,
      0b01 => Clock::HSE,
      0b10 => Clock::PLL,
      0b11 => Clock::HSI48,
      _    => panic!("CFGR::get_system_clock_source - set bits gave an unknown value!"),
    }
  }

  fn set_system_clock_source(&self, clock: Clock) {
    let mask = match clock {
      Clock::HSI => 0b00,
      Clock::HSE => 0b01,
      Clock::PLL => 0b10,
      Clock::HSI48 => 0b11,
      //TODO: Do we want to panic here? or return an error?
      _ => panic!("CFGR::set_system_clock_source - the clock argument cannot be used as a source!"),
    };

    unsafe {
      let mut reg = self.addr();

      // Zero the selection first (does this have any side effects)?
      *reg &= !0b11;
      *reg |= mask;
    }
  }

  fn get_pll_source(&self) -> Clock {
    let set_bits = unsafe {
      let reg = self.addr();
      (*reg & (0b11 << 15)) >> 15
    };

    match set_bits {
      0b00 | 0b01 => Clock::HSI,
      0b10 => Clock::HSE,
      0b11 => Clock::HSI48,
      _ => panic!("CFGR::get_pll_source - set bits gave an unknown value!"),
    }
  }

  fn set_pll_source(&self, clock: Clock) {
    let mask = match clock {
      Clock::HSI   => 0b00 << 15,
      Clock::HSE   => 0b10 << 15,
      Clock::HSI48 => 0b11 << 15,
      _ => panic!("CFGR::set_pll_source - the clock argument cannot be used as a source!"),
    };

    unsafe {
      let mut reg = self.addr();

      // Zero the register first
      *reg &= !0b11 << 18;
      *reg |= mask;
    }
  }

  fn get_pll_multiplier(&self) -> u8 {
    let set_bits = unsafe {
      let reg = self.addr();
      (*reg & (0b1111 << 18)) >> 18
    };
    
    // Just the way the multiplier is set up...
    let mut mul = set_bits + 2;
    if mul > 16 { 
      mul = 16
    }
    mul as u8
  }

  fn set_pll_multiplier(&self, mul: u8) {
    if mul < 2 || mul > 16 {
      panic!("CFGR::set_pll_multiplier - the multiplier must be between 2..16!");
    }
    let mask = ((mul - 2) as u32) << 18;

    unsafe {
      let mut reg = self.addr();

      // Zero the register field
      *reg &= !0b1111 << 18;
      *reg |= mask;
    }
  }
}

#[derive(Copy, Clone)]
struct CFGR2 {
  base_addr: u32,
}

impl Register for CFGR2 {
  fn new(base_addr: u32) -> Self {
    CFGR2 { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x2C
  }
}

impl CFGR2 {
  fn get_pll_prediv_factor(&self) -> u8 {
    let set_bits = unsafe {
      let reg = self.addr();
      *reg & 0b1111
    };
    
    // Division factor is 1 greater than the value of the bits set
    (set_bits + 1) as u8
  }

  fn set_pll_prediv_factor(&self, factor: u8) {
    if factor < 1 || factor > 16 {
      panic!("CFGR2::set_pll_prediv_factor - the division factor must be between 1..16!");
    }
    let mask = (factor - 1) as u32;

    unsafe {
      let mut reg = self.addr();

      // Zero the register field
      *reg &= !0b1111;
      *reg |= mask;
    }
  }
}

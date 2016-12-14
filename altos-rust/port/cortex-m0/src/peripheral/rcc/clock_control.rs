// peripheral/rcc/clock_control.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! This module handles the clock control register of the CRR

use super::super::Register;

pub mod clock_rate {
  static mut CLOCK_RATE: u32 = 0;

  pub fn get_system_clock_rate() -> u32 {
    unsafe { 
      CLOCK_RATE 
    }
  }

  pub fn update_system_clock_rate() {
    const HSI_VALUE: u32 = 8_000_000;
    const HSE_VALUE: u32 = 8_000_000;
    const HSI48_VALUE: u32 = 48_000_000;
    use super::Clock;
    use super::super::super::systick;

    let rcc = super::super::rcc();
    let rate = match rcc.get_system_clock_source() {
      Clock::HSI => HSI_VALUE,
      Clock::HSE => HSE_VALUE,
      Clock::HSI48 => HSI48_VALUE,
      Clock::PLL => {
        let multiplier = rcc.get_pll_multiplier() as u32;
        let source = rcc.get_pll_source();
        let prediv_factor = rcc.get_pll_prediv_factor() as u32;

        match source {
          Clock::HSE => (HSE_VALUE/prediv_factor) * multiplier,
          Clock::HSI48 => (HSI48_VALUE/prediv_factor) * multiplier,
          Clock::HSI => (HSI_VALUE/2) * multiplier,
          _ => panic!("CRR::update_system_core_clock - invalid clock driving the PLL!"),
        }
      },
      _ => panic!("CRR::update_system_core_clock - invalid clock for the system clock!"),
    };

    unsafe { CLOCK_RATE = rate; }
    let systick = systick::systick();
    // Interrupt every milisecond
    systick.set_reload_value(rate / 1000);
  }

}

pub enum Clock {
  HSI,
  HSI48,
  HSI14,
  HSE,
  PLL,
}

/// Clock Control Register
#[derive(Copy, Clone)]
pub struct ClockControl {
  cr: CR,
  cr2: CR2,
}

impl ClockControl {
  pub fn new(base_addr: u32) -> Self {
    ClockControl {
      cr: CR::new(base_addr),
      cr2: CR2::new(base_addr),
    }
  }
  
  /// Enable a clock
  pub fn enable_clock(&self, clock: Clock) {
    match clock {
      Clock::HSI | Clock::HSE | Clock::PLL => self.cr.set_clock(true, clock),
      Clock::HSI48 | Clock::HSI14 => self.cr2.set_clock(true, clock),
    };
  }

  /// Disable a clock, if a clock is unable to be disabled the return value will be false.
  pub fn disable_clock(&self, clock: Clock) -> bool {
    match clock {
      Clock::HSI | Clock::HSE | Clock::PLL => self.cr.set_clock(false, clock),
      Clock::HSI48 | Clock::HSI14 => self.cr2.set_clock(false, clock),
    }
  }

  /// Return true if the specified clock is enabled, false otherwise
  pub fn clock_is_on(&self, clock: Clock) -> bool {
    match clock {
      Clock::HSI | Clock::HSE | Clock::PLL => self.cr.clock_is_on(clock),
      Clock::HSI48 | Clock::HSI14 => self.cr2.clock_is_on(clock),
    }
  }

  /// Return true if the specified clock is ready for use, false otherwise
  pub fn clock_is_ready(&self, clock: Clock) -> bool {
    match clock {
      Clock::HSI | Clock::HSE | Clock::PLL => self.cr.clock_is_ready(clock),
      Clock::HSI48 | Clock::HSI14 => self.cr2.clock_is_ready(clock),
    }
  }
}

/// The CR register only controls the PLL, HSE, and HSI clocks, if another clock is passed in as an
/// argument to any of the methods that take a clock argument the kernel will panic.
#[derive(Copy, Clone)]
pub struct CR {
  base_addr: u32,
}

impl Register for CR {
  fn new(base_addr: u32) -> Self {
    CR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x0
  }
}

impl CR {
  /// Set a clock on if `enable` is true, off otherwise. If `enable` is true, the return value is
  /// always true. If `enable` is false, the return value will be true if the clock was
  /// successfully disabled.
  fn set_clock(&self, enable: bool, clock: Clock) -> bool {
    let mask = match clock {
      Clock::PLL => 1 << 24,
      Clock::HSE => 1 << 16,
      Clock::HSI => 1 << 0,
      _ => panic!("CR::enable_clock - argument clock is not controlled by this register!"),
    };
    
    unsafe {
      let mut reg = self.addr();
      if enable {
        *reg |= mask;
        true
      }
      else {
        *reg &= !mask;
        (*reg & mask) == 0
      }
    }
  }

  /// Return true if the specified clock is enabled.
  fn clock_is_on(&self, clock: Clock) -> bool {
    let mask = match clock {
      Clock::PLL => 1 << 24,
      Clock::HSE => 1 << 16,
      Clock::HSI => 1 << 0,
      _ => panic!("CR::clock_is_on - argument clock is not controlled by thsi register!"),
    };

    unsafe {
      let reg = self.addr();
      (*reg & mask) != 0
    }
  }

  /// Return true if the specified clock is ready for use.
  fn clock_is_ready(&self, clock: Clock) -> bool {
    let mask = match clock {
      Clock::PLL => 1 << 25,
      Clock::HSE => 1 << 17,
      Clock::HSI => 1 << 1,
      _ => panic!("CR::clock_is_ready - argument clock is not controlled by this register!"),
    };

    unsafe {
      let reg = self.addr();
      (*reg & mask) != 0
    }
  }
}

/// The CR2 register only controls the HSI48 and HSI14 clocks, if another clock is passed in as an
/// argument to any of the methods that take a clock argument the kernel will panic.
#[derive(Copy, Clone)]
pub struct CR2 {
  base_addr: u32,
}

impl Register for CR2 {
  fn new(base_addr: u32) -> Self {
    CR2 { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x34
  }
}

impl CR2 {
  /// Set a clock on if `enable` is true, off otherwise. If `enable` is true, the return value is
  /// always true. If `enable` is false, the return value will be true if the clock was
  /// successfully disabled.
  fn set_clock(&self, enable: bool, clock: Clock) -> bool {
    let mask = match clock {
      Clock::HSI48 => 1 << 16,
      Clock::HSI14 => 1 << 0,
      _ => panic!("CR2::set_clock - argument clock is not controlled by this register!"),
    };

    unsafe {
      let mut reg = self.addr();
      if enable {
        *reg |= mask;
        true
      }
      else {
        *reg &= !mask;
        (*reg & mask) == 0
      }
    }
  }

  /// Return true if the specified clock is enabled.
  fn clock_is_on(&self, clock: Clock) -> bool {
    let mask = match clock {
      Clock::HSI48 => 1 << 16,
      Clock::HSI14 => 1 << 0,
      _ => panic!("CR2::clock_is_on - argument clock is not controlled by this register!"),
    };

    unsafe {
      let reg = self.addr();
      (*reg & mask) != 0
    }
  }

  /// Return true if the specified clock is ready for use.
  fn clock_is_ready(&self, clock: Clock) -> bool {
    let mask = match clock {
      Clock::HSI48 => 1 << 17,
      Clock::HSI14 => 1 << 1,
      _ => panic!("CR2::clock_is_ready - argument clock is not controlled by this register!"),
    };

    unsafe {
      let reg = self.addr();
      (*reg & mask) != 0
    }
  }
}

// peripheral/rcc/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! This module controls the RCC (Reset and Clock Controller), it handles enabling and disabling
//! clocks, setting clock configurations and the reset flags that are set on a reset.

use super::Control;
use arm::asm::dsb;
use volatile::Volatile;
pub use self::clock_control::Clock;
pub use self::enable::Peripheral;

mod clock_control;
mod config;
mod enable;

pub fn rcc() -> RCC {
  RCC::rcc()
}

/// Reset and Clock Controller
#[derive(Copy, Clone)]
pub struct RCC {
  mem_addr: u32,
  cr: clock_control::ClockControl,
  cfgr: config::ConfigControl,
  enr: enable::PeripheralControl,
}

impl Control for RCC {
  unsafe fn mem_addr(&self) -> Volatile<u32> {
    Volatile::new(self.mem_addr as *const u32)
  }
}

impl RCC {
  fn rcc() -> Self {
    const RCC_ADDR: u32 = 0x4002_1000;
    RCC {
      mem_addr: RCC_ADDR,
      cr: clock_control::ClockControl::new(RCC_ADDR),
      cfgr: config::ConfigControl::new(RCC_ADDR),
      enr: enable::PeripheralControl::new(RCC_ADDR),
    }
  }

  /// Enable the specified clock
  pub fn enable_clock(&self, clock: Clock) {
    self.cr.enable_clock(clock);
  }

  /// Disable the specified clock, if the clock cannot be disabled (if it is driving the PLL for
  /// example) then this method will return false, it returns true otherwise
  pub fn disable_clock(&self, clock: Clock) -> bool {
    self.cr.disable_clock(clock)
  }

  /// Return true if the specified clock is enabled
  pub fn clock_is_on(&self, clock: Clock) -> bool {
    self.cr.clock_is_on(clock)
  }

  /// Return true if the specified clock is ready to be used as the system clock
  pub fn clock_is_ready(&self, clock: Clock) -> bool {
    self.cr.clock_is_ready(clock)
  }

  /// Return the clock driving the system clock
  pub fn get_system_clock_source(&self) -> Clock {
    self.cfgr.get_system_clock_source()
  }

  /// Set the system clock source. The system clock can only be run off of the HSI, HSE, PLL or
  /// HSI48 clocks, if another clock is specified the kernel will panic
  pub fn set_system_clock_source(&self, clock: Clock) {
    self.cfgr.set_system_clock_source(clock);
    // We need a memory barrier here since the hardware is writing to the system clock bit
    // the barrier ensures that the write to the control register takes effect before we
    // try to access the clock rate
    unsafe { dsb(); }
    clock_control::clock_rate::update_system_clock_rate();
  }

  /// Get the clock driving the PLL
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
    self.cfgr.get_pll_prediv_factor()
  }

  /// Set the PLL prediv factor, the factor specified MUST be within the range of [1..16], if it is
  /// outside that range the kernel will panic.
  pub fn set_pll_prediv_factor(&self, factor: u8) {
    self.cfgr.set_pll_prediv_factor(factor);
  }

  pub fn get_system_clock_rate(&self) -> u32 {
    clock_control::clock_rate::get_system_clock_rate()
  }

  /// Enable a peripheral
  pub fn enable_peripheral(&self, peripheral: Peripheral) {
    self.enr.enable_peripheral(peripheral);
  }

  pub fn disable_peripheral(&self, peripheral: Peripheral) {
    self.enr.disable_peripheral(peripheral);
  }

  pub fn peripheral_is_enabled(&self, peripheral: Peripheral) -> bool {
    self.enr.peripheral_is_enabled(peripheral)
  }
}


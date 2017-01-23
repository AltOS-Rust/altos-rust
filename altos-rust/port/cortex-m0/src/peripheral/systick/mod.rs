// peripheral/systick/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::{Control, Register};
use volatile::Volatile;

mod control_status;
mod reload_value;
mod current_value;

pub fn systick() -> SysTick {
  SysTick::systick()
}

#[derive(Copy, Clone)]
pub struct SysTick {
  mem_addr: u32,
  csr: control_status::CSR,
  rvr: reload_value::RVR,
  cvr: current_value::CVR,
}

impl Control for SysTick {
  unsafe fn mem_addr(&self) -> Volatile<u32> {
    Volatile::new(self.mem_addr as *const u32)
  }
}

impl SysTick {
  fn systick() -> Self {
    const SYSTICK_ADDR: u32 = 0xE000E010;
    SysTick {
      mem_addr: SYSTICK_ADDR,
      csr: control_status::CSR::new(SYSTICK_ADDR),
      rvr: reload_value::RVR::new(SYSTICK_ADDR),
      cvr: current_value::CVR::new(SYSTICK_ADDR),
    }
  }

  pub fn enable_counter(&self) {
    self.csr.set_enable(true);
  }

  pub fn disable_counter(&self) {
    self.csr.set_enable(false);
  }

  pub fn enable_interrupts(&self) {
    self.csr.set_interrupt(true);
  }

  pub fn disable_interrupts(&self) {
    self.csr.set_interrupt(false);
  }

  pub fn use_processor_clock(&self) {
    self.csr.set_source(control_status::ClockSource::Processor);
  }

  pub fn use_reference_clock(&self) {
    self.csr.set_source(control_status::ClockSource::Reference);
  }

  pub fn get_reload_value(&self) -> u32 {
    self.rvr.get_reload_value()
  }

  pub fn set_reload_value(&self, value: u32) {
    self.rvr.set_reload_value(value);
  }

  pub fn get_current_value(&self) -> u32 {
    self.cvr.get_current_value()
  }

  pub fn clear_current_value(&self) {
    self.cvr.clear_current_value();
  }

  pub fn did_underflow(&self) -> bool {
    self.csr.did_underflow()
  }
}

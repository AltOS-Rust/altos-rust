// peripheral/systick/reload_value.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::Register;

/// The Reload Value Register specifies the start value to load into the SYST_CVR (Current Value
/// Register)
#[derive(Copy, Clone)]
pub struct RVR {
  base_addr: u32,
}

impl Register for RVR {
  fn new(base_addr: u32) -> Self {
    RVR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x4
  }
}

impl RVR {
  /// Return the reload value of the register
  pub fn get_reload_value(&self) -> u32 {
    let mask = 0xFFFFFF;

    unsafe {
      let reg = self.addr();

      *reg & mask
    }
  }

  /// Set the reload value of the register, it must be <= 0xFFFFFF or the kernel will panic
  pub fn set_reload_value(&self, value: u32) {
    if value & !0xFFFFFF != 0 {
      // TODO: Figure out if we should panic or just mask away the top bits...
      panic!("RVR::set_reload_value - the value of the reload register must be <= 0xFFFFFF!");
    }

    unsafe {
      let mut reg = self.addr();

      reg.store(value);
    }
  }
}

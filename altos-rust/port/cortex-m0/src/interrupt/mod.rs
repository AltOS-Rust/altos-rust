// interrupt/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use peripheral::Control;
use volatile::Volatile;

mod enable;
mod pending;
mod priority;

#[derive(Copy, Clone)]
pub struct NVIC {
  mem_addr: u32,
  enable: enable::EnableControl,
  pending: pending::PendingControl,
  priority: priority::PriorityControl,
}

impl Control for NVIC {
  unsafe fn mem_addr(&self) -> Volatile<u32> {
    Volatile::new(self.mem_addr as *const u32)
  }
}

impl NVIC {
  pub fn nvic() -> Self {
    const NVIC_ADDR: u32 = 0xE000E100;
    NVIC {
      mem_addr: NVIC_ADDR,
      enable: enable::EnableControl::new(NVIC_ADDR),
      pending: pending::PendingControl::new(NVIC_ADDR),
      priority: priority::PriorityControl::new(NVIC_ADDR),
    }
  }
  
  pub fn enable_interrupt(&self, interrupt: u8) {
    self.enable.enable_interrupt(interrupt);
  }

  pub fn disable_interrupt(&self, interrupt: u8) {
    self.enable.disable_interrupt(interrupt);
  }

  pub fn interrupt_is_enabled(&self, interrupt: u8) -> bool {
    self.enable.interrupt_is_enabled(interrupt)
  }

  pub fn set_pending(&self, interrupt: u8) {
    self.pending.set_pending(interrupt);
  }

  pub fn clear_pending(&self, interrupt: u8) {
    self.pending.clear_pending(interrupt);
  }

  pub fn interrupt_is_pending(&self, interrupt: u8) -> bool {
    self.pending.interrupt_is_pending(interrupt)
  }

  pub fn set_priority(&self, priority: priority::Priority, interrupt: u8) {
    self.priority.set_priority(priority, interrupt);
  }

  pub fn get_priority(&self, interrupt: u8) -> priority::Priority {
    self.priority.get_priority(interrupt)
  }
}

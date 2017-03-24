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

//! This module defines interrupt behavior.

use peripheral::Control;
use volatile::Volatile;
pub use interrupt::defs::Hardware;
pub use self::priority::Priority;

mod defs;
mod enable;
mod pending;
mod priority;

/// Get an instance of the nested vector interrupt control.
pub fn nvic() -> NVIC {
    NVIC::nvic()
}

/// Controls the interrupt vectors for enabling/disabling interrupts for the peripherals.
#[derive(Copy, Clone)]
pub struct NVIC {
    mem_addr: *const u32,
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
    fn nvic() -> Self {
        const NVIC_ADDR: *const u32 = 0xE000E100 as *const _;
        NVIC {
            mem_addr: NVIC_ADDR,
            enable: enable::EnableControl::new(NVIC_ADDR),
            pending: pending::PendingControl::new(NVIC_ADDR),
            priority: priority::PriorityControl::new(NVIC_ADDR),
        }
    }

    /// Enable the interrupt for the specified peripheral.
    pub fn enable_interrupt(&mut self, hardware: Hardware) {
        self.enable.enable_interrupt(hardware);
    }

    /// Disable the interrupt for the specified peripheral.
    pub fn disable_interrupt(&mut self, hardware: Hardware) {
        self.enable.disable_interrupt(hardware);
    }

    /// Check if the interrupt for the peripheral is enabled.
    pub fn interrupt_is_enabled(&self, hardware: Hardware) -> bool {
        self.enable.interrupt_is_enabled(hardware)
    }

    /// Cause an interrupt for the specified peripheral to be set pending.
    ///
    /// If the interrupt is enabled, the interrupt handler will be called.
    /// Otherwise, no interrupt will be generated until the interrupt is enabled
    /// for the specified peripheral.
    pub fn set_pending(&mut self, hardware: Hardware) {
        self.pending.set_pending(hardware);
    }

    /// Clear the pending interrupt for the specified peripheral.
    pub fn clear_pending(&mut self, hardware: Hardware) {
        self.pending.clear_pending(hardware);
    }

    /// Check if interrupt is pending for the specified peripheral.
    pub fn interrupt_is_pending(&self, hardware: Hardware) -> bool {
        self.pending.interrupt_is_pending(hardware)
    }

    /// Set the priority of the interrupt for the specified peripheral.
    pub fn set_priority(&mut self, priority: priority::Priority, hardware: Hardware) {
        self.priority.set_priority(priority, hardware);
    }

    /// Get the priority of the interrupt for the specified peripheral.
    pub fn get_priority(&self, hardware: Hardware) -> priority::Priority {
        self.priority.get_priority(hardware)
    }
}

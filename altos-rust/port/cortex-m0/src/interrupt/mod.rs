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

use peripheral::Control;
use volatile::Volatile;
pub use interrupt::defs::Hardware;
pub use self::priority::Priority;

mod defs;
mod enable;
mod pending;
mod priority;

pub fn nvic() -> NVIC {
    NVIC::nvic()
}

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

    pub fn enable_interrupt(&self, hardware: Hardware) {
        self.enable.enable_interrupt(hardware);
    }

    pub fn disable_interrupt(&self, hardware: Hardware) {
        self.enable.disable_interrupt(hardware);
    }

    pub fn interrupt_is_enabled(&self, hardware: Hardware) -> bool {
        self.enable.interrupt_is_enabled(hardware)
    }

    pub fn set_pending(&self, hardware: Hardware) {
        self.pending.set_pending(hardware);
    }

    pub fn clear_pending(&self, hardware: Hardware) {
        self.pending.clear_pending(hardware);
    }

    pub fn interrupt_is_pending(&self, hardware: Hardware) -> bool {
        self.pending.interrupt_is_pending(hardware)
    }

    pub fn set_priority(&self, priority: priority::Priority, hardware: Hardware) {
        self.priority.set_priority(priority, hardware);
    }

    pub fn get_priority(&self, hardware: Hardware) -> priority::Priority {
        self.priority.get_priority(hardware)
    }
}

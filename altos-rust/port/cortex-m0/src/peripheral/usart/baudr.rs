// Daniel Seitz and RJ Russell

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone)]
pub enum BaudRate {
    Hz4800,
    Hz9600,
    Hz19200,
    Hz57600,
    Hz115200,
}

#[derive(Copy, Clone)]
pub struct BRR {
    base_addr: *const u32,
}

impl Register for BRR {
    fn new(base_addr: *const u32) -> Self {
        BRR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        BRR_OFFSET
    }
}

impl BRR {
    pub fn set_baud_rate(&self, baud_rate: BaudRate, clock_rate: u32, over8: bool) {
        let mut rate = match baud_rate {
            BaudRate::Hz4800 => clock_rate/4_800,
            BaudRate::Hz9600 => clock_rate/9_600,
            BaudRate::Hz19200 => clock_rate/19_200,
            BaudRate::Hz57600 => clock_rate/57_600,
            BaudRate::Hz115200 => clock_rate/115_200,
        };

        if over8 {
            let mut low_bits = rate & DIV_MASK;
            low_bits = low_bits >> 1;
            rate &= !(DIV_MASK);
            rate |= low_bits;
        }

        unsafe {
            let mut reg = self.addr();
            reg.store(rate);
        }
    }
}

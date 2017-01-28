// Daniel Seitz and RJ Russell

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone)]
pub struct UsartBRR {
    brr: BRR,
}

pub enum BaudRate {
    Rate4800,
    Rate9600,
    Rate19200,
    Rate57600,
    Rate115200,
}

impl UsartBRR {
    pub fn new(base_addr: *const u32) -> Self {
        UsartBRR { brr: BRR::new(base_addr) }
    }

    pub fn set_baud_rate(&self, baud_rate: BaudRate, clock_rate: u32, over8: bool) {
        self.brr.set_baud_rate(baud_rate, clock_rate, over8);
    }
}

#[derive(Copy, Clone)]
struct BRR {
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
    fn set_baud_rate(&self, baud_rate: BaudRate, clock_rate: u32, over8: bool) {
        let mut rate = match baud_rate {
            BaudRate::Rate4800 => clock_rate/4_800,
            BaudRate::Rate9600 => clock_rate/9_600,
            BaudRate::Rate19200 => clock_rate/19_200,
            BaudRate::Rate57600 => clock_rate/57_600,
            BaudRate::Rate115200 => clock_rate/115_200,
        };

        if over8 {
            const MASK: u32 = 0b1111;
            let mut low_bits = rate & MASK;
            low_bits = low_bits >> 1;
            rate &= !(MASK);
            rate |= low_bits;
        }

        unsafe {
            let mut reg = self.addr();
            reg.store(rate)
        }
    }
}

// Daniel Seitz and RJ Russell

use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone)]
pub struct USART_BRR {
    brr: BRR,
}

pub enum Baud_Rate {
    Rate_4800,
    Rate_9600,
    Rate_19200,
    Rate_57600,
    Rate_115200,
}

impl USART_BRR {
    pub fn new(base_addr: u32) -> Self {
        USART_BRR { brr: BRR::new(base_addr) }
    }

    pub fn set_baud_rate(&self, rate: Baud_Rate, clock_rate: u32) {
        self.brr.set_baud_rate(rate, clock_rate);
    }
}

#[derive(Copy, Clone)]
struct BRR {
    base_addr: u32,
}

impl Register for BRR {
    fn new(base_addr: u32) -> Self {
        BRR { base_addr: base_addr }
    }

    fn base_addr(&self) -> u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        BRR_offset
    }
}

impl BRR {
    fn set_baud_rate(&self, br: Baud_Rate, clock_rate: u32) {
        let rate = match br {
            Baud_Rate::Rate_4800 => {
                clock_rate/4_800
            },
            Baud_Rate::Rate_9600 => {
                clock_rate/9_600
            },
            Baud_Rate::Rate_19200 => {
                clock_rate/19_200
            },
            Baud_Rate::Rate_57600 => {
                clock_rate/57_600
            },
            Baud_Rate::Rate_115200 => {
                clock_rate/115_200
            },
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= ZERO;
            reg.store(rate)
        }
    }
}

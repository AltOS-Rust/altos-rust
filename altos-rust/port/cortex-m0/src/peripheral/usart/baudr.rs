// Daniel Seitz and RJ Russell

use super::super::Register;

#[derive(Copy, Clone)]
pub struct USART_BRR {
    br: BRR,
}

impl USART_BRR {
    pub fn new(base_addr: u32) -> Self {
        USART_BRR { br: BRR::new(base_addr) }
    }

    pub fn set_baud_rate(&self /* arg?? */) {
        // Need to set baud rate...
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
        0x0C
    }
}

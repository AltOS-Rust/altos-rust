// Daniel Seitz and RJ Russell

use super::super::Register;

#[derive(Copy, Clone)]
pub struct USART_BRR {
    br: BRR,
}

impl USART_BRR {
    pub fn new(base_addr: usize) -> Self {
        USART_BRR { br: BRR::new(base_addr) }
    }

    pub fn set_baud_rate(&self /* arg?? */) {
        // Need to set baud rate...
    }
}

#[derive(Copy, Clone)]
struct BRR {
    base_addr: usize,
}

impl Register for BRR {
    fn new(base_addr: usize) -> Self {
        BRR { base_addr: base_addr }
    }

    fn base_addr(&self) -> usize {
        self.base_addr
    }

    fn mem_offset(&self) -> usize {
        0x0C
    }
}

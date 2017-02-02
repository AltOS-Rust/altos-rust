use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone)]
pub struct ISR {
    base_addr: *const u32,
}

impl Register for ISR {
    fn new(base_addr: *const u32) -> Self {
        ISR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        ISR_OFFSET
    }
}

impl ISR {
    pub fn get_txe(&self) -> bool {
        unsafe {
            *self.addr() & ISR_TXE != 0
        }
    }
}

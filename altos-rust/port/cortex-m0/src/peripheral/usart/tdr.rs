use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone)]
pub struct TDR {
    base_addr: *const u32,
}

impl Register for TDR {
    fn new(base_addr: *const u32) -> Self {
        TDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        TDR_OFFSET
    }
}

impl TDR {
    pub fn store(&self, byte: u8) {
        unsafe {
            let mut reg = self.addr();
            reg.store(byte as u32);
        }
    }
}

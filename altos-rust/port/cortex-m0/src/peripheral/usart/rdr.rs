use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone)]
pub struct RDR {
    base_addr: *const u32,
}

impl Register for RDR {
    fn new(base_addr: *const u32) -> Self {
        RDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        RDR_OFFSET
    }
}

impl RDR {
    pub fn store(&self, byte: u8) {
        unsafe {
            let mut reg = self.addr();
            reg.store(byte as u32);
        }
    }
}

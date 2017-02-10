use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct ICR {
    base_addr: *const u32,
}

impl Register for ICR {
    fn new(base_addr: *const u32) -> Self {
        ICR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        ICR_OFFSET
    }
}

impl ICR {
    pub fn clear_ore(&self) {
        unsafe {
            *self.addr() |= ICR_ORECF;
        }
    }

    pub fn clear_tc(&self) {
        unsafe {
            *self.addr() |= ICR_TCCF;
        }
    }
}

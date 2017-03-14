pub const SCB_ADDR: *const u32 = 0xE000_ED00 as *const _;

pub const ICSR_OFFSET: u32 = 0x04;
pub const PEND_SV_CLEAR: u32 = 0b1 << 27;
pub const PEND_SV_SET: u32 = 0b1 << 28;

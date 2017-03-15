pub const SCB_ADDR: *const u32 = 0xE000_ED00 as *const _;

pub const ICSR_OFFSET: u32 = 0x04;
pub const ICSR_PENDSVCLR: u32 = 0b1 << 27;
pub const ICSR_PENDSVSET: u32 = 0b1 << 28;

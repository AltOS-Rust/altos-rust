pub const SYSTICK_ADDR: *const u32 = 0xE000E010 as *const _;

// Control Status Register
pub const CSR_OFFSET: u32 = 0x00;
pub const ENABLE: u32 = 0b1 << 0;
pub const TICKINT: u32 = 0b1 << 1;
pub const CLKSOURCE: u32 = 0b1 << 2;
pub const COUNTFLAG: u32 = 0b1 << 16;

// Reload Value Register
pub const RVR_OFFSET: u32 = 0x04;
pub const RELOAD: u32 = 0xFFFFFF;

// Current Value Register
pub const CVR_OFFSET: u32 = 0x08;
pub const CURRENT: u32 = 0xFFFFFF;

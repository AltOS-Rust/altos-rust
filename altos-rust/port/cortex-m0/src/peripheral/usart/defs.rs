// Zero
pub const ZERO: u32 = 0;

// Base addresses for USART 1 and 2
pub const USART1_ADDR: u32 = 0x4001_3800;
pub const USART2_ADDR: u32 = 0x4000_4400;

// USART1 Mem. Offsets
pub const CR3_offset: u32 = 0x0000_0008;
pub const BRR_offset: u32 = 0x0000_000C;

// ------------------------------------
// USART1 - CR1 Bit definitions
// ------------------------------------
pub const CR1_offset: u32 = 0x0;
pub const CR1_UE:     u32 = 0b1;
pub const CR1_UESM:   u32 = 0b1 << 1;
pub const CR1_RE:     u32 = 0b1 << 2;
pub const CR1_TE:     u32 = 0b1 << 3;
pub const CR1_IDLEIE: u32 = 0b1 << 4;
pub const CR1_RXNEIE: u32 = 0b1 << 5;
pub const CR1_TCIE:   u32 = 0b1 << 6;
pub const CR1_TXEIE:  u32 = 0b1 << 7;
pub const CR1_PEIE:   u32 = 0b1 << 8;
pub const CR1_PS:     u32 = 0b1 << 9;
pub const CR1_PCE:    u32 = 0b1 << 10;
pub const CR1_WAKE:   u32 = 0b1 << 11;
pub const CR1_M0:     u32 = 0b1 << 12;
pub const CR1_MME:    u32 = 0b1 << 13;
pub const CR1_CMIE:   u32 = 0b1 << 14;
pub const CR1_OVER8:  u32 = 0b1 << 15;
// pub const CR1_DEDT: u32 = ??; // this is bits 16-20
// pub const CR1_DEAT: u32 = ??; // this is bits 21-25
pub const CR1_RTOIE:  u32 = 0b1 << 26;
pub const CR1_EOBIE:  u32 = 0b1 << 27;
pub const CR1_M1:     u32 = 0b1 << 28;
// Bits 29 - 31 are reserved and must be kept at reset value.

// ------------------------------------
// USART1 - CR2 bit definitions
// ------------------------------------
pub const CR2_offset: u32 = 0x0000_0004;

pub const CR2_STOP_BIT0: u32 = 0b1 << 12;
pub const CR2_STOP_BIT1: u32 = 0b1 << 13;


// ------------------------------------
// BRR
// ------------------------------------
// Value of 5000 for USARTDIV yields a 9600 Kb/s baud rate
pub const USARTDIV_9600: u32 = 5000;
// Value of 417 for USARTDIV yields roughly a 115200 Kb/s baud rate
pub const USARTDIV_115200: u32 = 416;

// Zero
const ZERO: usize = 0;

// Base addresses for USART 1 and 2
const USART1: usize = 0x4001_3800;
const USART2: usize = 0x4000_4400;

// Mem. Offsets
const CR2_offset: usize = 0x0000_0004;
const CR3_offset: usize = 0x0000_0008;
const BRR_offset: usize = 0x0000_000C;

// CR1 Bit definitions
const CR1_UE:     usize = 1;
const CR1_UESM:   usize = 1 << 1;
const CR1_RE:     usize = 1 << 2;
const CR1_TE:     usize = 1 << 3;
const CR1_IDLEIE: usize = 1 << 4;
const CR1_RXNEIE: usize = 1 << 5;
const CR1_TCIE:   usize = 1 << 6;
const CR1_TXEIE:  usize = 1 << 7;
const CR1_PEIE:   usize = 1 << 8;
const CR1_PS:     usize = 1 << 9;
const CR1_PCE:    usize = 1 << 10;
const CR1_WAKE:   usize = 1 << 11;
const CR1_M0:     usize = 1 << 12;
const CR1_MME:    usize = 1 << 13;
const CR1_CMIE:   usize = 1 << 14;
const CR1_OVER8:  usize = 1 << 15;
// const CR1_DEDT: usize = ??; // this is bits 16-20
// const CR1_DEAT: usize = ??; // this is bits 21-25
const CR1_RTOIE:  usize = 1 << 26;
const CR1_EOBIE:  usize = 1 << 27;
const CR1_M1:     usize = 1 << 28;
// Bits 29 - 31 are reserved and must be kept at reset value.

const USARTDIV = 5000;

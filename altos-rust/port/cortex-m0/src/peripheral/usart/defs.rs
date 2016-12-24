// Zero
const ZERO: usize: 0x0000_0000;

// Base addresses for USART 1 and 2
const USART1: usize = 0x4001_3800;
const USART2: usize = 0x4000_4400;

// Mem. Offsets
const CR2_off: usize = 0x0000_0004;
const CR3_off: usize = 0x0000_0008;

// CR1 Bit definitions
const CR1_UE: usize = 0x0000_0001;
const CR1_RE: usize = 0x0000_0004;
const CR1_TE: usize = 0x0000_0008;
const M0: usize = 0x0000_1000;
const M1: usize = 0x1000_0000;

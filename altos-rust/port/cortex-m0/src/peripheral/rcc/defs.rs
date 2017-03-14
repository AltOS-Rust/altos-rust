pub const RCC_ADDR: *const u32 = 0x4002_1000 as *const _;

pub const CFGR_OFFSET: u32 = 0x04;

// AHB Peripherals
pub const AHBENR_OFFSET: u32 = 0x14;
pub const TSCEN: u32 = 0b1 << 24;
pub const IOPAEN: u32 = 0b1 << 17;
pub const IOPBEN: u32 = 0b1 << 18;
pub const IOPCEN: u32 = 0b1 << 19;
pub const IOPFEN: u32 = 0b1 << 22;
pub const CRCEN: u32 = 0b1 << 6;
pub const FLITFEN: u32 = 0b1 << 4;
pub const SRAMEN: u32 = 0b1 << 2;
pub const DMAEN: u32 = 0b1 << 0;
pub const DMA2EN: u32 = 0b1 << 1;

// APB1 Peripherals
pub const APBENR1_OFFSET: u32 = 0x1C;
pub const CECEN: u32 = 0b1 << 30;
pub const DACEN: u32 = 0b1 << 29;
pub const PWREN: u32 = 0b1 << 28;
pub const CRSEN: u32 = 0b1 << 27;
pub const CANEN: u32 = 0b1 << 25;
pub const USBEN: u32 = 0b1 << 23;
pub const I2C1EN: u32 = 0b1 << 21;
pub const I2C2EN: u32 = 0b1 << 22;
pub const USART2EN: u32 = 0b1 << 17;
pub const USART3EN: u32 = 0b1 << 18;
pub const USART4EN: u32 = 0b1 << 19;
pub const USART5EN: u32 = 0b1 << 20;
pub const SPI2EN: u32 = 0b1 << 14;
pub const WWDGEN: u32 = 0b1 << 11;
pub const TIM2EN: u32 = 0b1 << 0;
pub const TIM3EN: u32 = 0b1 << 1;
pub const TIM6EN: u32 = 0b1 << 4;
pub const TIM7EN: u32 = 0b1 << 5;
pub const TIM14EN: u32 = 0b1 << 8;

// APB2 Peripherals
pub const APBENR2_OFFSET: u32 = 0x18;
pub const DBGMCUEN: u32 = 0b1 << 22;
pub const TIM1EN: u32 = 0b1 << 11;
pub const TIM15EN: u32 = 0b1 << 16;
pub const TIM16EN: u32 = 0b1 << 17;
pub const TIM17EN: u32 = 0b1 << 18;
pub const USART1EN: u32 = 0b1 << 14;
pub const USART6EN: u32 = 0b1 << 5;
pub const USART7EN: u32 = 0b1 << 6;
pub const USART8EN: u32 = 0b1 << 7;
pub const SPI1EN: u32 = 0b1 << 12;
pub const ADCEN: u32 = 0b1 << 9;
pub const SYSCFGCOMPEN: u32 = 0b1 << 0;

pub const CR_OFFSET: u32 = 0x00;
pub const HSI_VALUE: u32 = 8_000_000;
pub const HSE_VALUE: u32 = 8_000_000;
pub const HSI48_VALUE: u32 = 48_000_000;

pub const HSION: u32 = 0b1 << 0;
pub const HSIRDY: u32 = 0b1 << 1;
pub const HSEON: u32 = 0b1 << 16;
pub const HSERDY: u32 = 0b1 << 17;
pub const PLLON: u32 = 0b1 << 24;
pub const PLLRDY: u32 = 0b1 << 25;

pub const CR2_OFFSET: u32 = 0x34;
pub const CR2_HSI14ON: u32 = 0b1 << 0;
pub const CR2_HSI14RDY: u32 = 0b1 << 1;
pub const CR2_HSI48ON: u32 = 0b1 << 16;
pub const CR2_HSI48RDY: u32 = 0b1 << 17;

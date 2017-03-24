/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

pub const RCC_ADDR: *const u32 = 0x4002_1000 as *const _;

pub const CR_OFFSET: u32 = 0x00;
// TODO: Preface these with CR
pub const HSI_VALUE: u32 = 8_000_000;
pub const HSE_VALUE: u32 = 8_000_000;
pub const HSI48_VALUE: u32 = 48_000_000;

pub const HSION: u32 = 0b1 << 0;
pub const HSIRDY: u32 = 0b1 << 1;
pub const HSEON: u32 = 0b1 << 16;
pub const HSERDY: u32 = 0b1 << 17;
pub const PLLON: u32 = 0b1 << 24;
pub const PLLRDY: u32 = 0b1 << 25;

// CFGR Bit Offsets
pub const CFGR_OFFSET: u32 = 0x04;
pub const CFGR_CLOCK_HSI: u32 = 0b00;
pub const CFGR_CLOCK_HSE: u32 = 0b01;
pub const CFGR_CLOCK_PLL: u32 = 0b10;
pub const CFGR_CLOCK_HSI48: u32 = 0b11;

pub const CFGR_SWS_MASK: u32 = 0b11 << 2;
pub const CFGR_SWS_HSI: u32 = CFGR_CLOCK_HSI << 2;
pub const CFGR_SWS_HSE: u32 = CFGR_CLOCK_HSE << 2;
pub const CFGR_SWS_PLL: u32 = CFGR_CLOCK_PLL << 2;
pub const CFGR_SWS_HSI48: u32 = CFGR_CLOCK_HSI48 << 2;

pub const CFGR_SW_CLEAR_MASK: u32 = 0b11;
pub const CFGR_PLLSRC_MASK: u32 = 0b11 << 15;
pub const CFGR_PLLSRC_HSI_2: u32 = CFGR_CLOCK_HSI << 15;
pub const CFGR_PLLSRC_HSI_PREDIV: u32 = 0b01 << 15;
pub const CFGR_PLLSRC_HSE_PREDIV: u32 = 0b10 << 15;
pub const CFGR_PLLSRC_HSI48_PREDIV: u32 = CFGR_CLOCK_HSI48 << 15;

pub const CFGR_PLLMUL_MASK: u32 = 0b1111 << 18;

// AHBENR Bit Offsets
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

// CFGR2 Bit Offsets
pub const CFGR2_OFFSET: u32 = 0x2C;
pub const CFGR2_PREDIV_MASK: u32 = 0b1111;

// CR2 Bit Offsets
pub const CR2_OFFSET: u32 = 0x34;
pub const CR2_HSI14ON: u32 = 0b1 << 0;
pub const CR2_HSI14RDY: u32 = 0b1 << 1;
pub const CR2_HSI48ON: u32 = 0b1 << 16;
pub const CR2_HSI48RDY: u32 = 0b1 << 17;

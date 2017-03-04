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

/* This file contains the constants associated with the bit definitions
 * for the registers being used for the ADC driver.
 * This is not a complete listing, but all constants used throughout
 * the program are listed here. However, some bit definitions are listed
 * and unused, or not listed at all.
 */

// Base addresses for ADC
pub const ADC_ADDR: *const u32 = 0x4001_2400 as *const _;

// ------------------------------------
// ADC_ISR - ISR Bit definitions
// Interrupt and status register
// ------------------------------------
pub const ISR_OFFSET: u32 = 0x00;
pub const ISR_ADRDY:  u32 = 0b1;
pub const ISR_EOSMP:  u32 = 0b1 << 1;
pub const ISR_EOC:    u32 = 0b1 << 2;
pub const ISR_EOSEQ:  u32 = 0b1 << 3;
pub const ISR_OVR:    u32 = 0b1 << 4;
// Bits 5-6 are reserved
pub const ISR_AWD:    u32 = 0b1 << 7;

// ------------------------------------
// ADC_IER - IER Bit definitions
// Interrupt enable register
// ------------------------------------
pub const IER_OFFSET:   u32 = 0x04;
pub const IER_ADRDYIE:  u32 = 0b1;
pub const IER_EOSMPIE:  u32 = 0b1 << 1;
pub const IER_EOCIE:    u32 = 0b1 << 2;
pub const IER_EOSEQIE:  u32 = 0b1 << 3;
pub const IER_OVRIE:    u32 = 0b1 << 4;
// Bits 5-6 are reserved
pub const IER_AWDIE:    u32 = 0b1 << 7;

// ------------------------------------
// ADC_CR - CR Bit definitions
// Control register
// ------------------------------------
pub const CR_OFFSET:  u32 = 0x08;
pub const CR_ADEN:    u32 = 0b1;
pub const CR_ADDIS:   u32 = 0b1 << 1;
pub const CR_ADSTART: u32 = 0b1 << 2;
// Bit 3 is reserved
pub const CR_ADSTP:   u32 = 0b1 << 4;
pub const CR_ADCAL:   u32 = 0b1 << 31;

// ------------------------------------
// ADC_DR - ISR Bit definitions
// Data register
// ------------------------------------
pub const DR_OFFSET:  u32 = 0x40;

// Other registers...
// CFGR1: Configuration register 1
// CFGR2: Configuration register 2
// SMPR: Sampling time register
// TR: Watchdog threshold regsiter
// CHSELR: Channel selection register
// CCR: Common configuration register

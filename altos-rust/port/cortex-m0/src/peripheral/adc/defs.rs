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

// Base address for ADC register set
pub const ADC_ADDR: *const u32 = 0x4001_2400 as *const _;

// ------------------------------------
// ADC - CFGR1 bit definitions
// ------------------------------------

pub const CFGR1_OFFSET:  u32 = 0x0C;
pub const CFGR1_DMEAN:   u32 = 0b1;
pub const CFGR1_DMACFG:  u32 = 0b1 << 1;
pub const CFGR1_SCANDIR: u32 = 0b1 << 2;
// CFGR1_RES bits 4 - 3 select conversion resolutions.
pub const CFGR1_RES_12_BIT: u32 = 0x0 << 3;
pub const CFGR1_RES_10_BIT: u32 = 0x1 << 3;
pub const CFGR1_RES_8_BIT: u32 = 0x2 << 3;
pub const CFGR1_RES_6_BIT: u32 = 0x3 << 3;

pub const CFGR1_ALIGN: u32 = 0b1 << 5;

// CFGR1_EXTSEL bits 6 - 8 select external events to trigger the start of conversion
pub const CFGR1_EXTSEL_TRG0: u32 = 0x0 << 6;
pub const CFGR1_EXTSEL_TRG1: u32 = 0x1 << 6;
pub const CFGR1_EXTSEL_TRG2: u32 = 0x2 << 6;
pub const CFGR1_EXTSEL_TRG3: u32 = 0x3 << 6;
pub const CFGR1_EXTSEL_TRG4: u32 = 0x4 << 6;
pub const CFGR1_EXTSEL_TRG5: u32 = 0x5 << 6;
pub const CFGR1_EXTSEL_TRG6: u32 = 0x7 << 6;
pub const CFGR1_EXTSEL_TRG7: u32 = 0x8 << 6;
// Bit 9 is reserved and must be kept at reset value.

// CFGR1_EXTEN bits selects 1 of 4 external trigger polarity options.
pub const CFGR1_EXTEN_DSBL: u32 = 0b00 << 10;
pub const CFGR1_EXTEN_RISE: u32 = 0b01 << 10;
pub const CFGR1_EXTEN_FALL: u32 = 0b10 << 10;
pub const CFGR1_EXTEN_RF:   u32 = 0b11 << 10;

pub const CFGR1_OVRMOD:     u32 = 0b1 << 12;
pub const CFGR1_CONT:       u32 = 0b1 << 13;
pub const CFGR1_WAIT:       u32 = 0b1 << 14;
pub const CFGR1_AUTOFF:     u32 = 0b1 << 15;
pub const CFGR1_DISCEN:     u32 = 0b1 << 16;
// Bits 17 - 21 are reserved and must be kept at reset value.
pub const CFGR1_AWDSGL:     u32 = 0b1 << 22;
pub const CFGR1_AWDEN:     u32 = 0b1 << 23;
// Bits 24 - 25 are reserved and must be kept at reset value.

// CFGR1_AWDCH bits 26 - 30 select 1 of the 19 input channels to be guarded by the analog watchdog.
// This may be an unnecassary definition... but I put them in here for completeness.
pub const CFGR1_AWDCH_0:     u32 = 0x0 << 26;
pub const CFGR1_AWDCH_1:     u32 = 0x1 << 26;
pub const CFGR1_AWDCH_2:     u32 = 0x2 << 26;
pub const CFGR1_AWDCH_3:     u32 = 0x3 << 26;
pub const CFGR1_AWDCH_4:     u32 = 0x4 << 26;
pub const CFGR1_AWDCH_5:     u32 = 0x5 << 26;
pub const CFGR1_AWDCH_6:     u32 = 0x6 << 26;
pub const CFGR1_AWDCH_7:     u32 = 0x7 << 26;
pub const CFGR1_AWDCH_8:     u32 = 0x8 << 26;
pub const CFGR1_AWDCH_9:     u32 = 0x9 << 26;
pub const CFGR1_AWDCH_10:     u32 = 0x10 << 26;
pub const CFGR1_AWDCH_11:     u32 = 0x11 << 26;
pub const CFGR1_AWDCH_12:     u32 = 0x12 << 26;
pub const CFGR1_AWDCH_13:     u32 = 0x13 << 26;
pub const CFGR1_AWDCH_14:     u32 = 0x14 << 26;
pub const CFGR1_AWDCH_15:     u32 = 0x15 << 26;
pub const CFGR1_AWDCH_16:     u32 = 0x16 << 26;
pub const CFGR1_AWDCH_17:     u32 = 0x17 << 26;
pub const CFGR1_AWDCH_18:     u32 = 0x18 << 26;
// The remaining bit strings for for this range are reserved and must not be used.
// Bit 31 is reserved and must be kept at reset value.

// ------------------------------------
// ADC - CFGR2 bit definitions
// ------------------------------------

pub const CFGR2_OFFSET:         u32 = 0x10;
// Bits 0 - 29 are reserved and must be kept at reset value.

// CFGR2_CKMODE bits 30 - 31 select the ADC clock mode.
// Bit string 0b11 for this range is reserved
pub const CFGR2_CKMODE_ADCCLK:  u32 = 0x0 << 30;
pub const CFGR2_CKMODE_PCLK2:   u32 = 0x1 << 30;
pub const CFGR2_CKMODE_PCLK4:   u32 = 0x2 << 30;

// ------------------------------------
// ADC - SMPR bit definitions
// ------------------------------------

pub const SMPR_OFFSET:          u32 = 0x14;
// SMPR_SMP bits 0 - 2 select the sampling time that applies to all channels.
// Here, the variable suffix "1_5" means 1.5 ADC clock cycles and so forth.
pub const SMPR_SMP_1_5:          u32 = 0x0;
pub const SMPR_SMP_7_5:          u32 = 0x1;
pub const SMPR_SMP_13_5:         u32 = 0x2;
pub const SMPR_SMP_28_5:         u32 = 0x3;
pub const SMPR_SMP_41_5:         u32 = 0x4;
pub const SMPR_SMP_55_5:         u32 = 0x5;
pub const SMPR_SMP_71_5:         u32 = 0x6;
pub const SMPR_SMP_238_5:        u32 = 0x7;

// Bits 3 - 31 are reserved and must be kept at reset value.

// ------------------------------------
// ADC - CHSELR bit definitions
// ------------------------------------

pub const CHSELR_OFFSET:    u32 = 0x28;
// Bits 0 - 18 select the analog channels for conversion.
pub const CHSELR_0:          u32 = 0b1;
pub const CHSELR_1:          u32 = 0b1 << 1;
pub const CHSELR_2:          u32 = 0b1 << 2;
pub const CHSELR_3:          u32 = 0b1 << 3;
pub const CHSELR_4:          u32 = 0b1 << 4;
pub const CHSELR_5:          u32 = 0b1 << 5;
pub const CHSELR_6:          u32 = 0b1 << 6;
pub const CHSELR_7:          u32 = 0b1 << 7;
pub const CHSELR_8:          u32 = 0b1 << 8;
pub const CHSELR_9:          u32 = 0b1 << 9;
pub const CHSELR_10:          u32 = 0b1 << 10;
pub const CHSELR_11:          u32 = 0b1 << 11;
pub const CHSELR_12:          u32 = 0b1 << 12;
pub const CHSELR_13:          u32 = 0b1 << 13;
pub const CHSELR_14:          u32 = 0b1 << 14;
pub const CHSELR_15:          u32 = 0b1 << 15;
pub const CHSELR_16:          u32 = 0b1 << 16;
pub const CHSELR_17:          u32 = 0b1 << 17;
pub const CHSELR_18:          u32 = 0b1 << 18;
// Bits 19 - 31 are reserved and must be kept at reset value.

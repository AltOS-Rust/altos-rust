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

// Base address for ADC register set
pub const ADC_ADDR: *const u32 = 0x4001_2400 as *const _;

// ------------------------------------
// ADC - CFGR1 bit definitions
// ------------------------------------

pub const CFGR1_OFFSET:  u32 = 0x0C;
pub const CFGR1_DMEAN:   u32 = 0b1;
pub const CFGR1_DMACFG:  u32 = 0b1 << 1;
pub const CFGR1_SCANDIR: u32 = 0b1 << 2;
// CFGR1_RES bits 3 - 4 select conversion resolutions.
pub const CFGR1_RES_12_BIT: u32 = 0x0 << 3;
pub const CFGR1_RES_10_BIT: u32 = 0x1 << 3;
pub const CFGR1_RES_8_BIT: u32 = 0x2 << 3;
pub const CFGR1_RES_6_BIT: u32 = 0x3 << 3;

pub const CFGR1_ALIGN: u32 = 0b1 << 5;

// CFGR1_EXTSEL bits 6-8 select external events to trigger the start of conversion
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
// Bits 21 - 17 are reserved and must be kept at reset value.
pub const CFGR1_AWDSGL:     u32 = 0b1 << 22;
pub const CFGR1_AWDEN:     u32 = 0b1 << 23;
// Bits 25 - 24 are reserved and must be kept at reset value.

// CFGR1_AWDCH bits 26 - 30 select 1 of the 18 input channels to be guarded by the analog watchdog.
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
// Bit 9 is reserved and must be kept at reset value.

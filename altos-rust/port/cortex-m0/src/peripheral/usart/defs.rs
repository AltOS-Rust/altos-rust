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
 * for the registers being used.
 * This is not a complete listing, however, all constants used throughout
 * the program are listed here, there are bit definitions that are listed
 * and not being used, or not listed at all.
 */

// Base addresses for USART 1 and 2
pub const USART1_ADDR: *const u32 = 0x4001_3800 as *const _;
pub const USART2_ADDR: *const u32 = 0x4000_4400 as *const _;

// ------------------------------------
// USARTx - CR1 Bit definitions
// ------------------------------------
pub const CR1_OFFSET: u32 = 0x00;
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
// USARTx - CR2 bit definitions
// ------------------------------------
pub const CR2_OFFSET: u32 = 0x04;
// Bits 0 - 3 are reserved and must be kept at reset value.
pub const CR2_ADDM7:     u32 = 0b1 << 4;
pub const CR2_LBDL:      u32 = 0b1 << 5;
pub const CR2_LBDIE:     u32 = 0b1 << 6;
// Bit 7 is reserved and must be kept at reset value.
pub const CR2_LBCL:      u32 = 0b1 << 8;
pub const CR2_CPHA:      u32 = 0b1 << 9;
pub const CR2_CPOL:      u32 = 0b1 << 10;
pub const CR2_CLKEN:     u32 = 0b1 << 11;
pub const CR2_STOP_BIT0: u32 = 0b1 << 12;
pub const CR2_STOP_BIT1: u32 = 0b1 << 13;
pub const CR2_LINEN:     u32 = 0b1 << 14;
pub const CR2_SWAP:      u32 = 0b1 << 15;
pub const CR2_RXINV:     u32 = 0b1 << 16;
pub const CR2_TXINV:     u32 = 0b1 << 17;
pub const CR2_DATAINV:   u32 = 0b1 << 18;
pub const CR2_MSBFIRST:  u32 = 0b1 << 19;
pub const CR2_ABREN:     u32 = 0b1 << 20;
pub const CR2_ABRMOD0:   u32 = 0b1 << 21;
pub const CR2_ABRMOD1:   u32 = 0b1 << 22;
pub const CR2_RTOEN:     u32 = 0b1 << 23;
pub const CR2_ADD:       u32 = 0b1111 << 24; // This might need to change
pub const CR2_ADD1:      u32 = 0b1111 << 28; // This might need to change

// ------------------------------------
// USARTx - CR3 bit definitions
pub const CR3_OFFSET: u32 = 0x08;
pub const CR3_RTSE:   u32 = 0b1 << 8;
pub const CR3_CTSE:   u32 = 0b1 << 9;

// ------------------------------------
// USARTx - BRR bit definitions
// ------------------------------------
pub const BRR_OFFSET: u32 = 0x0C;
pub const DIV_MASK: u32   = 0b1111;

// ------------------------------------
// USARTx - GTPR bit definitions
// ------------------------------------
pub const GTPR_OFFSET: u32 = 0x10;

// ------------------------------------
// USARTx - ISR bit definitions
// ------------------------------------
pub const ISR_OFFSET: u32 = 0x1C;
pub const ISR_PE: u32     = 0b1;
pub const ISR_FE: u32     = 0b1 << 1;
pub const ISR_NF: u32     = 0b1 << 2;
pub const ISR_ORE: u32    = 0b1 << 3;
pub const ISR_IDLE: u32   = 0b1 << 4;
pub const ISR_RXNE: u32   = 0b1 << 5;
pub const ISR_TC: u32     = 0b1 << 6;
pub const ISR_TXE: u32    = 0b1 << 7;

// ------------------------------------
// USARTx - ICR bit definitions
// ------------------------------------
pub const ICR_OFFSET: u32 = 0x20;
pub const ICR_PECF: u32   = 0b1;
pub const ICR_FECF: u32   = 0b1 << 1;
pub const ICR_NCF: u32    = 0b1 << 2;
pub const ICR_ORECF: u32  = 0b1 << 3;
pub const ICR_IDLECF: u32 = 0b1 << 4;
// Bit 5 reserved. Must be kept at reset value.
pub const ICR_TCCF: u32 = 0b1 << 6;
// Bit 7 reserved. Must be kept at reset value.
pub const ICR_LBDCF: u32 = 0b1 << 8;
pub const ICR_CTSCF: u32 = 0b1 << 9;
// Bit 10 reserved. Must be kept at reset value.
pub const ICR_RTOCF: u32 = 0b1 << 11;
pub const ICR_EOBCF: u32 = 0b1 << 12;
// Bit 13 - 16 reserved. Must be kept at reset value.
pub const ICR_CMCF: u32 = 0b1 << 17;
// Bit 18 and 19 reserved. Must be kept at reset value.
pub const ICR_WUCF: u32 = 0b1 << 20;
// Bit 21 - 31 reserved. Must be kept at reset value.

// ------------------------------------
// USARTx - RDR bit definitions
// ------------------------------------
pub const RDR_OFFSET: u32 = 0x24;

// ------------------------------------
// USARTx - TDR bit definitions
// ------------------------------------
pub const TDR_OFFSET: u32 = 0x28;


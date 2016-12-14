// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/6/16

#![no_std]
#![feature(asm)]
#![feature(naked_functions)]

mod math;
mod mem;
pub mod asm;

pub use math::*;
pub use mem::*;


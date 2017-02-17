#![no_std]
#![feature(cfg_target_has_atomic)]
#![feature(const_fn)]

#[cfg(all(target_arch="arm", not(target_has_atomic="ptr")))]
extern crate cm0_atomic as atomic;

#[cfg(target_has_atomic="ptr")]
use core::sync::atomic as atomic;

pub mod spin;

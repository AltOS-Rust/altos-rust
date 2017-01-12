// timer.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! System time handling.
//!
//! This module helps keep track of the system time and how much time has passed.

use atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

static SYSTEM_TICKS: AtomicUsize = ATOMIC_USIZE_INIT;

/// Tick the system tick counter
///
/// This method should only be called by the system tick interrupt handler.
pub fn tick() {
  SYSTEM_TICKS.fetch_add(1, Ordering::Relaxed);
}

/// Return the number of ticks that have passed since the system started.
///
/// The ticks can overflow and wrap back to 0, so the value returned is not guaranteed to be
/// greater than the previous values.
pub fn get_tick() -> usize {
  SYSTEM_TICKS.load(Ordering::Relaxed)
}


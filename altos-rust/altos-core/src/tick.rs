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

//! System time handling.
//!
//! This module helps keep track of the system time and how much time has passed.

use atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};

static SYSTEM_TICKS: AtomicUsize = ATOMIC_USIZE_INIT;

/// Tick the system tick counter.
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

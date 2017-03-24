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

//! Synchronization primitives for the AltOS-Rust kernel.
//!
//! This module implements several synchronization primitives for the kernel as well as
//! applications that rely on the kernel. They are used to control access to shared resources
//! across threads in order to avoid any data races.

mod mutex;
mod spin;
mod critical;
mod condvar;

pub use self::mutex::{RawMutex, Mutex, MutexGuard};
pub use self::mutex::{LockResult, LockError, UnlockError};
pub use self::mutex::mutex_from_guard;
pub use self::spin::{SpinMutex, SpinGuard};
pub use self::critical::CriticalSection;
pub use self::condvar::CondVar;

// sync/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/1/16

//! Synchronization primitives for the AltOSRust kernel.
//!
//! This module implements several synchronization primitives for the kernel as well as
//! applications that rely on the kernel. They are used to control access to shared resources
//! across threads in order to avoid any data races.

mod mutex;
mod spin;
mod critical;
mod condvar;

pub use self::mutex::{Mutex, MutexGuard};
pub use self::mutex::mutex_from_guard;
pub use self::spin::{SpinMutex, SpinGuard};
pub use self::critical::CriticalSection;
pub use self::condvar::CondVar;

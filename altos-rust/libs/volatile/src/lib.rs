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

#![no_std]
#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(unsize)]
#![feature(coerce_unsized)]
#![deny(trivial_casts, trivial_numeric_casts)]

//! Volatile memory operations.
//!
//! This library contains wrappers around raw pointers to perform volatile memory operations. This
//! is mainly useful for memory mapped I/O. Writing to the peripheral memory addresses will
//! normally be optimized out by the compiler. The `Volatile` type wraps a memory address to
//! perform volatile operations in order to force the compiler to keep these memory accesses and
//! stores.
//!
//! The creation of a `Volatile` pointer is generally unsafe, but the actual operations that you
//! can perform on it are considered safe by Rust's standards. This is because the actual memory
//! operations are performed through the `Deref` and `DerefMut` traits, which are defined as safe
//! methods. It is important to remember that a `Volatile` pointer is nearly identical to a
//! primitive pointer, and so all dereferencing operations on one should be considered unsafe (even
//! if not enforced by the compiler).
//!
//! # Examples
//!
//! ```rust,no_run
//! use volatile::Volatile;
//!
//! const IO_ADDR: *const u32 = 0x4000_4400 as *const _;
//!
//! unsafe {
//!     let mut io_ptr = Volatile::new(IO_ADDR);
//!     // Some bit that we need to set for an IO operation
//!     *io_ptr |= 0b1 << 5;
//! }
//! ```
//!
//! On some embedded devices you may want to do something like wait for a certain amount of time to
//! pass measured by some amount of ticks.
//!
//! ```rust,no_run
//! // Some tick counter that may be updated by a hardware interrupt
//! static mut TICKS: usize = 0;
//!
//! while unsafe { TICKS < 10 } {/* wait for ticks to change */}
//! ```
//!
//! Normally, the Rust compilier would optimize this kind of operation into just an infinite
//! `loop`, since the value of `TICKS` can't change in a single threaded environment, but `TICKS`
//! could be updated by some hardware interrupt, so we want to keep reloading the value in order to
//! check it. So to get around this we can use a `Volatile` pointer to force the compiler to reload
//! the value every time through the loop.
//!
//! ```rust,no_run
//! use volatile::Volatile;
//!
//! static mut TICKS: usize = 0;
//!
//! unsafe {
//!     let ticks_ptr = Volatile::new(&TICKS);
//!     while *ticks_ptr < 10 {/* wait for ticks to change */}
//! }
//! ```
//!
//! Now the value of `TICKS` will be reloaded every time through the loop.
//!
//! Oftentimes when working with memory mapped peripherals, you will have a block of memory that
//! you want to be working on that contains control, status, and data registers for some hardware
//! peripheral. These are often best represented as structs with each of their registers as fields,
//! but without volatile operations, loads and stores to these memory addresses often get optimized
//! out by the compiler. To get around this you can use a `Volatile` pointer to point at the mapped
//! address and have it be represented as a struct of the correct type.
//!
//! ```rust,no_run
//! use volatile::Volatile;
//!
//! const USART_ADDR: *const Usart = 0x4000_4400 as *const _;
//! // For transmitting and receiving data over serial
//! #[repr(C)]
//! struct Usart {
//!     control_reg: u32,
//!     status_reg: u32,
//!     tx_data_reg: u32,
//!     rx_data_reg: u32,
//! }
//!
//! let recieved = unsafe {
//!     let mut usart_block = Volatile::new(USART_ADDR);
//!     // Set some bits, these will be hardware specific
//!     usart_block.control_reg |= 0b11 << 5;
//!
//!     while usart_block.status_reg & 0b1 << 7 == 0 {/*wait for hardware to set a bit*/}
//!
//!     // Transmit some data
//!     usart_block.tx_data_reg = 100;
//!
//!     while usart_block.status_reg & 0b1 << 6 == 0 {/*wait for hardware to set some other bit*/}
//!
//!     // Receive some data
//!     usart_block.rx_data_reg
//! };
//! ```
//!
//! Every field access to a pointed at struct will be considered volatile and so will not be
//! optimized out by the compiler.
//!
//! Just as with primitive pointers, `Volatile` pointers can be created from valid references
//! safely, though their use should still be considered unsafe.
//!
//! ```rust
//! # #![allow(dead_code)]
//! use volatile::Volatile;
//!
//! let x: u32 = 0;
//! let ptr = Volatile::from(&x);
//! ```

mod tests;

use core::fmt;
use core::hash;
use core::ops::*;
use core::marker::Unsize;
use core::intrinsics::{volatile_load, volatile_store};

/// A volatile pointer.
///
/// This type acts as a pointer that only uses volatile operations. Pointer arithmetic can be
/// performed on it, and it can be dereferenced to its raw type in order to perform volatile memory
/// operations. This is especially useful for I/O operations where writing and reading from memory
/// mapped I/O registers would normally be optimized out by the compiler.
///
/// The `Volatile` type has the same syntax as a primitive pointer, so all functions that you can
/// expect to use with a primitive pointer like `*const` or `*mut` can also be used with a
/// `Volatile` pointer.
///
/// # Examples
///
/// ```rust
/// use volatile::Volatile;
///
/// let value: i32 = 0;
/// unsafe {
///   let mut ptr = Volatile::new(&value);
///   let mask = 0x0F0F;
///   *ptr |= mask;
/// }
/// assert_eq!(value, 0x0F0F);
/// ```
///
/// `Volatile` pointers can also be used to point at whole structs, and any memory accesses into
/// that struct will also be considered volatile
///
/// ```rust
/// use volatile::Volatile;
/// use std::mem;
///
/// struct IODevice {
///     reg1: u32,
///     reg2: u32,
///     reg3: u32,
/// }
///
/// let io_device: IODevice = unsafe { mem::uninitialized() };
///
/// unsafe {
///     let mut ptr = Volatile::new(&io_device);
///     ptr.reg1 = 1;
///     ptr.reg2 = 2;
///     ptr.reg3 = 3;
///     assert_eq!(ptr.reg1, 1);
///     assert_eq!(ptr.reg2, 2);
///     assert_eq!(ptr.reg3, 3);
/// }
/// ```
///
/// # Safety
///
/// Because `Volatile` pointers are often used for memory mapped peripherals, the compiler can not
/// guarantee that the address pointed at is valid, so the creation of a `Volatile` pointer is
/// unsafe. If you have a valid reference to some object, you can safely create a `Volatile`
/// pointer to that object through the `From` trait implemented for references.
///
/// ```rust
/// use volatile::Volatile;
///
/// let x: u32 = 0;
/// let ptr = Volatile::from(&x);
/// ```
///
/// Be wary however that a `Volatile` pointer provides interior mutability, so creating a pointer
/// from a shared reference may break immutability guarantees if used improperly.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Volatile<T: ?Sized>(*const T);

impl<T: ?Sized> Volatile<T> {
    /// Create a new `Volatile` pointer.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// const IO_ADDR: *const u32 = 0x4100_2000 as *const _;
    ///
    /// unsafe {
    ///     let ptr = Volatile::new(IO_ADDR);
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// Because `Volatile` pointers are often used to point at essentially arbitrary memory
    /// addresses, the compiler can not guarantee that the pointed at address is valid, so the
    /// creation of a `Volatile` pointer is considered unsafe.
    pub unsafe fn new(ptr: *const T) -> Self {
        Volatile(ptr)
    }

    /// Return true if the pointer is null, false otherwise
    pub fn is_null(self) -> bool where T: Sized {
        (self.0).is_null()
    }

    /// Return the inner pointer.
    ///
    /// Future accesses to this inner pointer will not be volatile.
    pub fn as_ptr(self) -> *const T {
        self.0
    }

    /// Returns the inner pointer mutably.
    ///
    /// Future accesses to this inner pointer will not be volatile.
    pub fn as_mut(self) -> *mut T {
        self.0 as *mut T
    }

    /// Calculate the offset from the inner pointer, returning a new Volatile pointer.
    ///
    /// `count` is in units of T; e.g. a `count` of 3 represents a pointer offset of `3 *
    /// size_of::<T>()` bytes.
    ///
    /// # Safety
    ///
    /// Both the starting and resulting pointer must be either in bounds or one byte past the end
    /// of an allocated object. If either pointer is out of bounds or arithmetic overflow occurs
    /// then any further use of the returned value will result in undefined behavior.
    pub unsafe fn offset(self, count: isize) -> Self where T: Sized {
        Volatile::new((self.0).offset(count))
    }

    /// Store a value into the memory address pointed at.
    ///
    /// This operation is guaranteed to not be optimized out by the compiler, even if it believes
    /// that it will have no effect on the program.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let x: u32 = 0;
    /// unsafe {
    ///     let mut ptr = Volatile::new(&x);
    ///     ptr.store(0x1234);
    /// }
    ///
    /// assert_eq!(x, 0x1234);
    /// ```
    ///
    /// # Safety
    ///
    /// Storing to a `Volatile` pointer is equivalent to storing a value to a primitive raw pointer
    /// so the operation could potentially be performed on some shared data or even an invalid
    /// address.
    pub unsafe fn store(&mut self, rhs: T) where T: Sized {
        volatile_store(self.0 as *mut T, rhs);
    }

    /// Load a value from the memory address pointed at.
    ///
    /// This operation is guaranteed to not be optimized out by the compiler, even if it believes
    /// that it will have no effect on the program.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let x: u32 = 0xAAAA;
    /// unsafe {
    ///     let ptr = Volatile::new(&x);
    ///     assert_eq!(ptr.load(), 0xAAAA);
    /// }
    /// ```
    ///
    /// # Safety
    ///
    /// Loading from a `Volatile` pointer is equivalent to loading a value from a primitive raw
    /// pointer so the operation could potentially be performed on an invalid address.
    pub unsafe fn load(&self) -> T where T: Sized {
        volatile_load(self.0)
    }

    /// Perform a read-modify-write operation on the memory address pointed at.
    ///
    /// This operation is guaranteed to not be optimized out by the compiler, even if it believes
    /// that it will have no effect on the program. The value stored in the address pointed at will
    /// be loaded and passed into the given function before being written back out to memory.
    ///
    /// # Examples
    /// ```rust
    /// use volatile::Volatile;
    ///
    /// let x: u32 = 0;
    /// unsafe {
    ///     let mut ptr = Volatile::new(&x);
    ///     ptr.modify(|x| {
    ///         let old = *x;
    ///         if old == 0 {
    ///             *x = 100;
    ///         }
    ///         else {
    ///             *x = 200;
    ///         }
    ///     });
    /// }
    /// assert_eq!(x, 100);
    /// ```
    ///
    /// # Safety
    ///
    /// Modifying a `Volatile` pointer's contents involves both a load and store of the underlying
    /// raw pointer so it could be performed on some shared data or even on an invalid address.
    pub unsafe fn modify<F>(&mut self, f: F) where T: Sized, F: FnOnce(&mut T) {
        let mut value = volatile_load(self.0);
        f(&mut value);
        volatile_store(self.0 as *mut T, value);
    }
}

impl<'a, T: ?Sized + 'a> From<&'a T> for Volatile<T> {
    fn from(reference: &'a T) -> Self {
        unsafe { Volatile::new(reference) }
    }
}

impl<'a, T: ?Sized + 'a> From<&'a mut T> for Volatile<T> {
    fn from(reference: &'a mut T) -> Self {
        unsafe { Volatile::new(reference) }
    }
}

impl<T: ?Sized> Deref for Volatile<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // A bit of a hack to forcibly get Rust to reload the value from memory, we mark this
            // empty assemby block here as clobbering memory, so the compiler thinks that the
            // pointer we're about to load from may have been touched
            asm!("" ::: "memory" : "volatile");
            &*(self.0)
        }
    }
}

impl<T: ?Sized> DerefMut for Volatile<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            asm!("" ::: "memory" : "volatile");
            &mut *((self.0) as *mut _)
        }
    }
}

impl<T: ?Sized> hash::Hash for Volatile<T> where T: Sized {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (self.0).hash(state);
    }
}

impl<T> fmt::Pointer for Volatile<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.0, f)
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Volatile<U>> for Volatile<T> {}


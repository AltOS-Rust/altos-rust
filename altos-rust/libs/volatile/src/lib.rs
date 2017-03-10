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
#![deny(trivial_casts, trivial_numeric_casts)]

//! Volatile memory operations.
//!
//! This library contains wrappers around raw pointers to perform volatile memory operations. This
//! is mainly useful for memory mapped I/O. Writing to the peripheral memory addresses will
//! normally be optimized out by the compiler. The `Volatile` type wraps a memory address to
//! perform volatile operations in order to force the compiler to keep these memory accesses and
//! stores.

mod tests;

use core::ops::*;
use core::intrinsics::{volatile_load, volatile_store};
use core::mem::size_of;

/// A volatile pointer.
///
/// This type acts as a pointer that only uses volatile operations. Pointer arithmetic can be
/// performed on it, and it can be dereferenced to its raw type in order to perform volatile memory
/// operations. This is especially useful for I/O operations where writing and reading from memory
/// mapped I/O registers would normally be optimized out by the compiler.
///
/// # Examples
///
/// ```rust,no_run
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
#[derive(Copy, Clone)]
pub struct Volatile<T>(RawVolatile<T>);

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct RawVolatile<T>(*const T);

impl<T> Volatile<T> {
  /// Creates a new `Volatile` pointer.
  ///
  /// This is unsafe because the address could be potentially anywhere, and forcing a write to a
  /// memory address could cause undefined behavior if the wrong address is chosen.
  pub unsafe fn new(ptr: *const T) -> Self {
    Volatile(RawVolatile(ptr))
  }

  /// Returns the inner pointer.
  pub fn as_ptr(self) -> *const T {
    (self.0).0
  }

  /// Returns the inner pointer mutably.
  pub fn as_mut(self) -> *mut T {
    (self.0).0 as *mut T
  }

  pub unsafe fn offset(self, count: isize) -> Self {
    let base = self.as_ptr() as isize;
    // TODO: See https://github.com/rust-lang/rust/issues/39056
    //  Change this back to a regular multiply once this gets fixed so we have overflow checking
    let offset = count.wrapping_mul(size_of::<T>() as isize);
    let addr = (base + offset) as *const T;
    Volatile::new(addr)
  }
}

impl<T> Deref for Volatile<T> {
  type Target = RawVolatile<T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for Volatile<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/*** Raw Volatile Implementation ***/

impl<T: Copy> RawVolatile<T> {
  /// Stores a value into the address pointed at.
  pub unsafe fn store(&mut self, rhs: T) {
    volatile_store(self.0 as *mut T, rhs);
  }

  /// Loads a value from the address pointed at.
  pub unsafe fn load(&self) -> T {
    volatile_load(self.0)
  }
}

impl<T> Deref for RawVolatile<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.0 }
  }
}

impl<T> DerefMut for RawVolatile<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *(self.0 as *mut _) }
  }
}

impl<T: Add<Output=T> + Copy> Add<T> for RawVolatile<T> {
  type Output = T;

  fn add(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) + rhs
    }
  }
}

impl<T: Add<Output=T> + Copy> AddAssign<T> for RawVolatile<T> {
  fn add_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) + rhs);
    }
  }
}

impl<T: Sub<Output=T> + Copy> Sub<T> for RawVolatile<T> {
  type Output = T;

  fn sub(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) - rhs
    }
  }
}

impl<T: Sub<Output=T> + Copy> SubAssign<T> for RawVolatile<T> {
  fn sub_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) - rhs);
    }
  }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for RawVolatile<T> {
  type Output = T;

  fn mul(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) * rhs
    }
  }
}

impl<T: Mul<Output=T> + Copy> MulAssign<T> for RawVolatile<T> {
  fn mul_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) * rhs);
    }
  }
}

impl<T: Div<Output=T> + Copy> Div<T> for RawVolatile<T> {
  type Output = T;

  fn div(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) / rhs
    }
  }
}

impl<T: Div<Output=T> + Copy> DivAssign<T> for RawVolatile<T> {
  fn div_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) / rhs);
    }
  }
}

impl<T: Rem<Output=T> + Copy> Rem<T> for RawVolatile<T> {
  type Output = T;

  fn rem(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) % rhs
    }
  }
}

impl<T: Rem<Output=T> + Copy> RemAssign<T> for RawVolatile<T> {
  fn rem_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) % rhs);
    }
  }
}

/*** Bitwise Operators ***/

impl<T: BitAnd<Output=T> + Copy> BitAnd<T> for RawVolatile<T> {
  type Output = T;

  fn bitand(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) & rhs
    }
  }
}

impl<T: BitAnd<Output=T> + Copy> BitAndAssign<T> for RawVolatile<T> {
  fn bitand_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) & rhs);
    }
  }
}

impl<T: BitOr<Output=T> + Copy> BitOr<T> for RawVolatile<T> {
  type Output = T;

  fn bitor(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) | rhs
    }
  }
}

impl<T: BitOr<Output=T> + Copy> BitOrAssign<T> for RawVolatile<T> {
  fn bitor_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) | rhs);
    }
  }
}

impl<T: BitXor<Output=T> + Copy> BitXor<T> for RawVolatile<T> {
  type Output = T;

  fn bitxor(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) ^ rhs
    }
  }
}

impl<T: BitXor<Output=T> + Copy> BitXorAssign<T> for RawVolatile<T> {
  fn bitxor_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) ^ rhs);
    }
  }
}

impl<T: Shl<T, Output=T> + Copy> Shl<T> for RawVolatile<T> {
  type Output = T;

  fn shl(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) << rhs
    }
  }
}

impl<T: Shl<T, Output=T> + Copy> ShlAssign<T> for RawVolatile<T> {
  fn shl_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) << rhs);
    }
  }
}

impl<T: Shr<T, Output=T> + Copy> Shr<T> for RawVolatile<T> {
  type Output = T;

  fn shr(self, rhs: T) -> Self::Output {
    unsafe {
      volatile_load(self.0) >> rhs
    }
  }
}

impl<T: Shr<T, Output=T> + Copy> ShrAssign<T> for RawVolatile<T> {
  fn shr_assign(&mut self, rhs: T) {
    unsafe {
      volatile_store(self.0 as *mut T, volatile_load(self.0) >> rhs);
    }
  }
}

/*** Negation ***/

impl<T: Neg<Output=T> + Copy> Neg for RawVolatile<T> {
  type Output = T;

  fn neg(self) -> Self::Output {
    unsafe {
      -volatile_load(self.0)
    }
  }
}

impl<T: Not<Output=T> + Copy> Not for RawVolatile<T> {
  type Output = T;

  fn not(self) -> Self::Output {
    unsafe {
      !volatile_load(self.0)
    }
  }
}

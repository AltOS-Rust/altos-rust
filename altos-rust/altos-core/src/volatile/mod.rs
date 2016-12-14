// volatile/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! Volatile memory operations.
//!
//! This module contains wrappers around raw pointers to perform volatile memory operations. This
//! is mainly useful for memory mapped I/O. Writing to the peripheral memory addresses will
//! normally be optimized out by the compiler. The `Volatile` type wraps a memory address to
//! perform volatile operations in order to force the compiler to keep these memory accesses and
//! stores.

mod tests;

use core::ops::*;
use core::intrinsics::{volatile_load, volatile_store};

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
/// use altos_core::volatile::Volatile;
/// 
/// let mem_port: *const usize = 0x4000_2000 as *const usize; // Some memory mapped I/O port
/// unsafe {
///   let mut ptr = Volatile::new(mem_port);
///   let mask = 0x0F0F;
///   *ptr |= mask;
/// }
/// ```
#[derive(Copy, Clone)]
pub struct Volatile<T: Copy>(RawVolatile<T>);

#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct RawVolatile<T: Copy>(*const T);

impl<T: Copy> Volatile<T> {
  /// Creates a new `Volatile` pointer.
  ///
  /// This is unsafe because the address could be potentially anywhere, and forcing a write to a
  /// memory address could cause undefined behavior if the wrong address is chosen.
  pub unsafe fn new(ptr: *const T) -> Self {
    Volatile(RawVolatile::new(ptr))
  }

  /// Returns the inner pointer.
  pub fn as_ptr(self) -> *const T {
    (self.0).0
  }

  /// Returns the inner pointer mutably.
  pub fn as_mut(self) -> *mut T {
    (self.0).0 as *mut T
  }
}

impl<T: Copy> Deref for Volatile<T> {
  type Target = RawVolatile<T>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: Copy> DerefMut for Volatile<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T: Add<Output=T> + Copy> Add<T> for Volatile<T> where usize: Add<T, Output=usize> {
  type Output = Volatile<T>;

  fn add(self, rhs: T) -> Self::Output {
    Volatile(RawVolatile(((self.0).0 as usize + rhs) as *const T))
  }
}

impl<T: Add<Output=T> + Copy> AddAssign<T> for Volatile<T> where usize: Add<T, Output=usize> {
  fn add_assign(&mut self, rhs: T) {
    self.0 = RawVolatile(((self.0).0 as usize + rhs) as *const T);
  }
}

impl<T: Sub<Output=T> + Copy> Sub<T> for Volatile<T> where usize: Sub<T, Output=usize> { 
  type Output = Volatile<T>;

  fn sub(self, rhs: T) -> Self::Output {
    Volatile(RawVolatile(((self.0).0 as usize - rhs) as *const T))
  }
}

impl<T: Sub<Output=T> + Copy> SubAssign<T> for Volatile<T> where usize: Sub<T, Output=usize> {
  fn sub_assign(&mut self, rhs: T) {
    self.0 = RawVolatile(((self.0).0 as usize - rhs) as *const T);
  }
}

/*** Raw Volatile Implementation ***/

impl<T: Copy> RawVolatile<T> {
  fn new(ptr: *const T) -> Self {
    RawVolatile(ptr)
  }

  /// Stores a value into the address pointed at.
  pub unsafe fn store(&mut self, rhs: T) {
    volatile_store(self.0 as *mut T, rhs);
  }

  /// Loads a value from the address pointed at.
  pub unsafe fn load(&self) -> T {
    volatile_load(self.0 as *const T)
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



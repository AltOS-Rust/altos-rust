// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

#![feature(asm)]
#![feature(const_fn)]
#![no_std]

use core::cell::UnsafeCell;
use core::ops::{Add, Sub, BitAnd, BitOr, BitXor};

macro_rules! start_critical {
  ($var:ident) => {{
    unsafe {
      #![cfg(target_arch="arm")]
      asm!(
        concat!(
          "mrs $0, PRIMASK\n",
          "cpsid i\n")
        : "=r"($var)
        : /* no inputs */
        : /* no clobbers */
        : "volatile");
    }
  }}
}

macro_rules! end_critical {
  ($var:ident) => {{
    unsafe {
      #![cfg(target_arch="arm")]
      asm!("msr PRIMASK, $0"
        : /* no outputs */
        : "r"($var)
        : /* no clobbers */
        : "volatile");
    }
  }}
}

macro_rules! atomic {
  { $( $code:expr );*; } => {{
    let primask: u32;
    start_critical!(primask);
    $(
      $code;
    )*
    end_critical!(primask);
  }};
  { $last:expr } => {{
    let primask: u32;
    start_critical!(primask);
    let result = $last;
    end_critical!(primask);
    result
  }}
}

#[derive(Copy, Clone)]
pub enum Ordering {
  Relaxed,
  Release,
  Acquire,
  AcqRel,
  SeqCst,
  __Nonexhaustive,
}

pub type AtomicBool = Atomic<bool>;
pub type AtomicI8 = Atomic<i8>;
pub type AtomicU8 = Atomic<u8>;
pub type AtomicI16 = Atomic<i16>;
pub type AtomicU16 = Atomic<u16>;
pub type AtomicI32 = Atomic<i32>;
pub type AtomicU32 = Atomic<u32>;
pub type AtomicI64 = Atomic<i64>;
pub type AtomicU64 = Atomic<u64>;
pub type AtomicIsize = Atomic<isize>;
pub type AtomicUsize = Atomic<usize>;

pub const ATOMIC_BOOL_INIT: AtomicBool = AtomicBool::new(false);
pub const ATOMIC_I8_INIT: AtomicI8 = AtomicI8::new(0);
pub const ATOMIC_U8_INIT: AtomicU8 = AtomicU8::new(0);
pub const ATOMIC_I16_INIT: AtomicI16 = AtomicI16::new(0);
pub const ATOMIC_U16_INIT: AtomicU16 = AtomicU16::new(0);
pub const ATOMIC_I32_INIT: AtomicI32 = AtomicI32::new(0);
pub const ATOMIC_U32_INIT: AtomicU32 = AtomicU32::new(0);
pub const ATOMIC_I64_INIT: AtomicI64 = AtomicI64::new(0);
pub const ATOMIC_U64_INIT: AtomicU64 = AtomicU64::new(0);
pub const ATOMIC_ISIZE_INIT: AtomicIsize = AtomicIsize::new(0);
pub const ATOMIC_USIZE_INIT: AtomicUsize = AtomicUsize::new(0);

pub struct AtomicPtr<T> {
  p: UnsafeCell<*mut T>,
}

impl<T> Default for AtomicPtr<T> {
  fn default() -> Self {
    AtomicPtr::new(::core::ptr::null_mut())
  }
}

unsafe impl<T> Send for AtomicPtr<T> {}
unsafe impl<T> Sync for AtomicPtr<T> {}

impl<T> AtomicPtr<T> {
  pub const fn new(p: *mut T) -> Self {
    AtomicPtr { p: UnsafeCell::new(p) }
  }

  pub fn get_mut(&mut self) -> &mut *mut T {
    unsafe { &mut *self.p.get() }
  }

  pub fn into_inner(self) -> *mut T {
    unsafe { self.p.into_inner() }
  }

  pub fn load(&self, _order: Ordering) -> *mut T {
    atomic! {
      unsafe {
        *self.p.get()
      }
    }
  }

  pub fn store(&self, ptr: *mut T, _order: Ordering) {
    atomic! {
      unsafe {
        *self.p.get() = ptr;
      }
    }
  }

  pub fn swap(&self, ptr: *mut T, _order: Ordering) -> *mut T {
    atomic! {
      unsafe {
        ::core::mem::replace(&mut *self.p.get(), ptr)
      }
    }
  }

  pub fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
    match self.compare_exchange(current, new, order, order) {
      Ok(x) => x,
      Err(x) => x,
    }
  }

  pub fn compare_exchange(&self, 
                           current: *mut T, 
                           new: *mut T, 
                           _success: Ordering, 
                           _fail: Ordering)
                           -> Result<*mut T, *mut T> {
    atomic! {
      unsafe {
        let old = self.p.get();
        if *old == current {
          Ok(::core::mem::replace(&mut *old, new))
        }
        else {
          Err(*old)
        }
      }
    }
  }

  pub fn compare_exchange_weak(&self,
                                current: *mut T,
                                new: *mut T,
                                success: Ordering,
                                fail: Ordering)
                                -> Result<*mut T, *mut T> {
    self.compare_exchange(current, new, success, fail)
  }
}

impl AtomicBool {
  pub fn fetch_nand(&self, value: bool, _order: Ordering) -> bool {
    atomic! {
      unsafe {
        let old = self.data.get();
        let new = !(*old && value);
        ::core::mem::replace(&mut *old, new)
      }
    }
  }
}

pub struct Atomic<T> {
  data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for Atomic<T> {}

impl<T: Copy> Atomic<T> {
  pub const fn new(data: T) -> Self {
    Atomic { data: UnsafeCell::new(data) }
  }

  pub fn get_mut(&mut self) -> &mut T {
    unsafe { &mut *self.data.get() }
  }

  pub fn into_inner(self) -> T {
    unsafe { self.data.into_inner() }
  }

  pub fn load(&self, _order: Ordering) -> T {
    atomic! {
      unsafe {
        *self.data.get()
      }
    }
  }

  pub fn store(&self, data: T, _order: Ordering) {
    atomic! {
      unsafe {
        *self.data.get() = data;
      }
    }
  }

  pub fn swap(&self, data: T, _order: Ordering) -> T {
    atomic! {
      unsafe {
        ::core::mem::replace(&mut *self.data.get(), data)
      }
    }
  }
}

impl<T: Copy + Default> Default for Atomic<T> {
  fn default() -> Self {
    Atomic { data: UnsafeCell::new(T::default()) }
  }
}

impl<T: Copy + PartialOrd> Atomic<T> {
  pub fn compare_and_swap(&self, current: T, new: T, order: Ordering) -> T {
    match self.compare_exchange(current, new, order, order) {
      Ok(x) => x,
      Err(x) => x,
    }
  }

  pub fn compare_exchange(&self, 
                           current: T, 
                           new: T, 
                           _success: Ordering, 
                           _fail: Ordering) 
                           -> Result<T, T> {
    atomic! {
      unsafe {
        let old = self.data.get();
        if *old == current {
          Ok(::core::mem::replace(&mut *old, new))
        }
        else {
          Err(*old)
        }
      }
    }
  }

  pub fn compare_exchange_weak(&self, 
                                current: T, 
                                new: T, 
                                success: Ordering, 
                                fail: Ordering) 
                                -> Result<T, T> {
    self.compare_exchange(current, new, success, fail)
  }
}

impl<T: Copy + Add<Output=T>> Atomic<T> {
  pub fn fetch_add(&self, data: T, _order: Ordering) -> T {
    atomic! {
      unsafe {
        let old = self.data.get();
        let new = *old + data;
        ::core::mem::replace(&mut *old, new)
      }
    }
  }
}

impl<T: Copy + Sub<Output=T>> Atomic<T> {
  pub fn fetch_sub(&self, data: T, _order: Ordering) -> T {
    atomic! {
      unsafe {
        let old = self.data.get();
        let new = *old - data;
        ::core::mem::replace(&mut *old, new)
      }
    }
  }
}

impl<T: Copy + BitAnd<Output=T>> Atomic<T> {
  pub fn fetch_and(&self, data: T, _order: Ordering) -> T {
    atomic! {
      unsafe {
        let old = self.data.get();
        let new = *old & data;
        ::core::mem::replace(&mut *old, new)
      }
    }
  }

}

impl<T: Copy + BitOr<Output=T>> Atomic<T> {
  pub fn fetch_or(&self, data: T, _order: Ordering) -> T {
    atomic! {
      unsafe {
        let old = self.data.get();
        let new = *old | data;
        ::core::mem::replace(&mut *old, new)
      }
    }
  }

}

impl<T: Copy + BitXor<Output=T>> Atomic<T> {
  pub fn fetch_xor(&self, data: T, _order: Ordering) -> T {
    atomic! {
      unsafe {
        let old = self.data.get();
        let new = *old ^ data;
        ::core::mem::replace(&mut *old, new)
      }
    }
  }
}

#[cfg(test)]
mod tests {
  // As a side note, these operations are not actually atomic when compiled 
  // for anything other than ARM
  use super::{Atomic, Ordering};

  #[test]
  fn load() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.load(Ordering::SeqCst), 0);
  }

  #[test]
  fn store() {
    let atom: Atomic<usize> = Atomic::new(0);

    atom.store(1, Ordering::SeqCst);
    assert_eq!(atom.load(Ordering::SeqCst), 1);
  }

  #[test]
  fn swap() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.swap(1, Ordering::SeqCst), 0);
    assert_eq!(atom.load(Ordering::SeqCst), 1);
  }

  #[test]
  fn compare_and_swap() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.compare_and_swap(0, 1, Ordering::SeqCst), 0);
    assert_eq!(atom.load(Ordering::SeqCst), 1);

  }

  #[test]
  fn compare_and_swap_fail() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.compare_and_swap(1, 2, Ordering::SeqCst), 0);
    assert_eq!(atom.load(Ordering::SeqCst), 0);
  }

  #[test]
  fn compare_exchange() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst), Ok(0));
    assert_eq!(atom.load(Ordering::SeqCst), 1);
  }

  #[test]
  fn compare_exchange_fail() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.compare_exchange(1, 2, Ordering::SeqCst, Ordering::SeqCst), Err(0));
    assert_eq!(atom.load(Ordering::SeqCst), 0);
  }

  #[test]
  fn fetch_add() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.fetch_add(1, Ordering::SeqCst), 0);
    assert_eq!(atom.fetch_add(1, Ordering::SeqCst), 1);
    assert_eq!(atom.load(Ordering::SeqCst), 2);
  }

  #[test]
  fn fetch_sub() {
    let atom: Atomic<usize> = Atomic::new(10);

    assert_eq!(atom.fetch_sub(1, Ordering::SeqCst), 10);
    assert_eq!(atom.fetch_sub(4, Ordering::SeqCst), 9);
    assert_eq!(atom.load(Ordering::SeqCst), 5);
  }

  #[test]
  fn fetch_and() {
    let atom: Atomic<usize> = Atomic::new(0xFF);

    assert_eq!(atom.fetch_and(0xAA, Ordering::SeqCst), 0xFF);
    assert_eq!(atom.fetch_and(0xF, Ordering::SeqCst), 0xAA);
    assert_eq!(atom.load(Ordering::SeqCst), 0xA);
  }

  #[test]
  fn fetch_or() {
    let atom: Atomic<usize> = Atomic::new(0);

    assert_eq!(atom.fetch_or(0xAA, Ordering::SeqCst), 0x0);
    assert_eq!(atom.fetch_or(0x55, Ordering::SeqCst), 0xAA);
    assert_eq!(atom.load(Ordering::SeqCst), 0xFF);
  }

  #[test]
  fn fetch_xor() {
    let atom: Atomic<usize> = Atomic::new(0xAA);

    assert_eq!(atom.fetch_xor(0xFF, Ordering::SeqCst), 0xAA);
    assert_eq!(atom.fetch_xor(0xFF, Ordering::SeqCst), 0x55);
    assert_eq!(atom.load(Ordering::SeqCst), 0xAA);
  }
}

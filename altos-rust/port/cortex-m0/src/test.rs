// test.rs
// AltOSRust
//
// Created by Daniel Seitz on 1/27/17

use peripheral::{Register};
use std::ops::{Deref, DerefMut};
use std::boxed::Box;

pub struct MockRegister<T: Register> {
  addr: *mut u32,
  register: T,
}

impl<T: Register> MockRegister<T> {
  fn new(val: u32) -> Self {
    let temp_reg = Box::new(val);
    let ptr = Box::into_raw(temp_reg);
    let offset = T::new(0x0 as *const _).mem_offset() as isize;
    MockRegister {
      addr: ptr,
      register: unsafe { T::new(ptr.offset(-offset/4)) },
    }
  }

  pub fn register_value(&self) -> u32 {
    unsafe { *self.addr }
  }
}

impl<T: Register> Deref for MockRegister<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.register
  }
}

impl<T: Register> DerefMut for MockRegister<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.register
  }
}

impl<T: Register> Drop for MockRegister<T> {
  fn drop(&mut self) {
    unsafe { drop(Box::from_raw(self.addr)) };
  }
}

pub fn create_register<T: Register>() -> MockRegister<T> {
  MockRegister::new(0)
}

pub fn create_initialized_register<T: Register>(val: u32) -> MockRegister<T> {
    MockRegister::new(val)
}


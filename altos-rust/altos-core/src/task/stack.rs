// task/stack.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/22/16

use volatile::Volatile;
use super::args::Args;
use alloc::{self, heap};
use alloc::boxed::Box;
use arch;

#[repr(C)]
#[derive(Debug)]
pub struct Stack {
  ptr: *const usize,
  base: *const usize,
  depth: usize,
}

impl Stack {
  pub fn new(depth: usize) -> Self {
    let align = ::core::mem::align_of::<u8>();
    // UNSAFE: We're touching the allocation interface, but the stack keeps track of any memory
    // that gets allocated, when the stack is dropped it will free the memory.
    let ptr = unsafe { heap::allocate(depth, align) };
    if ptr.is_null() {
      alloc::oom();
    }

    Stack {
      // UNSAFE: We've allocated 'depth' size already successfuly, so this offset must be within
      // bounds
      ptr: unsafe { ptr.offset(depth as isize) } as *const usize,
      base: ptr as *const usize,
      depth: depth,
    }
  }

  pub fn initialize(&mut self, code: fn(&mut Args), args: &Box<Args>) {
    // UNSAFE: We're creating a volatile pointer to our stack, but we know that it must be valid if
    // the object was successfully created
    unsafe {
      let stack_ptr = self.ptr();
      self.ptr = arch::initialize_stack(stack_ptr, code, args) as *const usize;
    }
  }

  pub fn check_overflow(&self) -> bool {
    self.ptr <= self.base
  }

  pub fn depth(&self) -> usize { self.depth }

  unsafe fn ptr(&self) -> Volatile<usize> {
    Volatile::new(self.ptr)
  }
}

impl Drop for Stack {
  fn drop(&mut self) {
    let align = ::core::mem::align_of::<u8>();
    // UNSAFE: We're touching the allocation interface again, but we know this is the exact size
    // and location of the block of memory that we allocated
    unsafe {
      heap::deallocate(self.base as *mut _, self.depth, align);
    }
  }
}

/*
const GUARD: usize = 0xFACE1E55;
const NUM_GUARD_WORDS: usize = 1;

pub struct GuardedStack {
  inner: Stack,
}

impl GuardedStack {
  pub fn new(depth: usize) -> Self {
    let stack = GuardedStack { inner: Stack::new(depth + (NUM_GUARD_WORDS * 4)) };
    for i in 0..NUM_GUARD_WORDS {
      unsafe { 
        *(stack.inner.base as *mut usize).offset(i as isize) = GUARD;
      }
    }
    stack
  }

  pub fn check_overflow(&self) -> bool {
    for i in 0..NUM_GUARD_WORDS {
      unsafe {
        if *self.inner.base.offset(i as isize) != GUARD { return false }
      }
    }
    true
  }

  pub fn depth(&self) -> usize { self.inner.depth }

  pub unsafe fn ptr(&self) -> Volatile<usize> {
    Volatile::new(self.inner.ptr)
  }
}
*/

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn stack_allocates_correct_size() {
    let stack = Stack::new(1024);
    let size = stack.ptr as usize - stack.base as usize;

    assert_eq!(size, stack.depth);
  }

  #[test]
  fn check_stack_overflow_no_overflow() {
    let stack = Stack::new(1024);
    
    assert_not!(stack.check_overflow());
  }

  #[test]
  fn check_stack_overflow_yes_overflow() {
    let mut stack = Stack::new(1024);
    stack.ptr = unsafe { stack.ptr.offset(-1025) };

    assert!(stack.check_overflow());
  }
}

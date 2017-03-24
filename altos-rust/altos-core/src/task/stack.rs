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
            // UNSAFE: We've allocated 'depth' size already successfuly, so this offset must
            // be within bounds.
            ptr: unsafe { ptr.offset(depth as isize) } as *const usize,
            base: ptr as *const usize,
            depth: depth,
        }
    }

    pub fn initialize(&mut self, code: fn(&mut Args), args: &Box<Args>) {
        // UNSAFE: We're creating a volatile pointer to our stack, but we know that it must be
        // valid if the object was successfully created.
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
        // UNSAFE: We're touching the allocation interface again, but we know this is the exact
        // size and location of the block of memory that we allocated.
        unsafe {
            heap::deallocate(self.base as *mut _, self.depth, align);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_allocates_correct_size() {
        let stack = Stack::new(1024);
        let size = stack.ptr as usize - stack.base as usize;

        assert_eq!(size, stack.depth);
    }

    #[test]
    fn test_check_stack_overflow_no_overflow() {
        let stack = Stack::new(1024);

        assert_not!(stack.check_overflow());
    }

    #[test]
    fn test_check_stack_overflow_yes_overflow() {
        let mut stack = Stack::new(1024);
        stack.ptr = unsafe { stack.ptr.offset(-1025) };

        assert!(stack.check_overflow());
    }
}

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

#[no_mangle]
pub unsafe extern "C" fn __aeabi_memclr4(dest: *mut u8, n: usize) {
    memset(dest, 0, n);
}

unsafe fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

/*
#[no_mangle]
pub unsafe extern fn __aeabi_memclr4(dest: *mut u32, mut n: isize) {
  while n > 0 {
    n -= 1;
    *dest.offset(n) = 0;
  }
}

#[no_mangle]
// TODO: Implement this, right now we don't do any reallocations, so it should never get called,
//   but in the future we might want to do some memory reallocations
pub unsafe extern fn __aeabi_memmove(dest: *mut u8, src: *const u8, len: isize) {
  panic!("Don't Reallocate Memory yet!");
  //if dest.offset(0) >= src.offset(len)
}
*/

/*
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn memclr() {
    let mut block: [u32; 10] = [0xAAAAAAAA; 10];

    for i in 0..10 {
      assert_eq!(block[i], 0xAAAAAAAA);
    }

    unsafe { __aeabi_memclr4(block.as_mut_ptr(), 10) };

    for i in 0..10 {
      assert_eq!(block[i], 0x0);
    }
  }
}
*/

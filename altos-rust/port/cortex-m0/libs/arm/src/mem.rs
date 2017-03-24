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

// Clears the memory pointed at to 0.
#[no_mangle]
pub unsafe extern "C" fn __aeabi_memclr4(dest: *mut u8, n: usize) {
    memset(dest, 0, n);
}

#[no_mangle]
pub unsafe extern "aapcs" fn __aeabi_memcpy(dest: *mut u8, src: *const u8, n: usize) {
    memcpy(dest, src, n);
}

#[no_mangle]
pub unsafe extern "aapcs" fn __aeabi_memmove4(dest: *mut u8, src: *const u8, n: usize) {
    memmove(dest, src, n);
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32;
        }
        i += 1;
    }
    0
}

// Sets the memory pointed to the value passed in.
unsafe fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

unsafe fn memcpy(dest: *mut u8,
                 src: *const u8,
                 n: usize)
                 -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

unsafe fn memmove(dest: *mut u8,
                  src: *const u8,
                  n: usize)
                  -> *mut u8 {
    if src < dest as *const u8 {
        // copy from end
        let mut i = n;
        while i != 0 {
            i -= 1;
            *dest.offset(i as isize) = *src.offset(i as isize);
        }
    } else {
        // copy from beginning
        let mut i = 0;
        while i < n {
            *dest.offset(i as isize) = *src.offset(i as isize);
            i += 1;
        }
    }
    dest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memclr() {
        let mut block: [u8; 10] = [0xAA; 10];

        for i in 0..10 {
            assert_eq!(block[i], 0xAA);
        }

        unsafe { __aeabi_memclr4(block.as_mut_ptr(), 10) };

        for i in 0..10 {
            assert_eq!(block[i], 0x0);
        }
    }
}

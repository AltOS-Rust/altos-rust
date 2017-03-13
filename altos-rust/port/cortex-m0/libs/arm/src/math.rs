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


// This is for unsigned 64-bit multiplication.
#[no_mangle]
pub extern "C" fn __aeabi_lmul(a: u64, b: u64) -> u64 {
    let half_bits: u32 = 64 / 4;
    let lower_mask = !0 >> half_bits;
    let mut low = ((a as u32) & lower_mask).wrapping_mul((b as u32) & lower_mask);
    let mut t = low >> half_bits;
    low &= lower_mask;
    t += ((a as u32) >> half_bits).wrapping_mul((b as u32) & lower_mask);
    low += (t & lower_mask) << half_bits;
    let mut high = t >> half_bits;
    t = low >> half_bits;
    low &= lower_mask;
    t += ((b as u32) >> half_bits).wrapping_mul((a as u32) & lower_mask);
    low += (t & lower_mask) << half_bits;
    high += t >> half_bits;
    high += ((a as u32) >> half_bits).wrapping_mul((b as u32) >> half_bits);
    high = high.wrapping_add(((a >> 32) as u32).wrapping_mul((b as u32)).wrapping_add((a as u32).wrapping_mul(((b >> 32) as u32))));
    low as u64 | ((high as u64) << 32)
}

// This is for unsigned 32-bit division.
#[no_mangle]
pub extern "C" fn __aeabi_uidiv(num: u32, den: u32) -> u32 {
    __udivmodsi4(num, den, None)
}

// This is a for unsigned 32-bit mod/division.
#[no_mangle]
pub extern "C" fn __udivmodsi4(mut num: u32, mut den: u32, rem_p: Option<&mut u32>) -> u32 {
    let mut quot = 0;
    let mut qbit = 1;

    if den == 0 {
        return 0;
    }

    // left-justify denominator and count shift
    while den as i32 >= 0 {
        den <<= 1;
        qbit <<= 1;
    }

    while qbit != 0 {
        if den <= num {
            num -= den;
            quot += qbit;
        }
        den >>= 1;
        qbit >>= 1;
    }

    if let Some(rem) = rem_p {
        *rem = num;
    }
    quot
}

// This is a for unsigned 32-bit mod.
// Uses a special calling convention where the caller expects the
// return value to be in $r1.
#[cfg(target_arch="arm")]
#[no_mangle]
#[naked]
unsafe fn __aeabi_uidivmod() {
    asm!("push {lr}
        sub sp, sp, #4
        mov r2, sp
        bl __udivmodsi4
        ldr r1, [sp]
        add sp, sp, #4
        pop {pc}"
    );
    ::core::intrinsics::unreachable();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_even() {
        assert_eq!(10, __aeabi_uidiv(100, 10));
    }

    #[test]
    fn test_divide_uneven() {
        assert_eq!(10, __aeabi_uidiv(105, 10));
    }

    #[test]
    fn test_divide_denominator_bigger() {
        assert_eq!(0, __aeabi_uidiv(5, 10));
    }

    #[test]
    fn test_mod_even() {
        let mut rem: u32 = !0;
        assert_eq!(10, __udivmodsi4(100, 10, Some(&mut rem)));
        assert_eq!(0, rem);
    }

    #[test]
    fn test_mod_uneven() {
        let mut rem: u32 = !0;
        assert_eq!(10, __udivmodsi4(105, 10, Some(&mut rem)));
        assert_eq!(5, rem);
    }

    #[test]
    fn test_mod_denominator_bigger() {
        let mut rem: u32 = !0;
        assert_eq!(0, __udivmodsi4(5, 10, Some(&mut rem)));
        assert_eq!(5, rem);
    }

    #[test]
    fn test_multiply_bigger_first() {
        assert_eq!(100, __aeabi_lmul(20, 5));
    }

    #[test]
    fn test_multiply_bigger_second() {
        assert_eq!(100, __aeabi_lmul(5, 20));
    }
}

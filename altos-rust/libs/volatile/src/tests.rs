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

#![cfg(test)]

use super::Volatile;

#[test]
fn test_volatile_from_ref_is_safe() {
    let num = 0x1000;
    let _ = Volatile::from(&num);
}

#[test]
fn test_volatile_from_mut_ref_is_safe() {
    let mut num = 0x1000;
    let _ = Volatile::from(&mut num);
}

#[test]
fn test_ptr_positive_offset() {
    unsafe {
        let ptr = Volatile::new(100 as *const u32);
        assert_eq!(ptr.offset(5).as_ptr() as usize, 120);
    }
}

#[test]
fn test_ptr_negative_offset() {
    unsafe {
        let ptr = Volatile::new(100 as *const u32);
        assert_eq!(ptr.offset(-5).as_ptr() as usize, 80);
    }
}

#[test]
fn test_ptr_deref_offset() {
    unsafe {
        let array = [0; 10];
        let ptr = Volatile::new(array.as_ptr());
        *ptr.offset(5) += 100;
        assert_eq!(array[5], 100);
    }
}

#[test]
fn test_store_volatile() {
    let num = 0xFF00;
    unsafe {
        let mut volatile = Volatile::new(&num);
        volatile.store(0xFF);
    }
    assert_eq!(num, 0xFF);
}

#[test]
fn test_load_volatile() {
    let num = 0xFF00;
    unsafe {
        let volatile = Volatile::new(&num);
        assert_eq!(num, volatile.load());
    }
}

#[test]
fn test_modify_volatile() {
    let num = 0x00;
    unsafe {
        let mut volatile = Volatile::new(&num);
        volatile.modify(|x| {
            *x = 0xAAAA;
        });
    }
    assert_eq!(num, 0xAAAA);
}

#[test]
fn test_volatile_field_access() {
    struct Test {
        a: u32,
        b: u32,
    }
    let test = Test { a: 0, b: 0 };
    unsafe {
        let mut volatile = Volatile::new(&test);
        volatile.a = 0x1234;
        volatile.b = 0x5678;
    }
    assert_eq!(test.a, 0x1234);
    assert_eq!(test.b, 0x5678);
}

#[test]
fn test_assign_volatile_deref() {
    let num = 0x00;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile = 0xFFFF;
    }
    assert_eq!(num, 0xFFFF);
}

#[test]
fn test_add_assign_volatile_deref() {
    let num = 0xFF00;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile += 0xFF;
    }
    assert_eq!(num, 0xFFFF);
}

#[test]
fn test_add_volatile_deref() {
    let num = 0xFF00;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile + 0xFF
    };
    assert_eq!(num, 0xFF00);
    assert_eq!(num2, 0xFFFF);
}

#[test]
fn test_sub_assign_volatile_deref() {
    let num = 0xFF00;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile -= 0xFF00;
    }
    assert_eq!(num, 0x0000);
}

#[test]
fn test_sub_volatile_deref() {
    let num = 0xFF00;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile - 0xFF00
    };
    assert_eq!(num, 0xFF00);
    assert_eq!(num2, 0x0000);
}

#[test]
fn test_mul_assign_volatile_deref() {
    let num = 10;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile *= 10;
    }
    assert_eq!(num, 100);
}

#[test]
fn test_mul_volatile_deref() {
    let num = 10;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile * 10
    };
    assert_eq!(num, 10);
    assert_eq!(num2, 100);
}

#[test]
fn test_div_assign_volatile_deref() {
    let num = 100;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile /= 10;
    }
    assert_eq!(num, 10);
}

#[test]
fn test_div_volatile_deref() {
    let num = 100;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile / 10
    };
    assert_eq!(num, 100);
    assert_eq!(num2, 10);
}

#[test]
fn test_rem_assign_volatile_deref() {
    let num = 100;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile %= 10;
    }
    assert_eq!(num, 0);
}

#[test]
fn test_rem_volatile_deref() {
    let num = 100;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile % 10
    };
    assert_eq!(num, 100);
    assert_eq!(num2, 0);
}

#[test]
fn test_bitand_assign_volatile_deref() {
    let num = 0xF0;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile &= 0xF;
    }
    assert_eq!(num, 0x00);
}

#[test]
fn test_bitand_volatile_deref() {
    let num = 0xF0;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile & 0xF
    };
    assert_eq!(num, 0xF0);
    assert_eq!(num2, 0x00);
}

#[test]
fn test_bitor_assign_volatile_deref() {
    let num = 0xF0;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile |= 0xF;
    }
    assert_eq!(num, 0xFF);
}

#[test]
fn test_bitor_volatile_deref() {
    let num = 0xF0;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile | 0xF
    };
    assert_eq!(num, 0xF0);
    assert_eq!(num2, 0xFF);
}

#[test]
fn test_bitxor_assign_volatile_deref() {
    let num = 0xFF;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile ^= 0xF;
    }
    assert_eq!(num, 0xF0);
}

#[test]
fn test_bitxor_volatile_deref() {
    let num = 0xFF;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile ^ 0xF
    };
    assert_eq!(num, 0xFF);
    assert_eq!(num2, 0xF0);
}

#[test]
fn test_shl_assign_volatile_deref() {
    let num = 0b0001;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile <<= 3;
    }
    assert_eq!(num, 0b1000);
}

#[test]
fn test_shl_volatile_deref() {
    let num = 0b0001;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile << 3
    };
    assert_eq!(num, 0b0001);
    assert_eq!(num2, 0b1000);
}

#[test]
fn test_shr_assign_volatile_deref() {
    let num = 0b1000;
    unsafe {
        let mut volatile = Volatile::new(&num);
        *volatile >>= 3;
    }
    assert_eq!(num, 0b0001);
}

#[test]
fn test_shr_volatile_deref() {
    let num = 0b1000;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        *volatile >> 3
    };
    assert_eq!(num, 0b1000);
    assert_eq!(num2, 0b0001);
}

#[test]
fn test_neg_volatile_deref() {
    let num = 100;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        -(*volatile)
    };
    assert_eq!(num, 100);
    assert_eq!(num2, -100);
}

#[test]
fn test_not_volatile_deref() {
    let num: u16 = 0xFF;
    let num2 = unsafe {
        let volatile = Volatile::new(&num);
        !(*volatile)
    };
    assert_eq!(num, 0xFF);
    assert_eq!(num2, 0xFF00);
}

// tests.rs
// Volatile
//
// Created by Daniel Seitz on 11/30/16

#![cfg(test)]

use super::Volatile;

#[test]
fn add_assign_volatile_ptr() {
  unsafe {
    let mut ptr = Volatile::new(100 as *const _);
    ptr += 100;

    assert_eq!(ptr.as_ptr() as usize, 200);
  }
}

#[test]
fn add_volatile_ptr() {
  unsafe {
    let ptr = Volatile::new(100 as *const _);
    let ptr2 = ptr + 100;

    assert_eq!(ptr.as_ptr() as usize, 100);
    assert_eq!(ptr2.as_ptr() as usize, 200);
  }
}

#[test]
fn sub_assign_volatile_ptr() {
  unsafe {
    let mut ptr = Volatile::new(100 as *const _);
    ptr -= 50;

    assert_eq!(ptr.as_ptr() as usize, 50);
  }
}

#[test]
fn sub_volatile_ptr() {
  unsafe {
    let ptr = Volatile::new(100 as *const _);
    let ptr2 = ptr - 50;

    assert_eq!(ptr.as_ptr() as usize, 100);
    assert_eq!(ptr2.as_ptr() as usize, 50);
  }
}

#[test]
fn store_volatile() {
  let num = 0xFF00;
  unsafe {
    let mut volatile = Volatile::new(&num);
    volatile.store(0xFF);
  }
  assert_eq!(num, 0xFF);
}

#[test]
fn load_volatile() {
  let num = 0xFF00;
  unsafe {
    let volatile = Volatile::new(&num);
    assert_eq!(num, volatile.load());
  }
}

#[test]
fn add_assign_volatile_deref() {
  let num = 0xFF00;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile += 0xFF;
  }
  assert_eq!(num, 0xFFFF);
}

#[test]
fn add_volatile_deref() {
  let num = 0xFF00;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile + 0xFF
  };
  assert_eq!(num, 0xFF00);
  assert_eq!(num2, 0xFFFF);
}

#[test]
fn sub_assign_volatile_deref() {
  let num = 0xFF00;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile -= 0xFF00;
  }
  assert_eq!(num, 0x0000);
}

#[test]
fn sub_volatile_deref() {
  let num = 0xFF00;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile - 0xFF00
  };
  assert_eq!(num, 0xFF00);
  assert_eq!(num2, 0x0000);
}

#[test]
fn mul_assign_volatile_deref() {
  let num = 10;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile *= 10;
  }
  assert_eq!(num, 100);
}

#[test]
fn mul_volatile_deref() {
  let num = 10;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile * 10
  };
  assert_eq!(num, 10);
  assert_eq!(num2, 100);
}

#[test]
fn div_assign_volatile_deref() {
  let num = 100;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile /= 10;
  }
  assert_eq!(num, 10);
}

#[test]
fn div_volatile_deref() {
  let num = 100;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile / 10
  };
  assert_eq!(num, 100);
  assert_eq!(num2, 10);
}

#[test]
fn rem_assign_volatile_deref() {
  let num = 100;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile %= 10;
  }
  assert_eq!(num, 0);
}

#[test]
fn rem_volatile_deref() {
  let num = 100;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile % 10
  };
  assert_eq!(num, 100);
  assert_eq!(num2, 0);
}

#[test]
fn bitand_assign_volatile_deref() {
  let num = 0xF0;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile &= 0xF;
  }
  assert_eq!(num, 0x00);
}

#[test]
fn bitand_volatile_deref() {
  let num = 0xF0;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile & 0xF
  };
  assert_eq!(num, 0xF0);
  assert_eq!(num2, 0x00);
}

#[test]
fn bitor_assign_volatile_deref() {
  let num = 0xF0;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile |= 0xF;
  }
  assert_eq!(num, 0xFF);
}

#[test]
fn bitor_volatile_deref() {
  let num = 0xF0;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile | 0xF
  };
  assert_eq!(num, 0xF0);
  assert_eq!(num2, 0xFF);
}

#[test]
fn bitxor_assign_volatile_deref() {
  let num = 0xFF;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile ^= 0xF;
  }
  assert_eq!(num, 0xF0);
}

#[test]
fn bitxor_volatile_deref() {
  let num = 0xFF;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile ^ 0xF
  };
  assert_eq!(num, 0xFF);
  assert_eq!(num2, 0xF0);
}

#[test]
fn shl_assign_volatile_deref() {
  let num = 0b0001;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile <<= 3;
  }
  assert_eq!(num, 0b1000);
}

#[test]
fn shl_volatile_deref() {
  let num = 0b0001;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile << 3
  };
  assert_eq!(num, 0b0001);
  assert_eq!(num2, 0b1000);
}

#[test]
fn shr_assign_volatile_deref() {
  let num = 0b1000;
  unsafe {
    let mut volatile = Volatile::new(&num);
    *volatile >>= 3;
  }
  assert_eq!(num, 0b0001);
}

#[test]
fn shr_volatile_deref() {
  let num = 0b1000;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    *volatile >> 3
  };
  assert_eq!(num, 0b1000);
  assert_eq!(num2, 0b0001);
}

#[test]
fn neg_volatile_deref() {
  let num = 100;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    -(*volatile)
  };
  assert_eq!(num, 100);
  assert_eq!(num2, -100);
}

#[test]
fn not_volatile_deref() {
  let num: u16 = 0xFF;
  let num2 = unsafe {
    let volatile = Volatile::new(&num);
    !(*volatile)
  };
  assert_eq!(num, 0xFF);
  assert_eq!(num2, 0xFF00);
}

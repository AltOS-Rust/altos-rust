// math.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

#[no_mangle]
#[naked]
#[cfg(target_arch="arm")]
pub unsafe extern fn __aeabi_lmul(num1: u64, num2: u64) {
  // TODO: This function needs a good hard looking at...
  asm!(
    concat!(
    "push        {r4, lr}\n",
    "muls        r1, r2 \n",
    "muls        r3, r0\n", 
    "adds        r1, r1, r3\n",

    "lsrs        r3, r0, #16\n",
    "lsrs        r4, r2, #16\n",
    "muls        r3, r4\n",
    "adds        r1, r1, r3\n",

    "lsrs        r3, r0, #16\n",
    "uxth        r0, r0\n",
    "uxth        r2, r2\n",
    "muls        r3, r2\n",
    "muls        r4, r0\n",
    "muls        r0, r2\n",

    "movs        r2, #0\n",
    "adds        r3, r3, r4\n",
    "adcs        r2, r2\n",
    "lsls        r2, #16\n",
    "adds        r1, r1, r2\n",

    "lsls        r2, r3, #16\n",
    "lsrs        r3, #16\n",
    "adds        r0, r0, r2\n",
    "adcs        r1, r1, r3\n",
    "pop        {r4, pc}\n")
    : /* no outputs */
    : /* no inputs */
    : /* no clobbers */
    : "volatile");
}

#[cfg(not(target_arch="arm"))]
pub unsafe extern fn __aeabi_lmul(num1: u64, num2: u64) -> u64 {
  let mut res = 0;
  let (higher, mut lower) = if num1 > num2 { (num1, num2) } else { (num2, num1) };
  // Incredibly unoptimized...
  while lower > 0 {
    res += higher;
    lower -= 1;
  }
  res
}

#[no_mangle]
pub unsafe extern fn __aeabi_uidiv(num: u32, den: u32) -> u32 {
  __aeabi_uidivbase(num, den, false) 
}

#[no_mangle]
pub unsafe extern fn __aeabi_uidivmod(num: u32, den: u32) -> u32 {
  __aeabi_uidivbase(num, den, true)
}

fn __aeabi_uidivbase(mut num: u32, mut den: u32, modwanted: bool) -> u32 {
  let mut bit: u32 = 1;
  let mut res: u32 = 0;

  while den < num && bit != 0 && (den & (1<<31)) == 0 {
    den <<= 1;
    bit <<= 1;
  }
  while bit != 0 {
    if num >= den {
      num -= den;
      res |= bit;
    }
    bit >>= 1;
    den >>= 1;
  }
  if modwanted { num } else { res }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn divide_even() {
    unsafe {
      assert_eq!(10, __aeabi_uidiv(100, 10));
    }
  }

  #[test]
  fn divide_uneven() {
    unsafe {
      assert_eq!(10, __aeabi_uidiv(105, 10));
    }
  }

  #[test]
  fn divide_denominator_bigger() {
    unsafe {
      assert_eq!(0, __aeabi_uidiv(5, 10));
    }
  }

  #[test]
  fn mod_even() {
    unsafe {
      assert_eq!(0, __aeabi_uidivmod(100, 10));
    }
  }
  
  #[test]
  fn mod_uneven() {
    unsafe {
      assert_eq!(5, __aeabi_uidivmod(105, 10));
    }
  }
  
  #[test]
  fn mod_denominator_bigger() {
    unsafe {
      assert_eq!(5, __aeabi_uidivmod(5, 10));
    }
  }

  #[test]
  fn multiply_bigger_first() {
    unsafe {
      assert_eq!(100, __aeabi_lmul(20, 5));
    }
  }

  #[test]
  fn multiply_bigger_second() {
    unsafe {
      assert_eq!(100, __aeabi_lmul(5, 20));
    }
  }
}

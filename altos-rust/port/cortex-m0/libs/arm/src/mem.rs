// mem.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/6/16

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

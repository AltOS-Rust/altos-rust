// peripheral/gpio/afr.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::super::{Register, Field};

#[derive(Copy, Clone)]
pub enum AlternateFunction {
  Zero,
  One,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
}

impl Field for AlternateFunction {
  fn mask(&self) -> u32 {
    match *self {
      AlternateFunction::Zero => 0b0000,
      AlternateFunction::One => 0b0001,
      AlternateFunction::Two => 0b0010,
      AlternateFunction::Three => 0b0011,
      AlternateFunction::Four => 0b0100,
      AlternateFunction::Five => 0b0101,
      AlternateFunction::Six => 0b0110,
      AlternateFunction::Seven => 0b0111,
    }
  }
}

impl AlternateFunction {
  fn from_mask(mask: u32) -> Self {
    match mask {
      0b0000 => AlternateFunction::Zero,
      0b0001 => AlternateFunction::One,
      0b0010 => AlternateFunction::Two,
      0b0011 => AlternateFunction::Three,
      0b0100 => AlternateFunction::Four,
      0b0101 => AlternateFunction::Five,
      0b0110 => AlternateFunction::Six,
      0b0111 => AlternateFunction::Seven,
      _ => panic!("AlternateFunction::from_mask - mask was not a valid value!"),
    }
  }
}

#[derive(Copy, Clone)]
pub struct AlternateFunctionControl {
  afrl: AFRL,
  afrh: AFRH,
}

impl AlternateFunctionControl {
  pub fn new(base_addr: *const u32) -> Self {
    AlternateFunctionControl {
      afrl: AFRL::new(base_addr),
      afrh: AFRH::new(base_addr),
    }
  }

  pub fn set_function(&self, function: AlternateFunction, port: u8) {
    if port < 8 {
      self.afrl.set_function(function, port);
    }
    else {
      self.afrh.set_function(function, port);
    }
  }

  pub fn get_function(&self, port: u8) -> AlternateFunction {
    if port < 8 {
      self.afrl.get_function(port)
    }
    else {
      self.afrh.get_function(port)
    }
  }
}

#[derive(Copy, Clone)]
struct AFRL {
  base_addr: *const u32,
}

impl Register for AFRL {
  fn new(base_addr: *const u32) -> Self {
    AFRL { base_addr: base_addr }
  }

  fn base_addr(&self) -> *const u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x20
  }
}

impl AFRL {
  fn set_function(&self, function: AlternateFunction, port: u8) {
    if port > 8 {
      panic!("AFRL::set_function - specified port must be between [0..7]!");
    }
    let mask = function.mask();

    unsafe {
      let mut reg = self.addr();

      *reg &= !(0b1111 << (port * 4));
      *reg |= mask << (port * 4);
    }
  }

  fn get_function(&self, port: u8) -> AlternateFunction {
    if port > 8 {
      panic!("AFRL::get_function - specified port must be between [0..7]!");
    }

    let mask = unsafe {
      let reg = self.addr();

      *reg & (0b1111 << (port * 4))
    };
    AlternateFunction::from_mask(mask)
  }
}

#[derive(Copy, Clone)]
struct AFRH {
  base_addr: *const u32,
}

impl Register for AFRH {
  fn new(base_addr: *const u32) -> Self {
    AFRH { base_addr: base_addr }
  }

  fn base_addr(&self) -> *const u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x24
  }
}

impl AFRH {
  fn set_function(&self, function: AlternateFunction, port: u8) {
    if port > 15 || port < 8 {
      panic!("AFRH::set_function - specified port must be between [8..15]!");
    }
    let mask = function.mask();

    // #9: Port needs to be subtracted by 8 since afr registers are split into high and low for 0-7
    // and 8-15. i.e. port 9 is actually offset 1 * 4 in the afrh register (rather than offset
    // 9 * 4)
    let port = port - 8;
    unsafe {
      let mut reg = self.addr();

      *reg &= !(0b1111 << (port * 4));
      *reg |= mask << (port * 4);
    }
  }

  fn get_function(&self, port: u8) -> AlternateFunction {
    if port > 15 || port < 8 {
      panic!("AFRL::get_function - specified port must be between [8..15]!");
    }

    // #9: See comment in `set_function`
    let port = port - 8;
    let mask = unsafe {
      let reg = self.addr();

      *reg & (0b1111 << (port * 4))
    };
    AlternateFunction::from_mask(mask)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test;

  #[test]
  fn test_afrh_set_function() {
    let afrh = test::create_register::<AFRH>();
    afrh.set_function(AlternateFunction::Five, 8);

    assert_eq!(afrh.register_value(), 0x5);
  }

  #[test]
  #[should_panic]
  fn test_afrh_set_port_out_of_bounds_panics() {
    let afrh = test::create_register::<AFRH>();
    afrh.set_function(AlternateFunction::Seven, 2);
  }

  #[test]
  fn test_afrl_set_function() {
    let afrl = test::create_register::<AFRL>();
    afrl.set_function(AlternateFunction::Two, 3);

    assert_eq!(afrl.register_value(), 0x2000);
  }

  #[test]
  #[should_panic]
  fn test_afrl_set_port_out_of_bounds_panics() {
    let afrl = test::create_register::<AFRL>();
    afrl.set_function(AlternateFunction::Two, 10);
  }
}

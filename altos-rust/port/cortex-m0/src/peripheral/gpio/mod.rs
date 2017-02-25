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

use super::{Control, Register};
use volatile::Volatile;
use super::rcc;
pub use self::port::Port;
pub use self::moder::Mode;
pub use self::otyper::Type;
pub use self::ospeedr::Speed;
pub use self::pupdr::Pull;
pub use self::afr::AlternateFunction;

mod port;
mod moder;
mod otyper;
mod bsrr;
mod ospeedr;
mod pupdr;
mod afr;

/// An IO group containing up to 16 pins. For
/// some reason the datasheet shows the memory
/// for groups D and E as reserved, so for now
/// they are left out.
#[derive(Copy, Clone)]
pub enum Group {
  A,
  B,
  C,
  F,
}

/// A GPIO contains the base address for a
/// memory mapped GPIO group associated with
/// it.
#[derive(Copy, Clone)]
pub struct GPIO {
  mem_addr: *const u32,
  moder: moder::MODER,
  otyper: otyper::OTYPER,
  bsrr: bsrr::BSRR,
  ospeedr: ospeedr::OSPEEDR,
  pupdr: pupdr::PUPDR,
  afr: afr::AlternateFunctionControl,
}

impl Control for GPIO {
  unsafe fn mem_addr(&self) -> Volatile<u32> {
    Volatile::new(self.mem_addr as *const u32)
  }
}

impl GPIO {
  fn group(group: Group) -> GPIO {
    match group {
      Group::A => GPIO::new(0x4800_0000 as *const _),
      Group::B => GPIO::new(0x4800_0400 as *const _),
      Group::C => GPIO::new(0x4800_0800 as *const _),
      Group::F => GPIO::new(0x4800_1400 as *const _),
    }
  }

  fn new(mem_addr: *const u32) -> GPIO {
    GPIO {
      mem_addr: mem_addr,
      moder: moder::MODER::new(mem_addr),
      otyper: otyper::OTYPER::new(mem_addr),
      bsrr: bsrr::BSRR::new(mem_addr),
      ospeedr: ospeedr::OSPEEDR::new(mem_addr),
      pupdr: pupdr::PUPDR::new(mem_addr),
      afr: afr::AlternateFunctionControl::new(mem_addr),
    }
  }

  /// Enable a GPIO group, you must do this before you can set any
  /// pins within a group.
  ///
  /// Example Usage:
  /// ```
  ///   GPIO::enable(Group::B); // Enable IO group B (LED is pb3)
  /// ```
  pub fn enable(group: Group) {
    let rcc = rcc::rcc();

    // Get the register bit that should be set to enable this group
    let io_group = match group {
      Group::A => rcc::Peripheral::GPIOA,
      Group::B => rcc::Peripheral::GPIOB,
      Group::C => rcc::Peripheral::GPIOC,
      Group::F => rcc::Peripheral::GPIOF,
    };
    rcc.enable_peripheral(io_group);
  }

  fn set_mode(&self, mode: Mode, port: u8) {
    self.moder.set_mode(mode, port);
  }

  fn get_mode(&self, port: u8) -> Mode {
    self.moder.get_mode(port)
  }

  fn set_type(&self, p_type: Type, port: u8) {
    self.otyper.set_type(p_type, port);
  }

  fn get_type(&self, port: u8) -> Type {
    self.otyper.get_type(port)
  }

  fn set_bit(&self, port: u8) {
    self.bsrr.set(port);
  }

  fn reset_bit(&self, port: u8) {
    self.bsrr.reset(port);
  }

  fn set_speed(&self, speed: Speed, port: u8) {
    self.ospeedr.set_speed(speed, port);
  }

  fn get_speed(&self, port: u8) -> Speed {
    self.ospeedr.get_speed(port)
  }

  fn set_pull(&self, pull: Pull, port: u8) {
    self.pupdr.set_pull(pull, port);
  }

  fn get_pull(&self, port: u8) -> Pull {
    self.pupdr.get_pull(port)
  }

  fn set_function(&self, function: AlternateFunction, port: u8) {
    self.afr.set_function(function, port);
  }

  fn get_function(&self, port: u8) -> AlternateFunction {
    self. afr.get_function(port)
  }
}

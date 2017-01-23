// peripheral/gpio/port.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::{GPIO, Mode, Group, Type, Speed, Pull, AlternateFunction};

/// A specific GPIO port. You can modify the mode it is set to
/// and set the pin high or low with the .set() and .reset() methods
/// respectively.
///
/// Example Usage:
/// ```
///   let mut port = Port::new(3, Group::B); // The port to the user LED
///   port.set_mode(Mode::Output);
///   port.set_type(Type::PushPull);
///   port.set(); // Light's green!
/// ```
pub struct Port {
  group: Group,
  port: u8,
}

impl Port {
  /// Create a new port for the associated group. Ports are NOT thread safe, if you
  /// must ensure an atomic set of operations on a port use some kind of synchronization
  /// tool 
  // TODO: Create synchronization tool...
  pub fn new(port: u8, group: Group) -> Port {
    if port > 15 {
      //TODO: Handle this more gracefully hopefully
      panic!("Port::new - port must be a value between 0..15");
    }
    Port {
      group: group,
      port: port,
    }
  }

  pub fn set_mode(&mut self, mode: Mode) {
    let gpio = GPIO::group(self.group);
    gpio.set_mode(mode, self.port);
  }

  pub fn get_mode(&self) -> Mode {
    let gpio = GPIO::group(self.group);
    gpio.get_mode(self.port)
  }

  pub fn set_type(&mut self, p_type: Type) {
    let gpio = GPIO::group(self.group);
    gpio.set_type(p_type, self.port);
  }

  pub fn get_type(&self) -> Type {
    let gpio = GPIO::group(self.group);
    gpio.get_type(self.port)
  }

  pub fn set_speed(&mut self, speed: Speed) {
    let gpio = GPIO::group(self.group);
    gpio.set_speed(speed, self.port);
  }

  pub fn get_speed(&self) -> Speed {
    let gpio = GPIO::group(self.group);
    gpio.get_speed(self.port)
  }

  pub fn set_pull(&mut self, pull: Pull) {
    let gpio = GPIO::group(self.group);
    gpio.set_pull(pull, self.port);
  }

  pub fn get_pull(&self) -> Pull {
    let gpio = GPIO::group(self.group);
    gpio.get_pull(self.port)
  }

  pub fn set_function(&mut self, function: AlternateFunction) {
    let gpio = GPIO::group(self.group);
    gpio.set_function(function, self.port);
  }

  pub fn get_function(&self) -> AlternateFunction {
    let gpio = GPIO::group(self.group);
    gpio.get_function(self.port)
  }

  /// Sets the pin high.
  pub fn set(&self) {
    let gpio = GPIO::group(self.group);
    gpio.set_bit(self.port);
  }

  /// Sets the pin low.
  pub fn reset(&self) {
    let gpio = GPIO::group(self.group);
    gpio.reset_bit(self.port);
  }
}

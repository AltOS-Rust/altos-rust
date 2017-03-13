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
    pub fn new(port: u8, group: Group) -> Port {
        if port > 15 {
            panic!("Port::new - port must be a value between 0..15");
        }
        Port {
            group: group,
            port: port,
        }
    }

    /// Set the port mode.
    pub fn set_mode(&mut self, mode: Mode) {
        let mut gpio = GPIO::group(self.group);
        gpio.set_mode(mode, self.port);
    }

    /// Get the current port mode.
    pub fn get_mode(&self) -> Mode {
        let gpio = GPIO::group(self.group);
        gpio.get_mode(self.port)
    }

    /// Set the port type.
    pub fn set_type(&mut self, p_type: Type) {
        let mut gpio = GPIO::group(self.group);
        gpio.set_type(p_type, self.port);
    }

    /// Get the current port type.
    pub fn get_type(&self) -> Type {
        let gpio = GPIO::group(self.group);
        gpio.get_type(self.port)
    }

    /// Set the port pin speed.
    pub fn set_speed(&mut self, speed: Speed) {
        let mut gpio = GPIO::group(self.group);
        gpio.set_speed(speed, self.port);
    }

    /// Get the current port pin speed.
    pub fn get_speed(&self) -> Speed {
        let gpio = GPIO::group(self.group);
        gpio.get_speed(self.port)
    }

    /// Set behavior of GPIO pin when it is not asserted.
    pub fn set_pull(&mut self, pull: Pull) {
        let mut gpio = GPIO::group(self.group);
        gpio.set_pull(pull, self.port);
    }

    /// Get currently defined behavior of GPIO pin when not asserted.
    pub fn get_pull(&self) -> Pull {
        let gpio = GPIO::group(self.group);
        gpio.get_pull(self.port)
    }

    /// Set the function mode for the port.
    pub fn set_function(&mut self, function: AlternateFunction) {
        let mut gpio = GPIO::group(self.group);
        gpio.set_function(function, self.port);
    }

    /// Get the current function mode for the port.
    pub fn get_function(&self) -> AlternateFunction {
        let gpio = GPIO::group(self.group);
        gpio.get_function(self.port)
    }

    /// Set the pin high.
    pub fn set(&mut self) {
        let mut gpio = GPIO::group(self.group);
        gpio.set_bit(self.port);
    }

    /// Set the pin low.
    pub fn reset(&mut self) {
        let mut gpio = GPIO::group(self.group);
        gpio.reset_bit(self.port);
    }
}

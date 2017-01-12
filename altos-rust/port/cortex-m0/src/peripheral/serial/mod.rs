// peripheral/serial/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use super::Control;
use volatile::Volatile;
use peripheral::{gpio, rcc};

fn init() {
  let rcc = rcc::rcc();

  gpio::GPIO::enable(gpio::GPIOGroup::A);
  rcc.enable_peripheral(rcc::Peripheral::USART1);

  let pa9 = gpio::GPIOPort::new(9, gpio::GPIOGroup::A);
  let pa10 = gpio::GPIOPort::new(10, gpio::GPIOGroup::A);

  pa9.set_function(gpio::AlternateFunction::One);
  pa10.set_function(gpio::AlternateFunction::One);

  pa9.set_speed(gpio::Speed::High);
  pa10.set_speed(gpio::Speed::High);

  pa9.set_mode(gpio::Mode::Alternate);
  pa10.set_mode(gpio::Mode::Alternate);

  pa9.set_type(gpio::Type::PushPull);
  pa10.set_type(gpio::Type::PushPull);

  pa9.set_pull(gpio::Pull::Up);
  pa10.set_pull(gpio::Pull::Up);


}

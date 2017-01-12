use peripheral::{gpio, rcc};

pub fn gpio_init() {
    // TODO Need to set clocks for whole usart (and get freq for baud rate)
    // TODO Does this need to live here? Should the Peripheral be used
    //  for the USART struct as well? or should they be separate?
  let rcc = rcc::rcc();
  let cr = rcc.get_system_clock_rate();

  gpio::GPIO::enable(gpio::Group::A);
  rcc.enable_peripheral(rcc::Peripheral::USART1);

  let mut pa9 = gpio::Port::new(9, gpio::Group::A);
  let mut pa10 = gpio::Port::new(10, gpio::Group::A);

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

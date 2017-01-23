// peripheral/serial/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16
mod control;
mod defs;
mod baudr;

use super::Control;
use volatile::Volatile;
use self::control::UsartCR;
use self::baudr::UsartBRR;
use self::defs::*;
use peripheral::{gpio, rcc};

pub use self::control::{WordLength, Mode, Parity, Stoplength, HardwareFlowControl};
pub use self::baudr::BaudRate;

#[derive(Copy, Clone)]
pub enum USARTx {
    USART1,
    USART2,
}

pub struct USART {
    mem_addr: u32,
    control: UsartCR,
    baud: UsartBRR,
}

impl Control for USART {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr as *const u32)
    }
}

impl USART {
    pub fn new(x: USARTx) -> Self {
        match x {
            USARTx::USART1 => USART {
                mem_addr: USART1_ADDR,
                control: UsartCR::new(USART1_ADDR),
                baud: UsartBRR::new(USART1_ADDR),
            },
            USARTx::USART2 => USART {
                mem_addr: USART2_ADDR,
                control: UsartCR::new(USART2_ADDR),
                baud: UsartBRR::new(USART2_ADDR),
            },
        }
    }
}
//
// TODO
//pub fn init() {
//    let usart1: USART = USART::new(USARTx::USART1);
//}


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

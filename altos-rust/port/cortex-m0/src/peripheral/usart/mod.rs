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
use peripheral::rcc;

pub use self::control::{WordLength, Mode, Parity, Stoplength, HardwareFlowControl};
pub use self::baudr::BaudRate;

#[derive(Copy, Clone)]
pub enum USARTx {
    USART1,
    USART2,
}

pub struct USART {
    mem_addr: *const u32,
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

    // TODO: Change &self -> &mut self for all these?? Maybe??
    pub fn enable_usart(&self) {
        self.control.enable_usart();
    }

    pub fn disable_usart(&self) {
        self.control.disable_usart();
    }

    pub fn set_word_length(&self, length: WordLength) {
        self.control.set_word_length(length);
    }

    pub fn set_mode(&self, mode: Mode) {
        self.control.set_mode(mode);
    }

    pub fn set_parity(&self, parity: Parity) {
        self.control.set_parity(parity);
    }

    pub fn set_stop_bits(&self, length: Stoplength) {
        self.control.set_stop_bits(length);
    }

    pub fn enable_over8(&self) {
        self.control.enable_over8();
    }

    pub fn disable_over8(&self) {
        self.control.disable_over8();
    }

    pub fn set_hardware_flow_control(&self, hfc: HardwareFlowControl) {
        self.control.set_hardware_flow_control(hfc);
    }

    pub fn set_baud_rate(&self, baud_rate: BaudRate, clock_rate: u32) {
        self.baud.set_baud_rate(baud_rate, clock_rate, self.control.get_over8());
    }
}

pub fn init() {
    let rcc = rcc::rcc();
    rcc.enable_peripheral(rcc::Peripheral::USART1);
/* TODO: Turn this back on at some point in the future.
    gpio::GPIO::enable(gpio::Group::A);
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
*/
    let usart1 = USART::new(USARTx::USART1);
    usart1.disable_usart();
    usart1.set_word_length(WordLength::Eight);
    usart1.set_mode(Mode::All);
    usart1.set_parity(Parity::None);
    usart1.enable_over8();
    usart1.set_hardware_flow_control(HardwareFlowControl::None);


    let cr = rcc.get_system_clock_rate();
    usart1.set_baud_rate(BaudRate::Rate9600, cr);

    usart1.enable_usart();
}

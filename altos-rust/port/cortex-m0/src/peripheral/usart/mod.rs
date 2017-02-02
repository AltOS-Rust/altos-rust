// peripheral/serial/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16
mod control;
mod defs;
mod baudr;
mod tdr;
mod isr;

use super::{Control, Register};
use volatile::Volatile;
use self::control::UsartCR;
use self::baudr::UsartBRR;
use self::tdr::TDR;
use self::isr::ISR;
use self::defs::*;
use peripheral::{rcc, gpio};

pub use self::control::{WordLength, Mode, Parity, StopLength, HardwareFlowControl};
pub use self::baudr::BaudRate;

#[derive(Copy, Clone)]
pub enum UsartX {
    Usart1,
    Usart2,
}

#[derive(Copy, Clone)]
pub struct Usart {
    mem_addr: *const u32,
    control: UsartCR,
    baud: UsartBRR,
    tdr: TDR,
    isr: ISR,
}

impl Control for Usart {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr as *const u32)
    }
}

impl Usart {
    pub fn new(x: UsartX) -> Self {
        match x {
            UsartX::Usart1 => Usart {
                mem_addr: USART1_ADDR,
                control: UsartCR::new(USART1_ADDR),
                baud: UsartBRR::new(USART1_ADDR),
                tdr: TDR::new(USART1_ADDR),
                isr: ISR::new(USART1_ADDR),
            },
            UsartX::Usart2 => Usart {
                mem_addr: USART2_ADDR,
                control: UsartCR::new(USART2_ADDR),
                baud: UsartBRR::new(USART2_ADDR),
                tdr: TDR::new(USART1_ADDR),
                isr: ISR::new(USART1_ADDR),
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

    pub fn set_stop_bits(&self, length: StopLength) {
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

    pub fn transmit_byte(&self, byte: u8) {
        self.tdr.store(byte);
    }

    pub fn get_txe(&self) -> bool {
        self.isr.get_txe()
    }
}

pub fn init() {
    let rcc = rcc::rcc();
    rcc.enable_peripheral(rcc::Peripheral::USART1);

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

    let usart1 = Usart::new(UsartX::Usart1);
    usart1.disable_usart();
    usart1.set_word_length(WordLength::Eight);
    usart1.set_mode(Mode::Transmit);
    usart1.set_parity(Parity::None);
    usart1.set_hardware_flow_control(HardwareFlowControl::None);


    let cr = rcc.get_system_clock_rate();
    usart1.set_baud_rate(BaudRate::Rate9600, cr);

    usart1.enable_usart();

    loop {
        let byte: u8 = b'a';
        while !usart1.get_txe() {}
        usart1.transmit_byte(byte);
    }
}

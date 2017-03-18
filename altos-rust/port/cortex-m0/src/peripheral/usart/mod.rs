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

//! This module is the highest level in the Usart heirarchy for implementing
//! the serial driver.
//!
//! Configuration for each of the two Usart registers, and each of the registers
//! that are offset from Usartx, and the public functions used to initialize,
//! configure, and manipulate the bits for each register are defined in this file.
//!
//! The functions here are used as wrappers that pass the call down through
//! each necessary level (one or more), until the actual register is reached
//! and is able to set the bits for itself accordingly.
//!
//! This module is also responsible for initial setup of the Usart register
//! (Either Usart1 or Usart2).

mod control;
mod defs;
mod baudr;
mod tdr;
mod rdr;
mod isr;
mod icr;

use super::{Control, Register};
use volatile::Volatile;
use self::control::UsartControl;
use self::baudr::BRR;
use self::tdr::TDR;
use self::rdr::RDR;
use self::isr::ISR;
use self::icr::ICR;
use self::defs::*;
use peripheral::{rcc, gpio};
use interrupt;

pub use self::control::{WordLength, Mode, Parity, StopLength, HardwareFlowControl};
pub use self::baudr::BaudRate;

/// Defines the wake/sleep channel for the TX buffer when full.
pub const USART2_TX_CHAN: usize = 43;
/// Defines the wake/sleep channel for when bytes are available in the receive buffer.
pub const USART2_RX_CHAN: usize = 43 * 3;

/// STM32F0 has two Usart registers available.
#[derive(Copy, Clone, Debug)]
pub enum UsartX {
    /// Connected to PA9 (TX) and PA10 (RX).
    Usart1,
    /// Usart2 is the debug serial.
    /// Connected to PA2 (TX) and pa15 (RX).
    Usart2,
}

/// Usart is the serial peripheral. This type is used to configure
/// the serial peripheral to send and receive data through the serial bus.
#[derive(Copy, Clone, Debug)]
pub struct Usart {
    // Memory address of the Usart
    mem_addr: *const u32,
    // Collection of control registers
    control: UsartControl,
    // Baud rate register
    baud: BRR,
    // Transmit data register
    tdr: TDR,
    // Read data register
    rdr: RDR,
    // Interrupt service register
    isr: ISR,
    // Interrupt clear register
    icr: ICR,
}

impl Control for Usart {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr as *const u32)
    }
}

impl Usart {
    /// Creates a new Usart object to configure the specifications for
    /// the serial peripheral.
    pub fn new(x: UsartX) -> Self {
        match x {
            UsartX::Usart1 => Usart {
                mem_addr: USART1_ADDR,
                control: UsartControl::new(USART1_ADDR),
                baud: BRR::new(USART1_ADDR),
                tdr: TDR::new(USART1_ADDR),
                rdr: RDR::new(USART1_ADDR),
                isr: ISR::new(USART1_ADDR),
                icr: ICR::new(USART1_ADDR),
            },
            UsartX::Usart2 => Usart {
                mem_addr: USART2_ADDR,
                control: UsartControl::new(USART2_ADDR),
                baud: BRR::new(USART2_ADDR),
                tdr: TDR::new(USART2_ADDR),
                rdr: RDR::new(USART2_ADDR),
                isr: ISR::new(USART2_ADDR),
                icr: ICR::new(USART2_ADDR),
            },
        }
    }

    /// Enable the Usart.
    pub fn enable_usart(&mut self) {
        self.control.enable_usart();
    }

    /// Disable the Usart.
    pub fn disable_usart(&mut self) {
        self.control.disable_usart();
    }

    /// Check if Usart is enabled. Returns true if enabled, false otherwise.
    pub fn is_usart_enabled(&mut self) -> bool {
        self.control.is_usart_enabled()
    }

    /// Set the Usart mode for transmit and receive configurations.
    pub fn set_mode(&mut self, mode: Mode) {
        self.control.set_mode(mode);
    }

    /// Enable the RXNE interrupt. This interrupt occurs when the
    /// receive data register has data in it.
    pub fn enable_receiver_not_empty_interrupt(&mut self) {
        self.control.enable_receiver_not_empty_interrupt();
    }

    /// Disable the RXNE interrupt. This interrupt occurs when the
    /// receive data register has data in it.
    pub fn disable_receiver_not_empty_interrupt(&mut self) {
        self.control.disable_receiver_not_empty_interrupt();
    }

    /// Enable the TC interrupt. This interrupt occurs when complete
    /// transmission of the data is finished.
    pub fn enable_transmit_complete_interrupt(&mut self) {
        self.control.enable_transmit_complete_interrupt();
    }

    /// Disable the TC interrupt. This interrupt occurs when complete
    /// transmission of the data is finished.
    pub fn disable_transmit_complete_interrupt(&mut self) {
        self.control.disable_transmit_complete_interrupt();
    }

    /// Enable the TXE interrupt. This interrupt occurs when the transmit
    /// data register is ready for more data.
    pub fn enable_transmit_interrupt(&mut self) {
        self.control.enable_transmit_interrupt();
    }

    /// Disable the TXE interrupt. This interrupt occurs when the transmit
    /// data register is ready for more data.
    pub fn disable_transmit_interrupt(&mut self) {
        self.control.disable_transmit_interrupt();
    }

    /// Enables parity checking. Used to determine if data corruption
    /// has occurred.
    pub fn set_parity(&mut self, parity: Parity) {
        self.control.set_parity(parity);
    }

    /// Sets the length of each data packet.
    pub fn set_word_length(&mut self, length: WordLength) {
        self.control.set_word_length(length);
    }

    /// Enable oversampling by 8.
    pub fn enable_over8(&mut self) {
        self.control.enable_over8();
    }

    /// Default to oversampling by 16.
    pub fn disable_over8(&mut self) {
        self.control.disable_over8();
    }

    /// Set the number of stop bits.
    pub fn set_stop_bits(&mut self, length: StopLength) {
        self.control.set_stop_bits(length);
    }

    /// Set hardware flow control mode.
    ///
    /// # Note
    /// Implementation for this functionality is not complete.
    pub fn set_hardware_flow_control(&mut self, hfc: HardwareFlowControl) {
        self.control.set_hardware_flow_control(hfc);
    }

    // --------------------------------------------------------------

    /// Set baud rate based on clock rate function argument.
    pub fn set_baud_rate(&mut self, baud_rate: BaudRate, clock_rate: u32) {
        self.baud.set_baud_rate(baud_rate, clock_rate, self.control.get_over8());
    }

    // --------------------------------------------------------------

    /// Move byte to TDR in order to transmit it.
    pub fn transmit_byte(&mut self, byte: u8) {
        self.tdr.store(byte);
    }

    // --------------------------------------------------------------

    /// Load byte from RDR.
    pub fn load_byte(&self) -> u8 {
        self.rdr.load()
    }

    // --------------------------------------------------------------

    /// Check if RXNE flag is set. RNXE flag is set when the RDR has
    /// data available. Returns true if RXNE flag is set, false otherwise.
    pub fn is_rx_reg_full(&self) -> bool {
        self.isr.get_rxne()
    }

    /// Check if TC flag is set. TC flag is set when transmission of a
    /// series of packets is complete. Returns true if TC flag is set,
    /// false otherwise.
    pub fn is_transmission_complete(&self) -> bool {
        self.isr.get_tc()
    }

    /// Check if TXE flag is set. TXE flag is set when the TDR is empty.
    /// Returns true if TXE flag is set, false otherwise.
    pub fn is_tx_reg_empty(&self) -> bool {
        self.isr.get_txe()
    }

    // --------------------------------------------------------------

    /// Clear the ORE flag. ORE flag is set when data is received when
    /// the RDR is full.
    pub fn clear_ore_flag(&mut self) {
        self.icr.clear_ore();
    }

    /// Clear the TC flag. TC flag is set when transmission of a
    /// series of packets is complete.
    pub fn clear_tc_flag(&mut self) {
        self.icr.clear_tc();
    }

    /// Clear the IDLE flag. IDLE flag is set when an idle line is detected. :P
    pub fn clear_idle_flag(&mut self) {
        self.icr.clear_idle();
    }
}

/// Initialize the Usart2 peripheral.
///
/// Connects the necessary GPIO pins, sets the clock, enables interrupts,
/// and currently configures the Usart2 to 9600 8N1 configuration.
pub fn init() {
    let mut rcc = rcc::rcc();
    rcc.enable_peripheral(rcc::Peripheral::USART2);

    gpio::GPIO::enable(gpio::Group::A);
    let mut pa2 = gpio::Port::new(2, gpio::Group::A);
    let mut pa15 = gpio::Port::new(15, gpio::Group::A);
    pa2.set_function(gpio::AlternateFunction::One);
    pa15.set_function(gpio::AlternateFunction::One);
    pa2.set_speed(gpio::Speed::High);
    pa15.set_speed(gpio::Speed::High);
    pa2.set_mode(gpio::Mode::Alternate);
    pa15.set_mode(gpio::Mode::Alternate);
    pa2.set_type(gpio::Type::PushPull);
    pa15.set_type(gpio::Type::PushPull);
    pa2.set_pull(gpio::Pull::Up);
    pa15.set_pull(gpio::Pull::Up);

    let mut usart2 = Usart::new(UsartX::Usart2);
    usart2.disable_usart();

    usart2.set_word_length(WordLength::Eight);
    usart2.set_mode(Mode::All);
    usart2.set_parity(Parity::None);
    usart2.set_hardware_flow_control(HardwareFlowControl::None);

    let clock_rate = rcc.get_system_clock_rate();
    usart2.set_baud_rate(BaudRate::Hz9600, clock_rate);

    usart2.enable_receiver_not_empty_interrupt();
    usart2.enable_transmit_interrupt();
    usart2.enable_usart();

    let mut nvic = interrupt::nvic();
    nvic.enable_interrupt(interrupt::Hardware::USART2);
}

/*
 * Copyright © 2017 AltOS-Rust Team
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
 */

mod control;
mod defs;
mod baudr;
mod tdr;
mod rdr;
mod isr;
mod icr;

/// This module is the highest level in the Usart heirarchy for implementing
/// the serial driver.
///
/// Configuration for each of the two Usart registers, and each of the registers
/// that are offset from Usartx, and the public functions used to initialize,
/// configure, and manipulate the bits for each register is defined in this file.
///
/// The functions here are used as wrappers that pass the call down through
/// each necessary level (one or more), until the actual register is reached
/// and is able to set the bits for itself accordingly.
///
/// This module is also responsible for initial setup of the Usart register
/// (Either Usart1 or Usart2)

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

pub const USART2_TX_BUFFER_FULL_CHAN: usize = 43;
pub const USART2_TC_CHAN: usize = 43 * 2;

/// STM32F0 has two Usart registers available.
#[derive(Copy, Clone, Debug)]
pub enum UsartX {
    Usart1,
    Usart2,
}

/// Defines the memory address for the Usart and each variable needed
/// to access the registers that comprise the Usart.
/* mem_addr: Memory address of the Usart
 * control: Collection of control registers
 * baud: Baud rate register
 * tdr: Transmit data register
 * rdr: Read data register
 * isr: Interrupt service register
 * icr: Interrupt clear register
 */
#[derive(Copy, Clone, Debug)]
pub struct Usart {
    mem_addr: *const u32,
    control: UsartControl,
    baud: BRR,
    tdr: TDR,
    rdr: RDR,
    isr: ISR,
    icr: ICR,
}

/// Wraps the Usart memory address in order to make it volatile.
impl Control for Usart {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr as *const u32)
    }
}

/// Function implementations for the Usart struct. These are the wrapper
/// functions that can be called on a Usart object in order to set
/// specific bits in each of the registers contained within the Usart struct.
/// Each function calls the appropriate function associated with the
/// register needed to set the correct bits.
impl Usart {

    /// Creates a new Usart. Pattern matches the argument passed in, and
    /// creates and initializes either a new Usart1 or a new Usart2.
    /// The base address of the Usart being used is passed into each 'new` call
    /// the proper address with offset can be calculate for the associated
    /// register.
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

    // --------------------------------------------------------------
    /// This section pertains to calls made to the control registers.
    /// See control.rs for more specific information.

    /// Makes call using the Usart control variable to enable the Usart.
    pub fn enable_usart(&mut self) {
        self.control.enable_usart();
    }

    /// Makes call using the Usart control variable to disable the Usart.
    pub fn disable_usart(&mut self) {
        self.control.disable_usart();
    }

    /// Makes call using the Usart control variable to check if the Usart
    /// Usart is enabled or not. Returns true if enabled, false otherwise.
    pub fn is_usart_enabled(&mut self) -> bool {
        self.control.is_usart_enabled()
    }

    /// Makes call using the Usart control variable to set the mode for the Usart.
    /// Possible Mode settings are: None, Transmit, Recieve, or All.
    pub fn set_mode(&mut self, mode: Mode) {
        self.control.set_mode(mode);
    }

    /// Makes call using the Usart control variable to enable the RXNE interrupt.
    pub fn enable_receiver_not_empty_interrupt(&mut self) {
        self.control.enable_receiver_not_empty_interrupt();
    }

    /// Makes call using the Usart control variable to disable the RXNE interrupt.
    pub fn disable_receiver_not_empty_interrupt(&mut self) {
        self.control.disable_receiver_not_empty_interrupt();
    }

    /// Makes call using the Usart control variable to enable the TCIE interrupt.
    pub fn enable_transmit_complete_interrupt(&mut self) {
        self.control.enable_transmit_complete_interrupt();
    }

    /// Makes call using the Usart control variable to disable the TCIE interrupt.
    pub fn disable_transmit_complete_interrupt(&mut self) {
        self.control.disable_transmit_complete_interrupt();
    }

    /// Makes call using the Usart control variable to enable the TXEIE interrupt.
    pub fn enable_transmit_interrupt(&mut self) {
        self.control.enable_transmit_interrupt();
    }

    /// Makes call using the Usart control variable to disable the TXEIE interrupt.
    pub fn disable_transmit_interrupt(&mut self) {
        self.control.disable_transmit_interrupt();
    }

    /// Makes call using the Usart control variable to set the parity.
    /// Possible Parity settings are: None, Even, Odd
    pub fn set_parity(&mut self, parity: Parity) {
        self.control.set_parity(parity);
    }

    /// Makes call using the Usart control variable to set the word length.
    /// Possible WordLength settings are: Seven, Eight, Nine
    pub fn set_word_length(&mut self, length: WordLength) {
        self.control.set_word_length(length);
    }

    /// Makes call using the Usart control variable to enable oversampling by 8.
    pub fn enable_over8(&mut self) {
        self.control.enable_over8();
    }

    /// Makes call using the Usart control variable to disable oversampling by 8.
    /// Will default to oversampling by 16.
    pub fn disable_over8(&mut self) {
        self.control.disable_over8();
    }

    /// Makes call using the Usart control variable to set the number
    /// of stop bits.
    /// Possible StopLength settings are: Half, One, OneAndHalf, Two
    pub fn set_stop_bits(&mut self, length: StopLength) {
        self.control.set_stop_bits(length);
    }

    /// Makes call using the Usart control variable to set hardware flow control.
    /// Possible HardwareFlowControl settings are: None, Rts, Cts, RtsCts
    pub fn set_hardware_flow_control(&mut self, hfc: HardwareFlowControl) {
        self.control.set_hardware_flow_control(hfc);
    }

    // --------------------------------------------------------------
    /// This section pertains to calls made to the BRR.
    /// See baud.rs for more specific information.

    /// Makes call using the Usart baud variable to set the baud rate.
    /// Possible BaudRate settings are: Hz4800, Hz9600, Hz19200, Hz57600, Hz115200
    pub fn set_baud_rate(&mut self, baud_rate: BaudRate, clock_rate: u32) {
        self.baud.set_baud_rate(baud_rate, clock_rate, self.control.get_over8());
    }

    // --------------------------------------------------------------
    /// This section pertains to calls made to the TDR.
    /// See tdr.rs for more specific information.

    /// Transmits a byte by making call to the Usart tdr variable so that the
    /// byte can be stored in the TDR.
    pub fn transmit_byte(&mut self, byte: u8) {
        self.tdr.store(byte);
    }

    // --------------------------------------------------------------
    /// This section pertains to calls made to the RDR.
    /// See rdr.rs for more specific information.

    /// Receives a byte by making a call to the Usart rdr variable so that the
    /// byte can be loaded into the RDR.
    pub fn load_byte(&self) -> u8 {
        self.rdr.load()
    }

    // --------------------------------------------------------------
    /// This section pertains to calls made to the ISR.
    /// See isr.rs for more specific information.

    /// Checks to see if the the read register has data.
    /// Returns true if has data, false otherwise.
    pub fn is_rx_reg_full(&self) -> bool {
        self.isr.get_rxne()
    }

    /// Check to see if the transmission has completed.
    /// Returns true if complete, false otherwise.
    pub fn is_transmission_complete(&self) -> bool {
        self.isr.get_tc()
    }

    /// Checks to see if the transmit data register is empty.
    /// Returns true if empty, false otherwise.
    pub fn is_tx_reg_empty(&self) -> bool {
        self.isr.get_txe()
    }

    // --------------------------------------------------------------
    /// This section pertains to calls made to the ICR.
    /// See icr.rs for more specific information.

    /// Clears the overrun error flag in the ISR.
    pub fn clear_ore_flag(&self) {
        self.icr.clear_ore();
    }

    /// Clears the transmission complete flag in the ISR.
    pub fn clear_tc_flag(&self) {
        self.icr.clear_tc();
    }

    /// Clears the line idle flag in the ISR.
    pub fn clear_idle_flag(&self) {
        self.icr.clear_idle();
    }
}

/// Initializes the Usart register (either Usart1 or Usart2).
/// Connects the necessary GPIO pins, sets the clock, enables interrupts,
/// and currently initializes the Usart2 to tranmsit and receive.
pub fn init() {
    // Creates clock object.
    let rcc = rcc::rcc();
    // Enables Usart2 as the target for all serial communication.
    rcc.enable_peripheral(rcc::Peripheral::USART2);

    // Enables GPIO pins and initializes settings.
    gpio::GPIO::enable(gpio::Group::A);
    let mut pa2 = gpio::Port::new(2, gpio::Group::A);
    let mut pa3 = gpio::Port::new(3, gpio::Group::A);
    pa2.set_function(gpio::AlternateFunction::One);
    pa3.set_function(gpio::AlternateFunction::One);
    pa2.set_speed(gpio::Speed::High);
    pa3.set_speed(gpio::Speed::High);
    pa2.set_mode(gpio::Mode::Alternate);
    pa3.set_mode(gpio::Mode::Alternate);
    pa2.set_type(gpio::Type::PushPull);
    pa3.set_type(gpio::Type::PushPull);
    pa2.set_pull(gpio::Pull::Up);
    pa3.set_pull(gpio::Pull::Up);

    // Creates a Usart2 object and initializes it for serial communication.
    let mut usart2 = Usart::new(UsartX::Usart2);
    // Disable Usart prior to initializing.
    usart2.disable_usart();

    // Set word length to Eight
    usart2.set_word_length(WordLength::Eight);
    // Set mode to Transmit and Receive
    usart2.set_mode(Mode::All);
    // Set parity to none
    usart2.set_parity(Parity::None);
    // Set hardware flow control to none
    usart2.set_hardware_flow_control(HardwareFlowControl::None);

    // Get the current clock rate
    let clock_rate = rcc.get_system_clock_rate();
    // Set the baud rate to 9600 Hz
    usart2.set_baud_rate(BaudRate::Hz9600, clock_rate);

    // Enable interrupts.
    usart2.enable_receiver_not_empty_interrupt();
    usart2.enable_transmit_interrupt();
    // Enable usart
    usart2.enable_usart();

    // Enables the interrupt in the nested vector interrupt control.
    let nvic = interrupt::nvic();
    // TODO: The number 28 here should be replaced by an enum in the
    // `interrupt` module
    nvic.enable_interrupt(28);
}

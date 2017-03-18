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

/* This submodule contains the function implementations for the Usartx_CRx.
 * There are three control registers and that are responsible for proper
 * configuration of the Usart.
 */

use super::super::Register;
use super::defs::*;

// There are three control registers for each of the two Usarts.
#[derive(Copy, Clone, Debug)]
pub struct UsartControl {
    // Control Register 1
    cr1: CR1,
    // Control Register 2
    cr2: CR2,
    // Control Register 3
    cr3: CR3,
}

/* Function implementations for the Usart. These functions
 * are responsible for passing the call down to the the associated
 * function call in the correct control register. The implementations
 * for each of these functions can be found in the associated control
 * register.
 * These functions are called from the functions defined for the
 * Usart struct.
 */
impl UsartControl {
    pub fn new(base_addr: *const u32) -> Self {
        UsartControl {
            cr1: CR1::new(base_addr),
            cr2: CR2::new(base_addr),
            cr3: CR3::new(base_addr),
        }
    }

    // --------------------------------------------------------------

    pub fn enable_usart(&mut self) {
        self.cr1.enable_usart(true);
    }

    pub fn disable_usart(&mut self) {
        self.cr1.enable_usart(false);
    }

    pub fn is_usart_enabled(&self) -> bool {
        self.cr1.is_usart_enabled()
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.cr1.set_mode(mode);
    }

    pub fn enable_receiver_not_empty_interrupt(&mut self) {
        self.cr1.set_receiver_not_empty_interrupt(true);
    }

    pub fn disable_receiver_not_empty_interrupt(&mut self) {
        self.cr1.set_receiver_not_empty_interrupt(false);
    }

    pub fn enable_transmit_complete_interrupt(&mut self) {
        self.cr1.set_transmit_complete_interrupt(true);
    }

    pub fn disable_transmit_complete_interrupt(&mut self) {
        self.cr1.set_transmit_complete_interrupt(false);
    }

    pub fn enable_transmit_interrupt(&mut self) {
        self.cr1.set_transmit_interrupt(true);
    }

    pub fn disable_transmit_interrupt(&mut self) {
        self.cr1.set_transmit_interrupt(false);
    }

    pub fn set_parity(&mut self, parity: Parity) {
        self.cr1.set_parity(parity);
    }

    pub fn set_word_length(&mut self, length: WordLength) {
        self.cr1.set_word_length(length);
    }

    pub fn enable_over8(&mut self) {
        self.cr1.set_over8(true);
    }

    pub fn disable_over8(&mut self) {
        self.cr1.set_over8(false);
    }

    pub fn get_over8(&self) -> bool {
        self.cr1.get_over8()
    }

    // --------------------------------------------------------------

    pub fn set_stop_bits(&mut self, length: StopLength) {
        self.cr2.set_stop_bits(length);
    }

    // --------------------------------------------------------------

    pub fn set_hardware_flow_control(&mut self, hfc: HardwareFlowControl) {
        self.cr3.set_hardware_flow_control(hfc);
    }
}

// ------------------------------------
// CR1 - Control Register One
// ------------------------------------

/// Defines the possible Mode configurations for the Usart.
#[derive(Copy, Clone, Debug)]
pub enum Mode {
    /// Transmit and Receive turned off.
    None,
    /// Usart configured to only receive.
    Receive,
    /// Usart configuration to only transmit.
    Transmit,
    /// Transmit and Receive both turned on.
    All,
}

/// Defines the possible Parity configurations for the Usart.
#[derive(Copy, Clone, Debug)]
pub enum Parity {
    /// No parity configuration set.
    None,
    /// Even parity configuration.
    Even,
    /// Odd parity configuration.
    Odd,
}

/// Defines the possible WordLength configurations for the Usart.
#[derive(Copy, Clone, Debug)]
pub enum WordLength {
    /// Seven bit word length
    Seven,
    /// Eight bit word length
    Eight,
    /// Nine bit word length
    Nine,
}

#[derive(Copy, Clone, Debug)]
struct CR1 {
    base_addr: *const u32,
}

impl Register for CR1 {
    fn new(base_addr: *const u32) -> Self {
        CR1 { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CR1_OFFSET
    }
}

impl CR1 {
    /* Uses bit 0 in CR1 to enables or disable the USARTx based on bool
     * variable passed in.
     *  Bit 0 UE: USART enable
     *      When this bit is cleared, the USART prescalers and outputs are
     *      stopped immediately, and current operations are discarded. The
     *      configuration of the USART is kept, but all the status flags, in
     *      the USARTx_ISR are set to their default values. This bit is set and
     *      cleared by software.
     *          0: USART prescaler and outputs disabled, low-power mode
     *          1: USART enabled
     */
    fn enable_usart(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_UE);
            if enable {
                *reg |= CR1_UE;
            }
        }
    }

    /* Checks if usart is enabled.
     * Returns true if enabled (CR1 bit 0 (UE) = 1), false otherwise
     */
    fn is_usart_enabled(&self) -> bool {
        unsafe {
            *self.addr() & CR1_UE != 0
        }
    }

    /* Uses bits 2 and 3 in CR1 to set the mode to None, Receive, Transmit or All
     *  Bit 2 RE: Receiver enable
     *      This bit enables the receiver. It is set and cleared by software.
     *          0: Receiver is disabled
     *          1: Receiver is enabled and begins searching for a start bit
     *  Bit 3 TE: Transmitter enable
     *      This bit enables the transmitter. It is set and cleared by software.
     *          0: Transmitter is disabled
     *          1: Transmitter is enabled
     */
    fn set_mode(&mut self, mode: Mode) {
        let mask = match mode {
            Mode::None => 0,
            Mode::Receive => CR1_RE,
            Mode::Transmit => CR1_TE,
            Mode::All => (CR1_RE | CR1_TE),
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_RE | CR1_TE);
            *reg |= mask;
        }
    }

    /* Uses bit 5 in CR1 to enable or disable RXNE interrupt based on bool
     * variable passed in.
     *      true: Enables interrupt
     *      false: Disables interrupt
     *
     *  Bit 5 RXNEIE: RXNE interrupt enable
     *      This bit is set and cleared by software.
     *          0: Interrupt is inhibited
     *          1: A USART interrupt is generated whenever ORE=1 or RXNE=1
     *          in the USARTx_ISR register
     *
     */
    fn set_receiver_not_empty_interrupt(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_RXNEIE);
            if enable {
                *reg |= CR1_RXNEIE;
            }
        }
    }

    /* Uses bit 6 in CR1 to enable or disable the TCIE interrupt based on the
     * bool variable passed in.
     *      true: Enables interrupt
     *      false: Disables interrupt
     *
     *  Bit 6 TCIE: Transmission complete interrupt enable
     *      This bit is set and cleared by software.
     *          0: Interrupt is inhibited
     *          1: A USART interrupt is generated whenever TC=1 in the
     *          USARTx_ISR register
     */
    fn set_transmit_complete_interrupt(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_TCIE);
            if enable {
                *reg |= CR1_TCIE;
            }
        }
    }

    /* Uses bit 7 in CR1 to enable or disable the TXEIE interrupt based on
     * bool variable passed in.
     *      true: Enables interrupt
     *      false: Disables interrupt
     *  Bit 7 TXEIE: interrupt enable
     *      This bit is set and cleared by software.
     *          0: Interrupt is inhibited
     *          1: A USART interrupt is generated whenever TXE=1 in the
     *          USARTx_ISR register
     */
    fn set_transmit_interrupt(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_TXEIE);
            if enable {
                *reg |= CR1_TXEIE;
            }
        }
    }

    /* Uses bits 9 and 10 in CR1 to set the parity to None, Even, Odd
     *  Bit 9 PS: Parity selection
     *      This bit selects the odd or even parity when the parity
     *      generation/detection is enabled (PCE bit set). It is set and
     *      cleared by software. The parity will be selected after the current
     *      byte.
     *          0: Even parity
     *          1: Odd parity
     *  Bit 10 PCE: Parity control enable
     *      This bit selects the hardware parity control (generation and
     *      detection). When the parity control is enabled, the computed parity
     *      is inserted at the MSB position (9th bit if M=1; 8th bit if M=0)
     *      and parity is checked on the received data. This bit is set and
     *      cleared by software.
     *          0: Parity control disabled
     *          1: Parity control enabled.
     */
    fn set_parity(&mut self, parity: Parity) {
        let mask = match parity {
            Parity::None => 0,
            Parity::Even => CR1_PCE,
            Parity::Odd => CR1_PS | CR1_PCE,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_PS | CR1_PCE);
            *reg |= mask;
        }
    }

    /* Uses bits 12 and 28 to set the word length to Seven, Eight, or Nine
     *  Bit [28:12] M1:M0: Word length
     *      Bit 28 (M1), with bit 12 (M0), determines the word length.
     *      It is set or cleared by software.
     *          M[1:0] = 10: 1 Start bit, 7 data bits, n stop bits
     *          M[1:0] = 00: 1 Start bit, 8 data bits, n stop bits
     *          M[1:0] = 01: 1 Start bit, 9 data bits, n stop bits
     */
    fn set_word_length(&mut self, length: WordLength) {
        let mask = match length {
            WordLength::Seven => CR1_M1,
            WordLength::Eight => 0,
            WordLength::Nine => CR1_M0,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_M0 | CR1_M1);
            *reg |= mask;
        }
    }

    /* Uses bit 15 to enable or disable oversampling by 8 based on the bool
     * variable passed in.
     *      Bit 15 OVER8: Oversampling mode
     *          0: Oversampling by 16
     *          1: Oversampling by 8
     */
    fn set_over8(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_OVER8);
            if enable {
                *reg |= CR1_OVER8;
            }
        }
    }

    /* Checks if over8 is enabled.
     *  Returns true if enabled (CR1 bit 15 (Over8) = 1), false otherwise.
     */
    fn get_over8(&self) -> bool {
        unsafe {
            *self.addr() & CR1_OVER8 != 0
        }
    }
}

// ------------------------------------
// CR2
// ------------------------------------

/// Defines the possible StopLength configurations for the Usart.
#[derive(Copy, Clone, Debug)]
pub enum StopLength {
    /// 0.5 stop bit
    Half,
    /// 1 stop bit
    One,
    /// 1.5 stop bits
    OneAndHalf,
    /// 2 stop bits
    Two,
}

#[derive(Copy, Clone, Debug)]
struct CR2 {
    base_addr: *const u32,
}

impl Register for CR2 {
    fn new(base_addr: *const u32) -> Self {
        CR2 { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CR2_OFFSET
    }
}


impl CR2 {
    /* Uses bits 12 and 13 to set the stop length to Half, One, OneAndHalf, Two
     *      Bits 13:12 STOP[1:0]: STOP bits
     *          These bits are used for programming the stop bits.
     *              00: 1 stop bit
     *              01: 0.5 stop bit
     *              10: 2 stop bits
     *              11: 1.5 stop bits
     */
    fn set_stop_bits(&mut self, length: StopLength) {
        let mask = match length {
            StopLength::Half => CR2_STOP_BIT0,
            StopLength::One => 0,
            StopLength::OneAndHalf => CR2_STOP_BIT0 | CR2_STOP_BIT1,
            StopLength::Two => CR2_STOP_BIT1,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR2_STOP_BIT0 | CR2_STOP_BIT1);
            *reg |= mask;
        }
    }
}

// ------------------------------------
// CR3
// ------------------------------------

/// Defines the possible HardwareFlowControl configurations for the Usart.
#[derive(Copy, Clone, Debug)]
pub enum HardwareFlowControl {
    /// No hardware flow control.
    None,
    /// Request to Send enabled.
    Rts,
    /// Clear to Send enabled.
    Cts,
    /// Both are enabled.
    All,
}

#[derive(Copy, Clone, Debug)]
struct CR3 {
    base_addr: *const u32,
}

impl Register for CR3 {
    fn new(base_addr: *const u32) -> Self {
        CR3 { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CR3_OFFSET
    }
}

impl CR3 {
    /* Uses bit 8 and 9 in CR3 to set the hardware flow control to None, Rts,
     * Cts, All.
     *      Bit 8 RTSE: RTS enable
     *          0: RTS hardware flow control disabled
     *          1: RTS output enabled, data is only requested when there is
     *          space in the receive buffer. The transmission of data is
     *          expected to cease after the current character has been
     *          transmitted. The nRTS output is asserted (pulled to 0) when
     *          data can be received.
     *      Bit 9 CTSE: CTS enable
     *          0: CTS hardware flow control disabled
     *          1: CTS mode enabled, data is only transmitted when the nCTS
     *           input is asserted (tied to 0). If the nCTS input is
     *           de-asserted while data is being transmitted, then the
     *           transmission is completed before stopping. If data is written
     *           into the data register while nCTS is de-asserted, the
     *           transmission is postponed until nCTS is asserted.
     */
    fn set_hardware_flow_control(&mut self, hfc: HardwareFlowControl) {
        let mask = match hfc {
            HardwareFlowControl::None => 0,
            HardwareFlowControl::Rts => CR3_RTSE,
            HardwareFlowControl::Cts => CR3_CTSE,
            HardwareFlowControl::All => CR3_RTSE | CR3_CTSE,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR3_RTSE | CR3_CTSE);
            *reg |= mask;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_cr1_enable_disable_usart() {
        let mut cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.register_value(), 0b0);

        cr1.enable_usart(true);
        assert_eq!(cr1.register_value(), 0b1);

        cr1.enable_usart(false);
        assert_eq!(cr1.register_value(), 0b0);
    }

    #[test]
    fn test_cr1_is_usart_enabled_returns_false_when_disabled() {
        let cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.is_usart_enabled(), false);
    }

    #[test]
    fn test_cr1_is_usart_enabled_returns_true_when_enabled() {
        let cr1 = test::create_initialized_register::<CR1>(1);
        assert_eq!(cr1.is_usart_enabled(), true);
    }

    #[test]
    fn test_cr1_set_word_length() {
        let mut cr1 = test::create_register::<CR1>();

        cr1.set_word_length(WordLength::Seven);
        assert_eq!(cr1.register_value(), 0b1 << 28);

        cr1.set_word_length(WordLength::Eight);
        assert_eq!(cr1.register_value(), 0b0);

        cr1.set_word_length(WordLength::Nine);
        assert_eq!(cr1.register_value(), 0b1 << 12);
    }

    #[test]
    fn test_cr1_set_mode() {
        let mut cr1 = test::create_register::<CR1>();

        cr1.set_mode(Mode::Receive);
        assert_eq!(cr1.register_value(), 0b1 << 2);

        cr1.set_mode(Mode::Transmit);
        assert_eq!(cr1.register_value(), 0b1 << 3);

        cr1.set_mode(Mode::All);
        assert_eq!(cr1.register_value(), 0b11 << 2);
    }

    #[test]
    fn test_cr1_set_parity() {
        let mut cr1 = test::create_register::<CR1>();

        cr1.set_parity(Parity::None);
        assert_eq!(cr1.register_value(), 0b0);

        cr1.set_parity(Parity::Even);
        assert_eq!(cr1.register_value(), 0b1 << 10);

        cr1.set_parity(Parity::Odd);
        assert_eq!(cr1.register_value(), 0b11 << 9);
    }

    #[test]
    fn test_cr1_set_over8() {
        let mut cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.register_value(), 0b0);

        cr1.set_over8(true);
        assert_eq!(cr1.register_value(), 0b1 << 15);

        cr1.set_over8(false);
        assert_eq!(cr1.register_value(), 0b0);
    }

    #[test]
    fn test_cr1_get_over8_returns_false_when_value_is_zero() {
        let cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.get_over8(), false);
    }

    #[test]
    fn test_cr1_get_over8_returns_true_when_value_is_set() {
        let cr1 = test::create_initialized_register::<CR1>(0b1 << 15);
        assert_eq!(cr1.get_over8(), true);
    }

    #[test]
    fn test_cr1_enable_transmit_interrupt() {
        let mut cr1 = test::create_register::<CR1>();
        cr1.set_transmit_interrupt(true);
        assert_eq!(cr1.register_value(), 0b1 << 7);
    }

    #[test]
    fn test_cr1_disable_transmit_interrupt() {
        let mut cr1 = test::create_initialized_register::<CR1>(0b1 << 7);
        cr1.set_transmit_interrupt(false);
        assert_eq!(cr1.register_value(), 0);
    }

    #[test]
    fn test_cr1_enable_transmit_complete_interrupt() {
        let mut cr1 = test::create_register::<CR1>();
        cr1.set_transmit_complete_interrupt(true);
        assert_eq!(cr1.register_value(), 0b1 << 6);
    }

    #[test]
    fn test_cr1_disable_transmit_complete_interrupt() {
        let mut cr1 = test::create_initialized_register::<CR1>(0b1 << 6);
        cr1.set_transmit_complete_interrupt(false);
        assert_eq!(cr1.register_value(), 0);
    }

    #[test]
    fn test_cr1_enable_receiver_not_empty_interrupt() {
        let mut cr1 = test::create_register::<CR1>();
        cr1.set_receiver_not_empty_interrupt(true);
        assert_eq!(cr1.register_value(), 0b1 << 5);
    }

    #[test]
    fn test_cr1_disable_receiver_not_empty_interrupt() {
        let mut cr1 = test::create_initialized_register::<CR1>(0b1 << 5);
        cr1.set_receiver_not_empty_interrupt(false);
        assert_eq!(cr1.register_value(), 0);
    }

    #[test]
    fn test_cr2_set_stop_bits() {
        let mut cr2 = test::create_register::<CR2>();
        assert_eq!(cr2.register_value(), 0b0);

        cr2.set_stop_bits(StopLength::Half);
        assert_eq!(cr2.register_value(), 0b1 << 12);

        cr2.set_stop_bits(StopLength::OneAndHalf);
        assert_eq!(cr2.register_value(), 0b11 << 12);

        cr2.set_stop_bits(StopLength::Two);
        assert_eq!(cr2.register_value(), 0b1 << 13);

        cr2.set_stop_bits(StopLength::One);
        assert_eq!(cr2.register_value(), 0b0);
    }

    #[test]
    fn test_cr3_set_hardware_flow_control() {
        let mut cr3 = test::create_register::<CR3>();
        assert_eq!(cr3.register_value(), 0b0);

        cr3.set_hardware_flow_control(HardwareFlowControl::Rts);
        assert_eq!(cr3.register_value(), 0b1 << 8);

        cr3.set_hardware_flow_control(HardwareFlowControl::Cts);
        assert_eq!(cr3.register_value(), 0b1 << 9);

        cr3.set_hardware_flow_control(HardwareFlowControl::All);
        assert_eq!(cr3.register_value(), 0b11 << 8);

        cr3.set_hardware_flow_control(HardwareFlowControl::None);
        assert_eq!(cr3.register_value(), 0b0);
    }
}

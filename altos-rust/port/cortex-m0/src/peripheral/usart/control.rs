// Daniel Seitz and RJ Russell

use super::super::Register;
use super::defs::*;

/// Three USART control registers.
#[derive(Copy, Clone)]
pub struct UsartCR {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
}

// TODO Need to implement a clear mask for each register to ensure that
// all register bits are set to zero before re-initializing register to
// necessary values for a specific usart configuration.
impl UsartCR {
    pub fn new(base_addr: *const u32) -> Self {
        UsartCR {
            cr1: CR1::new(base_addr),
            cr2: CR2::new(base_addr),
            cr3: CR3::new(base_addr),
        }
    }

    pub fn enable_usart(&self) {
        self.cr1.enable_usart(true);
    }

    pub fn disable_usart(&self) {
        self.cr1.enable_usart(false);
    }

    pub fn is_usart_enabled(&self) -> bool {
        self.cr1.is_usart_enabled()
    }

    pub fn set_word_length(&self, length: WordLength) {
        self.cr1.set_word_length(length);
    }

    pub fn set_mode(&self, mode: Mode) {
        self.cr1.set_mode(mode);
    }

    pub fn set_parity(&self, parity: Parity) {
        self.cr1.set_parity(parity);
    }

    pub fn set_stop_bits(&self, length: Stoplength) {
        self.cr2.set_stop_bits(length);
    }

    pub fn enable_over8(&self) {
        self.cr1.set_over8(true);
    }

    pub fn disable_over8(&self) {
        self.cr1.set_over8(false);
    }

    pub fn get_over8(&self) -> bool {
        self.cr1.get_over8()
    }

    pub fn set_hardware_flow_control(&self, hfc: HardwareFlowControl) {
        self.cr3.set_hardware_flow_control(hfc);
    }
}

// ------------------------------------
/// CR1
// ------------------------------------

/// Word length can be 7, 8, or 9 bits.
#[derive(Copy, Clone)]
pub enum WordLength {
    Seven,
    Eight,
    Nine,
}

pub enum Mode {
    Receive,
    Transmit,
    All,
}

pub enum Parity {
    None,
    Even,
    Odd,
}

#[derive(Copy, Clone)]
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
    // Enables and disables USARTx based on bool variable passed in.
    fn enable_usart(&self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            if enable {
                *reg |= CR1_UE;
            }
            else {
                *reg &= !(CR1_UE);
            }
        }
    }

    // Checks if usart is enabled.
    fn is_usart_enabled(&self) -> bool {
        unsafe {
            *self.addr() & CR1_UE != 0
        }
    }

    // Sets wordlength to 7, 8, or 9 bits.
    fn set_word_length(&self, length: WordLength) {
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

    // Sets mode for receive(Rx), transmit(Tx) or both(RxTx)
    fn set_mode(&self, mode: Mode) {
        let mask = match mode {
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

    // Sets parity to even or odd.
    fn set_parity(&self, parity: Parity) {
        let mask = match parity {
            Parity::None => 0,
            Parity::Even => CR1_PCE,
            Parity::Odd => CR1_PS | CR1_PCE,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_PCE | CR1_PS);
            *reg |= mask;
        }
    }

    // Sets oversampling by 16 (0) or by 8 (1)
    fn set_over8(&self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_OVER8);
            if enable {
                *reg |= CR1_OVER8;
            }
        }
    }

    fn get_over8(&self) -> bool {
        unsafe {
            *self.addr() & CR1_OVER8 != 0
        }
    }
}

// ------------------------------------
/// CR2
// ------------------------------------

/// There are four stop bit settings: .5, 1, 1.5, 2
pub enum Stoplength {
    Half,
    One,
    OneAndHalf,
    Two,
}

#[derive(Copy, Clone)]
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
    fn set_stop_bits(&self, length: Stoplength) {
        let mask = match length {
            Stoplength::Half => 0,
            Stoplength::One => CR2_STOP_BIT1,
            Stoplength::OneAndHalf => CR2_STOP_BIT0,
            Stoplength::Two => CR2_STOP_BIT0 | CR2_STOP_BIT1,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR2_STOP_BIT0 | CR2_STOP_BIT1);
            *reg |= mask;
        }
    }
}

// ------------------------------------
/// CR3
// ------------------------------------

#[derive(Copy, Clone)]
struct CR3 {
    base_addr: *const u32,
}

pub enum HardwareFlowControl {
    None,
    // Request to Send
    Rts,
    // Clear to Send
    Cts,
    // Both
    RtsCts,
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
    fn set_hardware_flow_control(&self, hfc: HardwareFlowControl) {
        let mask = match hfc {
            HardwareFlowControl::None => 0,
            HardwareFlowControl::Rts => CR3_RTSE,
            HardwareFlowControl::Cts => CR3_CTSE,
            HardwareFlowControl::RtsCts => CR3_RTSE | CR3_CTSE,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR3_RTSE | CR3_CTSE);
            *reg |= mask;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test;

    #[test]
    fn test_cr1_enable_disable_usart() {
        let cr1 = test::create_register::<CR1>();

        cr1.enable_usart(true);
        assert_eq!(cr1.register_value(), 0b1);

        cr1.enable_usart(false);
        assert_eq!(cr1.register_value(), 0b0);
    }

    #[test]
    fn test_cr1_is_usart_enabled() {
        let cr1 = test::create_register::<CR1>();

        assert_eq!(cr1.is_usart_enabled(), false);

        cr1.enable_usart(true);
        assert_eq!(cr1.is_usart_enabled(), true);

        cr1.enable_usart(false);
        assert_eq!(cr1.is_usart_enabled(), false);
    }

    #[test]
    fn test_cr1_set_word_length() {
        let cr1 = test::create_register::<CR1>();

        cr1.set_word_length(WordLength::Seven);
        assert_eq!(cr1.register_value(), 0b1 << 28);

        cr1.set_word_length(WordLength::Eight);
        assert_eq!(cr1.register_value(), 0b0);

        cr1.set_word_length(WordLength::Nine);
        assert_eq!(cr1.register_value(), 0b1 << 12);
    }

    #[test]
    fn test_cr1_set_mode() {
        let cr1 = test::create_register::<CR1>();

        cr1.set_mode(Mode::Receive);
        assert_eq!(cr1.register_value(), 0b1 << 2);

        cr1.set_mode(Mode::Transmit);
        assert_eq!(cr1.register_value(), 0b1 << 3);

        cr1.set_mode(Mode::All);
        assert_eq!(cr1.register_value(), 0b11 << 2);
    }

    #[test]
    fn test_cr1_set_parity() {
        let cr1 = test::create_register::<CR1>();

        cr1.set_parity(Parity::None);
        assert_eq!(cr1.register_value(), 0b0);

        cr1.set_parity(Parity::Even);
        assert_eq!(cr1.register_value(), 0b1 << 10);

        cr1.set_parity(Parity::Odd);
        assert_eq!(cr1.register_value(), 0b11 << 9);
    }

    #[test]
    fn test_cr1_set_over8() {
        let cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.register_value(), 0b0);
        assert_eq!(cr1.get_over8(), false);

        cr1.set_over8(true);
        assert_eq!(cr1.register_value(), 0b1 << 15);
        assert_eq!(cr1.get_over8(), true);

        cr1.set_over8(false);
        assert_eq!(cr1.register_value(), 0b0);
        assert_eq!(cr1.get_over8(), false);
    }

    #[test]
    fn test_cr1_get_over8() {
        let cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.register_value(), 0b0);
        assert_eq!(cr1.get_over8(), false);

        cr1.set_over8(true);
        assert_eq!(cr1.register_value(), 0b1 << 15);
        assert_eq!(cr1.get_over8(), true);

        cr1.set_over8(false);
        assert_eq!(cr1.register_value(), 0b0);
        assert_eq!(cr1.get_over8(), false);

        let cr1a = test::create_initialized_register::<CR1>(0b1111 << 12);
        assert_eq!(cr1a.register_value(), 0b1111 << 12);
    }
}


// Daniel Seitz and RJ Russell

use super::super::Register;
use super::defs::*;

/// Three USART control registers.
#[derive(Copy, Clone)]
pub struct USART_CR {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
}

// TODO Need to implement a clear mask for each register to ensure that
// all register bits are set to zero before re-initializing register to
// necessary values for a specific usart configuration.
impl USART_CR {
    pub fn new(base_addr: u32) -> Self {
        USART_CR {
            cr1: CR1::new(base_addr),
            cr2: CR2::new(base_addr),
            cr3: CR3::new(base_addr),
        }
    }

    pub fn enable_usart(&self) {
        self.cr1.enable_usart(true);
    }

    pub fn is_usart_enabled(&self) -> bool {
        self.cr1.is_usart_enabled()
    }

    pub fn disable_usart(&self) {
        self.cr1.enable_usart(false);
    }

    pub fn set_word_length(&self, length: WordLength) {
        self.cr1.set_word_length(length);
    }

    pub fn set_mode(&self, mode: Mode) {
        self.cr1.set_mode(mode);
    }

    pub fn set_stop_bits(&self, length: Stoplength) {
        self.cr2.set_stop_bits(length);
    }

    pub fn clear_stop_bits(&self) {
        self.cr2.clear_stop_bits();
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
    Rx,
    Tx,
    RxTx,
}

#[derive(Copy, Clone)]
struct CR1 {
    base_addr: u32,
}

impl Register for CR1 {
    fn new(base_addr: u32) -> Self {
        CR1 { base_addr: base_addr }
    }

    fn base_addr(&self) -> u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        ZERO
    }
}

impl CR1 {
    fn enable_usart(&self, enable: bool) { // TODO: Do I need a return type here??
        unsafe {
            let mut reg = self.addr();
            if enable {
                *reg |= CR1_UE;
            }
            else {
                *reg &= !(CR1_UE);
            }
            // TODO: Do I need to check if it was disabled properly??
        }
    }

    fn is_usart_enabled(&self) -> bool {
        unsafe {
            *self.addr() & CR1_UE != 0
        }
    }

    fn set_word_length(&self, length: WordLength) {
        let mask = match length {
            WordLength::Seven => CR1_M1,
            WordLength::Eight => ZERO,
            WordLength::Nine => CR1_M0,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_M0 | CR1_M1);
            *reg |= mask;
        }
    }

    fn set_mode(&self, mode: Mode) {
        let mask = match mode {
            Mode::Rx => CR1_RE,
            Mode::Tx => CR1_TE,
            Mode::RxTx => CR1_RE | CR1_TE,
        }

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_RE | CR1_TE)
            *reg |= mask;
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
    One_and_Half,
    Two,
}

#[derive(Copy, Clone)]
struct CR2 {
    base_addr: u32,
}

impl Register for CR2 {
    fn new(base_addr: u32) -> Self {
        CR2 { base_addr: base_addr }
    }

    fn base_addr(&self) -> u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CR2_offset
    }
}

impl CR2 {
    // TODO: Talk about this method vs previous method
    fn set_stop_bits(&self, length: Stoplength) {
        let mask = match length {
            Stoplength::Half => !(CR2_STOP_BIT0 | CR2_STOP_BIT1),
            Stoplength::One => !(CR2_STOP_BIT0) | CR2_STOP_BIT1,
            Stoplength::One_and_Half => CR2_STOP_BIT0 | !(CR2_STOP_BIT0),
            Stoplength::Two => CR2_STOP_BIT0 | CR2_STOP_BIT1,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR2_STOP_BIT0 | CR2_STOP_BIT1);
            *reg |= mask;
        }
    }

    fn clear_stop_bits(&self) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR2_STOP_BIT0 | CR2_STOP_BIT1);
        }
    }
}

#[derive(Copy, Clone)]
struct CR3 {
    base_addr: u32,
}

impl Register for CR3 {
    fn new(base_addr: u32) -> Self {
        CR3 { base_addr: base_addr }
    }

    fn base_addr(&self) -> u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        CR3_offset
    }
}

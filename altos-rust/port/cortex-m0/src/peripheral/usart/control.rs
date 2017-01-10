// Daniel Seitz and RJ Russell

use super::super::Register;
use self::defs;

/// Three USART control registers.
#[derive(Copy, Clone)]
pub struct USART_CR {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
}

/// Word length can be 7, 8, or 9 bits.
#[derive(Copy, Clone)]
pub enum WordLength {
    Seven,
    Eight,
    Nine,
}

impl USART_CR {
    pub fn new(base_addr: u32) -> Self {
        USART_CR {
            cr1: CR1::new(base_addr),
            cr2: CR2::new(base_addr),
            cr3: CR3::new(base_addr),
        }
    }

    pub fn set_word_length(&self, length: WordLength) {
        self.cr1.set_word_length(length);
    }

    pub fn enable_usart(&self, enable: bool) {
        self.cr1.enable_usart(enable);
    }
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
        let mask = match enable {
            false => ZERO,
            true => CR1_UE,
        };

        unsafe {
            let mut reg = self.addr();
            if enable {
                *reg |= mask;
            }
            else {
                *reg &= !(CR1_UE);
            }
            // TODO: Do I need to check if it was disabled properly??
        }
    }

    fn set_word_length(&self, length: WordLength) {
        let mask = match length {
            WordLength::Seven => M1,
            WordLength::Eight => ZERO,
            WordLength::Nine => M0,
        };

        unsafe {
            let mut reg = self.addr();
            *reg &= !(M0 | M1);
            *reg |= mask;
        }
    }

    fn enable_rx_tx(&self, rx_enable: bool, tx_enable: bool) {

    }
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

}

#[derive(Copy, Clone)]
struct CR3 { base_addr: u32,
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

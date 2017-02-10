
use super::super::Register;
use super::defs::*;

/// Three USART control registers.
#[derive(Copy, Clone, Debug)]
pub struct UsartControl {
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
}

// TODO Need to implement a clear mask for each register to ensure that
// all register bits are set to zero before re-initializing register to
// necessary values for a specific usart configuration.
impl UsartControl {
    pub fn new(base_addr: *const u32) -> Self {
        UsartControl {
            cr1: CR1::new(base_addr),
            cr2: CR2::new(base_addr),
            cr3: CR3::new(base_addr),
        }
    }

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

    pub fn get_rxneie(&self) -> bool {
        self.cr1.get_rxneie()
    }

    pub fn enable_transmit_complete_interrupt(&mut self) {
        self.cr1.set_transmit_complete_interrupt(true);
    }

    pub fn disable_transmit_complete_interrupt(&mut self) {
        self.cr1.set_transmit_complete_interrupt(false);
    }

    pub fn get_tcie(&self) -> bool {
        self.cr1.get_tcie()
    }

    pub fn enable_transmit_interrupt(&mut self) {
        self.cr1.set_transmit_interrupt(true);
    }

    pub fn disable_transmit_interrupt(&mut self) {
        self.cr1.set_transmit_interrupt(false);
    }

    pub fn get_txeie(&self) -> bool {
        self.cr1.get_txeie()
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

    pub fn set_stop_bits(&mut self, length: StopLength) {
        self.cr2.set_stop_bits(length);
    }

    pub fn set_hardware_flow_control(&mut self, hfc: HardwareFlowControl) {
        self.cr3.set_hardware_flow_control(hfc);
    }
}

// ------------------------------------
/// CR1
// ------------------------------------

/// Word length can be 7, 8, or 9 bits.
#[derive(Copy, Clone, Debug)]
pub enum WordLength {
    Seven,
    Eight,
    Nine,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    None,
    Receive,
    Transmit,
    All,
}

#[derive(Copy, Clone, Debug)]
pub enum Parity {
    None,
    Even,
    Odd,
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
    // Enables and disables USARTx based on bool variable passed in.
    fn enable_usart(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_UE);
            if enable {
                *reg |= CR1_UE;
            }
        }
    }

    // Checks if usart is enabled.
    fn is_usart_enabled(&self) -> bool {
        unsafe {
            *self.addr() & CR1_UE != 0
        }
    }

    // Sets mode for receive(Rx), transmit(Tx) or both(RxTx)
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

    fn set_receiver_not_empty_interrupt(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_RXNEIE);
            if enable {
                *reg |= CR1_RXNEIE;
            }
        }
    }

    fn get_rxneie(&self) -> bool {
        unsafe {
            *self.addr() & CR1_RXNEIE != 0
        }
    }

    fn set_transmit_complete_interrupt(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_TCIE);
            if enable {
                *reg |= CR1_TCIE;
            }
        }
    }

    fn get_tcie(&self) -> bool {
        unsafe {
            *self.addr() & CR1_TCIE != 0
        }
    }

    fn set_transmit_interrupt(&mut self, enable: bool) {
        unsafe {
            let mut reg = self.addr();
            *reg &= !(CR1_TXEIE);
            if enable {
                *reg |= CR1_TXEIE;
            }
        }
    }

    fn get_txeie(&self) -> bool {
        unsafe {
            *self.addr() & CR1_TXEIE != 0
        }
    }

    // Sets parity to even or odd.
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

    // Sets wordlength to 7, 8, or 9 bits.
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

    // Sets oversampling by 16 (0) or by 8 (1)
    fn set_over8(&mut self, enable: bool) {
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
#[derive(Copy, Clone, Debug)]
pub enum StopLength {
    Half,
    One,
    OneAndHalf,
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
/// CR3
// ------------------------------------

#[derive(Copy, Clone, Debug)]
pub enum HardwareFlowControl {
    None,
    /// Request to Send
    Rts,
    /// Clear to Send
    Cts,
    /// Both
    RtsCts,
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
    fn set_hardware_flow_control(&mut self, hfc: HardwareFlowControl) {
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
    fn test_cr1_get_txeie_returns_false_when_disabled() {
        let cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.get_txeie(), false);
    }

    #[test]
    fn test_cr1_get_txeie_returns_true_when_enabled() {
        let cr1 = test::create_initialized_register::<CR1>(0b1 << 7);
        assert_eq!(cr1.get_txeie(), true);
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
    fn test_cr1_get_tcie_returns_false_when_disabled() {
        let cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.get_tcie(), false);
    }

    #[test]
    fn test_cr1_get_tcie_returns_true_when_enabled() {
        let cr1 = test::create_initialized_register::<CR1>(0b1 << 6);
        assert_eq!(cr1.get_tcie(), true);
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
    fn test_cr1_get_receiver_not_empty_interrupt_returns_false_when_disabled() {
        let cr1 = test::create_register::<CR1>();
        assert_eq!(cr1.get_rxneie(), false);
    }

    #[test]
    fn test_cr1_get_receiver_not_empty_interrupt_returns_true_when_enabled() {
        let cr1 = test::create_initialized_register::<CR1>(0b1 << 5);
        assert_eq!(cr1.get_rxneie(), true);
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

        cr3.set_hardware_flow_control(HardwareFlowControl::RtsCts);
        assert_eq!(cr3.register_value(), 0b11 << 8);

        cr3.set_hardware_flow_control(HardwareFlowControl::None);
        assert_eq!(cr3.register_value(), 0b0);
    }
}

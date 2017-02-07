use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct ISR {
    base_addr: *const u32,
}

impl Register for ISR {
    fn new(base_addr: *const u32) -> Self {
        ISR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        ISR_OFFSET
    }
}

impl ISR {

    pub fn get_rxne(&self) -> bool {
        unsafe {
            *self.addr() & ISR_RXNE != 0
        }
    }

    pub fn get_tc(&self) -> bool {
        unsafe {
            *self.addr() & ISR_TC != 0
        }
    }

    pub fn get_txe(&self) -> bool {
        unsafe {
            *self.addr() & ISR_TXE != 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_isr_get_rxne_returns_false_when_bit_not_set() {
        let isr = test::create_register::<ISR>();
        assert_eq!(isr.get_rxne(), false);
    }

    #[test]
    fn test_isr_get_rxne_returns_false_when_bit_is_set() {
        let isr = test::create_initialized_register::<ISR>(0b1 << 5);
        assert_eq!(isr.get_rxne(), true);
    }

    #[test]
    fn test_isr_get_tc_returns_false_when_bit_not_set() {
        let isr = test::create_register::<ISR>();
        assert_eq!(isr.get_tc(), false);
    }

    #[test]
    fn test_isr_get_tc_returns_true_when_bit_is_set() {
        let isr = test::create_initialized_register::<ISR>(0b1 << 6);
        assert_eq!(isr.get_tc(), true);
    }

    #[test]
    fn test_isr_get_txe_returns_false_when_bit_not_set() {
        let isr = test::create_register::<ISR>();
        assert_eq!(isr.get_txe(), false);
    }

    #[test]
    fn test_isr_get_txe_returns_true_when_bit_is_set() {
        let isr = test::create_initialized_register::<ISR>(1 << 7);
        assert_eq!(isr.get_txe(), true);
    }
}

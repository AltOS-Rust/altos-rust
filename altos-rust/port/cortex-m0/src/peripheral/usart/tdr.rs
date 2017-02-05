use super::super::Register;
use super::defs::*;

#[derive(Copy, Clone, Debug)]
pub struct TDR {
    base_addr: *const u32,
}

impl Register for TDR {
    fn new(base_addr: *const u32) -> Self {
        TDR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        TDR_OFFSET
    }
}

impl TDR {
    pub fn store(&mut self, byte: u8) {
        unsafe {
            let mut reg = self.addr();
            reg.store(byte as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test;

    #[test]
    fn test_tdr_has_ascii_value_of_97_on_store_of_char_a() {
        let mut tdr = test::create_register::<TDR>();
        tdr.store(b'a');
        assert_eq!(tdr.register_value(), 97);
    }

    #[test]
    fn test_tdr_has_ascii_value_of_98_on_last_store_of_char_b() {
        let mut tdr = test::create_register::<TDR>();
        tdr.store(b'i');
        tdr.store(b'z');
        tdr.store(b'b');
        assert_eq!(tdr.register_value(), 98);
    }
}

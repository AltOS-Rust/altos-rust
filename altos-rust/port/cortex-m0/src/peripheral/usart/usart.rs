use super::super::Control;
use volatile::Volatile;
use super::control::USART_CR;
use super::baudr::USART_BRR;
use peripheral::usart::defs::*;

#[derive(Copy, Clone)]
enum USARTx {
    USART1,
    USART2,
}

pub struct USART {
    mem_addr: u32,
    control: USART_CR,
    baud: USART_BRR,
}

impl Control for USART {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr as *const u32)
    }
}

impl USART {
    fn new(x: USARTx) -> Self {
        match x {
            USARTx::USART1 => USART {
                mem_addr: USART1_ADDR,
                control: USART_CR::new(USART1_ADDR),
                baud: USART_BRR::new(USART1_ADDR),
            },
            USARTx::USART2 => USART {
                mem_addr: USART2_ADDR,
                control: USART_CR::new(USART2_ADDR),
                baud: USART_BRR::new(USART2_ADDR),
            },
        }
    }
}

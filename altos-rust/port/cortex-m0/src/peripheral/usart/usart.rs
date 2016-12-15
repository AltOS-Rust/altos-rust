mod defs;
mod control;
mod baudr;

use super::Control;
use volatile::Volatile;
use self::control::USART_CR;
use self::baudr::USART_BRR;

#[derive(Copy, Clone)]
enum USARTx {
    USART1,
    USART2,
}

struct USART {
    mem_addr: usize,
    control: USART_CR,
    baud: USART_BRR,
}

impl Control for USART {
    unsafe fn mem_addr(&self) -> Volatile<usize> {
        Volatile::new(self.mem_addr as *const usize)
    }
}

impl USART {
    fn new(x: USARTx) -> Self {
        match x {
            USARTx::USART1 => USART {
                mem_addr: USART1,
                control: USART_CR::new(USART1),
                baud: USART_BRR::new(USART1),
            },
            USARTx::USART2 => USART {
                mem_addr: USART2,
                control: USART_CR::new(USART2),
                baud: USART_BRR::new(USART2),
            },
        }
    }
}

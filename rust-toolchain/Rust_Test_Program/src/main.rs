#![feature(lang_items)]
#![feature(asm)]
#![no_std]
#![no_main]

mod exceptions;
mod gpio;

#[no_mangle]
pub fn start() -> ! {
  /*
  let mut i = 0;
  unsafe {
    let ram_boundary = *(0x0000_0000 as *const u32);  // Get stack boundary
    let crash = *(ram_boundary as *const u32);        // Crash the program
  }


  loop {
    i += 1;
  }
  */

  gpio::GPIO::enable(gpio::GPIOGroup::B);

  //turn_on_gpiob();

  let mut pb3 = gpio::GPIOPort::new(3, gpio::GPIOGroup::B);
  pb3.set_mode(gpio::GPIOMode::Output);
  pb3.set_type(gpio::GPIOType::PushPull);
  //put_pb3_in_output_mode();

  // Just looking...
  let pb3_mode = pb3.get_mode();
  let pb3_type = pb3.get_type();
  
  let mut ticks: u32 = 5_000;
  loop {
    //set_pb3_high();
    pb3.set();
    delay(ticks);
    //set_pb3_low();
    pb3.reset();
    delay(ticks);
  }

}

fn delay(n: u32) {
  for _ in 0..n {}
}

/*
fn turn_on_gpiob() {
  // Start address of the RCC register block
  const RCC: u32 = 0x4002_1000;

  const RCC_AHBENR: u32 = 0x14;

  const RCC_AHBENR_IOPBEN: u32 = (1 << 18);

  unsafe {
    let ahbenr = (RCC + RCC_AHBENR) as *mut u32;

    *ahbenr |= RCC_AHBENR_IOPBEN; // Enable GPIOB
  }
}

const GPIOB: u32 = 0x4800_0400;
const GPIOB_BSRR: u32 = 0x18;

fn put_pb3_in_output_mode() {
  const GPIOB_MODER: u32 = 0x0;
  const GPIOB_OTYPER: u32 = 0x4;

  unsafe {
    let moder = (GPIOB + GPIOB_MODER) as *mut u32;
    let otyper = (GPIOB + GPIOB_OTYPER) as *mut u32;

    *moder |= 0b01 << 6;  // Set mode to general purpose output
    *otyper |= 0b0 << 3;  // Set type to output push-pull
  }
}

fn set_pb3_high() {
  unsafe {
    let bsrr = (GPIOB + GPIOB_BSRR) as *mut u32;

    *bsrr |= 1 << 3;
  }
}

fn set_pb3_low() {
  unsafe {
    let bsrr = (GPIOB + GPIOB_BSRR) as *mut u32;

    *bsrr |= 1 << (16 + 3);
  }
}
*/

mod vector_table {
  #[link_section = ".reset"]
  static RESET: fn() -> ! = ::start;
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

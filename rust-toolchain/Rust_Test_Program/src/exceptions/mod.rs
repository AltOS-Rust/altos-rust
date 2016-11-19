
#[link_section = ".exceptions"]
static EXCEPTIONS: [Option<fn() -> !>; 14] = [Some(default_handler),  // NMI
                                              Some(default_handler),  // Hard Fault
                                              Some(default_handler),  // Memory Management Fault
                                              Some(default_handler),  // Bus Fault
                                              Some(default_handler),  // Usage Fault
                                              None,                   // Reserved
                                              None,                   // Reserved
                                              None,                   // Reserved
                                              None,                   // Reserved
                                              Some(default_handler),  // SVCall
                                              None,                   // Reserved for Debug
                                              None,                   // Reserved
                                              Some(default_handler),  // PendSV
                                              Some(default_handler)]; // SysTick
                                              


pub fn default_handler() -> ! {
  unsafe { asm!("bkpt"); }

  loop {}
}



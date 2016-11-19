
/// An IO group containing up to 16 pins. For
/// some reason the datasheet shows the memory
/// for groups D and E as reserved, so for now
/// they are left out.
#[derive(Copy, Clone)]
pub enum GPIOGroup {
  A,
  B,
  C,
  F,
}

/// The IO mode of a port.
#[derive(Copy, Clone)]
pub enum GPIOMode {
  Input,
  Output,
  Alternate,
  Analog,
}

/// The IO output type of a port.
#[derive(Copy, Clone)]
pub enum GPIOType {
  PushPull,
  OpenDrain,
}

/// A GPIO contains the base address for a 
/// memory mapped GPIO group associated with
/// it.
#[derive(Copy, Clone)]
pub struct GPIO {
  mem_addr: u32,
}

impl GPIO {
  fn Group(group: GPIOGroup) -> GPIO {
    match group {
      GPIOGroup::A => GPIO::new(0x4800_0000),
      GPIOGroup::B => GPIO::new(0x4800_0400),
      GPIOGroup::C => GPIO::new(0x4800_0800),
      GPIOGroup::F => GPIO::new(0x4800_1400),
    }
  }

  fn new(mem_addr: u32) -> GPIO {
    GPIO { mem_addr: mem_addr }
  }
  
  /// Enable a GPIO group, you must do this before you can set any
  /// pins within a group.
  /// 
  /// Example Usage:
  /// ```
  ///   GPIO::enable(GPIOGroup::B); // Enable IO group B (LED is pb3)
  /// ```
  pub fn enable(group: GPIOGroup) {
    //TODO: Refactor RCC and other control registers into their own structs
    // RCC is a control register
    const RCC: u32 = 0x4002_1000; 
    // AHBENR handles enabling/disabling peripherals, we want it for GPIO in our case
    const RCC_AHBENR: u32 = 0x14; 
    
    // Get the register bit that should be set to enable this group
    let io_group_enable: u32 = match group {
      GPIOGroup::A => 1 << 17,
      GPIOGroup::B => 1 << 18,
      GPIOGroup::C => 1 << 19,
      GPIOGroup::F => 1 << 22,
    };
    unsafe {
      let ahbenr = (RCC + RCC_AHBENR) as *mut u32;
      *ahbenr |= io_group_enable;
    }
  }
}

/// A specific GPIO port. You can modify the mode it is set to
/// and set the pin high or low with the .set() and .reset() methods
/// respectively.
///
/// Example Usage:
/// ```
///   let mut port = GPIOPort::new(3, GPIOGroup::B); // The port to the user LED
///   port.set_mode(GPIOMode::Output);
///   port.set_type(GPIOType::PushPull);
///   port.set(); // Light's green!
/// ```
pub struct GPIOPort {
  group: GPIOGroup,
  port: u8,
}

impl GPIOPort {
  /// Create a new port for the associated group. Ports are NOT thread safe, if you
  /// must ensure an atomic set of operations on a port use some kind of synchronization
  /// tool 
  // TODO: Create synchronization tool...
  pub fn new(port: u8, group: GPIOGroup) -> GPIOPort {
    if port > 15 {
      //TODO: Handle this more gracefully hopefully
      panic!("GPIOPort::new - port must be a value between 0..15");
    }
    GPIOPort {
      group: group,
      port: port,
    }
  }

  pub fn get_mode(&self) -> GPIOMode {
    let gpio = GPIO::Group(self.group);
    let set_bits = unsafe {
      // The mode register is at offset 0x0, so no need to add anything
      let moder = gpio.mem_addr as *mut u32;
      // The mode field is 2 bits wide, so shift over 2 * port_num to get to the right field
      (*moder & (0b11 << (self.port * 2))) >> (self.port * 2)
    };

    match set_bits {
      0b00 => GPIOMode::Input,
      0b01 => GPIOMode::Output,
      0b10 => GPIOMode::Alternate,
      0b11 => GPIOMode::Analog,
      _    => panic!("GPIOPort::mode - set bits gave an unknown value!"),
    }
  }

  pub fn set_mode(&mut self, mode: GPIOMode) {
    let gpio = GPIO::Group(self.group);
    let mask = match mode {
      GPIOMode::Input     => 0b00,
      GPIOMode::Output    => 0b01,
      GPIOMode::Alternate => 0b10,
      GPIOMode::Analog    => 0b11,
    };

    unsafe {
      let moder = gpio.mem_addr as *mut u32;
      // Again, the mode field for each port is 2 bits wide, so shift 2 * port_num
      *moder |= mask << (self.port * 2);
    }
  }

  pub fn get_type(&self) -> GPIOType {
    const OTYPER: u32 = 0x4;
    let gpio = GPIO::Group(self.group);
    let set_bits = unsafe {
      let otyper = (gpio.mem_addr + OTYPER) as *mut u32;
      (*otyper & (0b1 << self.port)) >> self.port
    };

    match set_bits {
      0b0 => GPIOType::PushPull,
      0b1 => GPIOType::OpenDrain,
      _   => panic!("GPIOPort::type - set bits gave an unknown value!"),
    }
  }

  pub fn set_type(&mut self, p_type: GPIOType) {
    const OTYPER: u32 = 0x4;
    let gpio = GPIO::Group(self.group);
    let mask = match p_type {
      GPIOType::PushPull  => 0b0,
      GPIOType::OpenDrain => 0b1,
    };

    unsafe {
      let otyper = (gpio.mem_addr + OTYPER) as *mut u32;
      *otyper |= mask << self.port;
    }
  }

  /// Sets the pin high.
  pub fn set(&self) {
    const BSRR: u32 = 0x18;
    let gpio = GPIO::Group(self.group);
    unsafe {
      let bsrr = (gpio.mem_addr + BSRR) as *mut u32;
      // The low half of the register asserts the pin
      *bsrr |= 1 << self.port;
    }
  }

  /// Sets the pin low.
  pub fn reset(&self) {
    const BSRR: u32 = 0x18;
    let gpio = GPIO::Group(self.group);
    unsafe {
      let bsrr = (gpio.mem_addr + BSRR) as *mut u32;
      // The high half deasserts the pin, so add 16 to the port_num
      *bsrr |= 1 << (16 + self.port);
    }
  }
}

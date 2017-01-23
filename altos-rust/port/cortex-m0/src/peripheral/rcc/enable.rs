// peripheral/rcc/enable.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! This module is used to control the AHBENR (AHB peripheral enable register) which controls the
//! clock to the peripherals controled by the AHB clock.

use super::super::{Register, Field};

#[derive(Copy, Clone)]
pub enum Peripheral {
  // AHB Peripherals
  TouchSenseController,
  GPIOA,
  GPIOB,
  GPIOC,
  GPIOF,
  CRC,
  FLITF,
  SRAMInterface,
  DMA,
  DMA2,

  // APB1 Peripherals
  CEC,
  DAC,
  PowerInterface,
  ClockRecoverySystem,
  CAN,
  USB,
  I2C1,
  I2C2,
  USART2,
  USART3,
  USART4,
  USART5,
  SPI2,
  WindowWatchdog,
  TIM2,
  TIM3,
  TIM6,
  TIM7,
  TIM14,

  // APB2 Peripherals
  MCUDebug,
  TIM1,
  TIM15,
  TIM16,
  TIM17,
  USART1,
  USART6,
  USART7,
  USART8,
  SPI1,
  ADC,
  SysCfgComp,
}

impl Field for Peripheral {
  fn mask(&self) -> u32 {
    match *self {
      // AHB Peripherals
      Peripheral::TouchSenseController => 0b1 << 24,
      Peripheral::GPIOA => 0b1 << 17,
      Peripheral::GPIOB => 0b1 << 18,
      Peripheral::GPIOC => 0b1 << 19,
      Peripheral::GPIOF => 0b1 << 22,
      Peripheral::CRC => 0b1 << 6,
      Peripheral::FLITF => 0b1 << 4,
      Peripheral::SRAMInterface => 0b1 << 2,
      Peripheral::DMA => 0b1 << 0,
      Peripheral::DMA2 => 0b1 << 1,

      // APB1 Peripherals
      Peripheral::CEC => 0b1 << 30,
      Peripheral::DAC => 0b1 << 29,
      Peripheral::PowerInterface => 0b1 << 28,
      Peripheral::ClockRecoverySystem => 0b1 << 27,
      Peripheral::CAN => 0b1 << 25,
      Peripheral::USB => 0b1 << 23,
      Peripheral::I2C1 => 0b1 << 21,
      Peripheral::I2C2 => 0b1 << 22,
      Peripheral::USART2 => 0b1 << 17,
      Peripheral::USART3 => 0b1 << 18,
      Peripheral::USART4 => 0b1 << 19,
      Peripheral::USART5 => 0b1 << 20,
      Peripheral::SPI2 => 0b1 << 14,
      Peripheral::WindowWatchdog => 0b1 << 11,
      Peripheral::TIM2 => 0b1 << 0,
      Peripheral::TIM3 => 0b1 << 1,
      Peripheral::TIM6 => 0b1 << 4,
      Peripheral::TIM7 => 0b1 << 5,
      Peripheral::TIM14 => 0b1 << 8,

      // APB2 Peripherals
      Peripheral::MCUDebug => 0b1 << 22,
      Peripheral::TIM1 => 0b1 << 11,
      Peripheral::TIM15 => 0b1 << 16,
      Peripheral::TIM16 => 0b1 << 17,
      Peripheral::TIM17 => 0b1 << 18,
      Peripheral::USART1 => 0b1 << 14,
      Peripheral::USART6 => 0b1 << 5,
      Peripheral::USART7 => 0b1 << 6,
      Peripheral::USART8 => 0b1 << 7,
      Peripheral::SPI1 => 0b1 << 12,
      Peripheral::ADC => 0b1 << 9,
      Peripheral::SysCfgComp => 0b1 << 0,
    }
  }
}

#[derive(Copy, Clone)]
pub struct PeripheralControl {
  ahbenr: AHBENR,
  apbenr1: APBENR1,
  apbenr2: APBENR2,
}

impl PeripheralControl {
  pub fn new(base_addr: u32) -> Self {
    PeripheralControl {
      ahbenr: AHBENR::new(base_addr),
      apbenr1: APBENR1::new(base_addr),
      apbenr2: APBENR2::new(base_addr),
    }
  }

  pub fn enable_peripheral(&self, peripheral: Peripheral) {
    self.set_control_register(true, peripheral);
  }

  pub fn disable_peripheral(&self, peripheral: Peripheral) {
    self.set_control_register(false, peripheral);
  }

  pub fn peripheral_is_enabled(&self, peripheral: Peripheral) -> bool {
    if self.ahbenr.serves_peripheral(peripheral) {
      self.ahbenr.get_enable(peripheral)
    }
    else if self.apbenr1.serves_peripheral(peripheral) {
      self.apbenr1.get_enable(peripheral)
    }
    else if self.apbenr2.serves_peripheral(peripheral) {
      self.apbenr2.get_enable(peripheral)
    }
    else {
      panic!("PeripheralControl::peripheral_is_enabled - specified peripheral not served, did you
      forget to add it to a control register?");
    }
  }

  fn set_control_register(&self, enable: bool, peripheral: Peripheral) {
    if self.ahbenr.serves_peripheral(peripheral) {
      self.ahbenr.set_enable(enable, peripheral);
    }
    else if self.apbenr1.serves_peripheral(peripheral) {
      self.apbenr1.set_enable(enable, peripheral);
    }
    else if self.apbenr2.serves_peripheral(peripheral) {
      self.apbenr2.set_enable(enable, peripheral);
    }
    else {
      panic!("PeripheralControl::set_control_register - specified peripheral not served, did you
      forget to add it to a control register?");
    }
  }
}

#[derive(Copy, Clone)]
struct AHBENR {
  base_addr: u32,
}

impl Register for AHBENR {
  fn new(base_addr: u32) -> Self {
    AHBENR { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x14
  }
}

impl AHBENR {
  fn get_enable(&self, peripheral: Peripheral) -> bool {
    if !self.serves_peripheral(peripheral) {
      panic!("AHBENR::get_enable - this register does not control the specified peripheral!");
    }
    let mask = peripheral.mask();

    unsafe {
      let reg = self.addr();

      *reg & mask != 0
    }
  }

  fn set_enable(&self, enable: bool, peripheral: Peripheral) {
    if !self.serves_peripheral(peripheral) {
      panic!("AHBENR::enable - This register does not control the specified peripheral!");
    }
    let mask = peripheral.mask();

    unsafe {
      let mut reg = self.addr();
      if enable {
        *reg |= mask;
      }
      else {
        *reg &= !mask;
      }
    }
  }

  fn serves_peripheral(&self, peripheral: Peripheral) -> bool {
    match peripheral {
      Peripheral::TouchSenseController | Peripheral::GPIOA | 
      Peripheral::GPIOB | Peripheral::GPIOC | Peripheral::GPIOF | 
      Peripheral::CRC | Peripheral::FLITF | Peripheral::SRAMInterface | 
      Peripheral::DMA | Peripheral::DMA2 => true,
      _ => false,
    }
  }
}

#[derive(Copy, Clone)]
struct APBENR1 {
  base_addr: u32,
}

impl Register for APBENR1 {
  fn new(base_addr: u32) -> Self {
    APBENR1 { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x1C
  }
}

impl APBENR1 {
  fn get_enable(&self, peripheral: Peripheral) -> bool {
    if !self.serves_peripheral(peripheral) {
      panic!("APBENR1::get_enable - this register does not control the specified peripheral!");
    }
    let mask = peripheral.mask();

    unsafe {
      let reg = self.addr();

      *reg & mask != 0
    }
  }

  fn set_enable(&self, enable: bool, peripheral: Peripheral) {
    if !self.serves_peripheral(peripheral) {
      panic!("APBENR1::enable - This register does not control the specified peripheral!");
    }
    let mask = peripheral.mask();

    unsafe {
      let mut reg = self.addr();
      if enable {
        *reg |= mask;
      }
      else {
        *reg &= !mask;
      }
    }
  }

  fn serves_peripheral(&self, peripheral: Peripheral) -> bool {  
    match peripheral {
      Peripheral::CEC | Peripheral::DAC | Peripheral::PowerInterface | 
      Peripheral::ClockRecoverySystem | Peripheral::CAN | Peripheral::USB | 
      Peripheral::I2C1 | Peripheral::I2C2 | Peripheral::USART2 | 
      Peripheral::USART3 | Peripheral::USART4 | Peripheral::USART5 | 
      Peripheral::SPI2 | Peripheral::WindowWatchdog | Peripheral::TIM2 | 
      Peripheral::TIM3 | Peripheral::TIM6 | Peripheral::TIM7 | Peripheral::TIM14 => true,
      _ => false,
    }
  }
}

#[derive(Copy, Clone)]
struct APBENR2 {
  base_addr: u32,
}

impl Register for APBENR2 {
  fn new(base_addr: u32) -> Self {
    APBENR2 { base_addr: base_addr }
  }

  fn base_addr(&self) -> u32 {
    self.base_addr
  }

  fn mem_offset(&self) -> u32 {
    0x18
  }
}

impl APBENR2 {
  fn get_enable(&self, peripheral: Peripheral) -> bool {
    if !self.serves_peripheral(peripheral) {
      panic!("APBENR2::get_enable - this register does not control the specified peripheral!");
    }
    let mask = peripheral.mask();

    unsafe {
      let reg = self.addr();

      *reg & mask != 0
    }
  }

  fn set_enable(&self, enable: bool, peripheral: Peripheral) {
    if !self.serves_peripheral(peripheral) {
      panic!("APBENR2::enable - This register does not control the specified peripheral!");
    }
    let mask = peripheral.mask();

    unsafe {
      let mut reg = self.addr();
      if enable {
        *reg |= mask;
      }
      else {
        *reg &= !mask;
      }
    }
  }
  
  fn serves_peripheral(&self, peripheral: Peripheral) -> bool {
    match peripheral {
      Peripheral::MCUDebug | Peripheral::TIM1 | Peripheral::TIM15 | 
      Peripheral::TIM16 | Peripheral::TIM17 | Peripheral::USART1 | 
      Peripheral::USART6 | Peripheral::USART7 | Peripheral::USART8 | 
      Peripheral::SPI1 | Peripheral::ADC | Peripheral::SysCfgComp => true,
      _ => false,
    }
  }
}

/*
* Copyright (C) 2017 AltOS-Rust Team
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU General Public License for more details.
*
* You should have received a copy of the GNU General Public License
* along with this program. If not, see <http://www.gnu.org/licenses/>.
*/

//! This module is used to control the AHBENR (AHB peripheral enable register), which controls the
//! clock to the peripherals controled by the AHB clock.

use super::super::{Register, Field};
use super::defs::*;

/// Defines available peripherals.
#[allow(missing_docs)]
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
            Peripheral::TouchSenseController => TSCEN,
            Peripheral::GPIOA => IOPAEN,
            Peripheral::GPIOB => IOPBEN,
            Peripheral::GPIOC => IOPCEN,
            Peripheral::GPIOF => IOPFEN,
            Peripheral::CRC => CRCEN,
            Peripheral::FLITF => FLITFEN,
            Peripheral::SRAMInterface => SRAMEN,
            Peripheral::DMA => DMAEN,
            Peripheral::DMA2 => DMA2EN,

            // APB1 Peripherals
            Peripheral::CEC => CECEN,
            Peripheral::DAC => DACEN,
            Peripheral::PowerInterface => PWREN,
            Peripheral::ClockRecoverySystem => CRSEN,
            Peripheral::CAN => CANEN,
            Peripheral::USB => USBEN,
            Peripheral::I2C1 => I2C1EN,
            Peripheral::I2C2 => I2C2EN,
            Peripheral::USART2 => USART2EN,
            Peripheral::USART3 => USART3EN,
            Peripheral::USART4 => USART4EN,
            Peripheral::USART5 => USART5EN,
            Peripheral::SPI2 => SPI2EN,
            Peripheral::WindowWatchdog => WWDGEN,
            Peripheral::TIM2 => TIM2EN,
            Peripheral::TIM3 => TIM3EN,
            Peripheral::TIM6 => TIM6EN,
            Peripheral::TIM7 => TIM7EN,
            Peripheral::TIM14 => TIM14EN,

            // APB2 Peripherals
            Peripheral::MCUDebug => DBGMCUEN,
            Peripheral::TIM1 => TIM1EN,
            Peripheral::TIM15 => TIM15EN,
            Peripheral::TIM16 => TIM16EN,
            Peripheral::TIM17 => TIM17EN,
            Peripheral::USART1 => USART1EN,
            Peripheral::USART6 => USART6EN,
            Peripheral::USART7 => USART7EN,
            Peripheral::USART8 => USART8EN,
            Peripheral::SPI1 => SPI1EN,
            Peripheral::ADC => ADCEN,
            Peripheral::SysCfgComp => SYSCFGCOMPEN,
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
    pub fn new(base_addr: *const u32) -> Self {
        PeripheralControl {
            ahbenr: AHBENR::new(base_addr),
            apbenr1: APBENR1::new(base_addr),
            apbenr2: APBENR2::new(base_addr),
        }
    }

    pub fn enable_peripheral(&mut self, peripheral: Peripheral) {
        self.set_control_register(true, peripheral);
    }

    pub fn disable_peripheral(&mut self, peripheral: Peripheral) {
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

    fn set_control_register(&mut self, enable: bool, peripheral: Peripheral) {
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
    base_addr: *const u32,
}

impl Register for AHBENR {
    fn new(base_addr: *const u32) -> Self {
        AHBENR { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        AHBENR_OFFSET
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

    fn set_enable(&mut self, enable: bool, peripheral: Peripheral) {
        if !self.serves_peripheral(peripheral) {
            panic!("AHBENR::enable - This register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        unsafe {
            let mut reg = self.addr();
            *reg &= !mask;
            if enable {
                *reg |= mask;
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
    base_addr: *const u32,
}

impl Register for APBENR1 {
    fn new(base_addr: *const u32) -> Self {
        APBENR1 { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        APBENR1_OFFSET
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

    fn set_enable(&mut self, enable: bool, peripheral: Peripheral) {
        if !self.serves_peripheral(peripheral) {
            panic!("APBENR1::enable - This register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        unsafe {
            let mut reg = self.addr();
            *reg &= !mask;
            if enable {
                *reg |= mask;
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
    base_addr: *const u32,
}

impl Register for APBENR2 {
    fn new(base_addr: *const u32) -> Self {
        APBENR2 { base_addr: base_addr }
    }

    fn base_addr(&self) -> *const u32 {
        self.base_addr
    }

    fn mem_offset(&self) -> u32 {
        APBENR2_OFFSET
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

    fn set_enable(&mut self, enable: bool, peripheral: Peripheral) {
        if !self.serves_peripheral(peripheral) {
            panic!("APBENR2::enable - This register does not control the specified peripheral!");
        }
        let mask = peripheral.mask();

        unsafe {
            let mut reg = self.addr();
            *reg &= !mask;
            if enable {
                *reg |= mask;
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

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

// Need module description/doc comment here

mod defs;
mod isr;
mod ier;
mod cr;
mod dr;

use super::{Control, Register};
use volatile::Volatile;
use self::cr::CR;
use self::isr::ISR;
use self::ier::IER;
use self::dr::DR;
use self::defs::*;
use peripheral::{rcc, gpio};
// use interrupt;

// #[derive(Copy, Clone, Debug)]
pub struct Adc {
    // Memory address of the ADC
    mem_addr: *const u32,
    // Control register
    cr: CR,
    // Interrupt and status register
    isr: ISR,
    // Interrupt enable register
    ier: IER,
    // Data register
    dr: DR,

    // Collection of configuration registers?
    // cfgr1: CFGR1,
    // cfgr2: CFGR2,
    // smpr: SMPR,
    // chselr: CHSELR,

    // tr: TR,
    // ccr: CCR,
}

impl Control for Adc {
    unsafe fn mem_addr(&self) -> Volatile<u32> {
        Volatile::new(self.mem_addr as *const u32)
    }
}

impl Adc {
    // Creates a new Adc object to allow for configuration of the ADC peripheral.
    pub fn new() -> Self {
        Adc {
            mem_addr: ADC_ADDR,
            cr: CR::new(ADC_ADDR),
            isr: ISR::new(ADC_ADDR),
            ier: IER::new(ADC_ADDR),
            dr: DR::new(ADC_ADDR),
        }
    }

    // Functions here...
    pub fn enable_adc(&mut self) {}
}

/// Initialize the ADC peripheral.
///
/// Connects the necessary GPIO pins, sets the clock, enables interrupts?
pub fn init() {
    // Notes from ADC_Config in example code:

    // Enable GPIOC clock
    // Enable ADC1 clock

    let rcc = rcc::rcc();
    rcc.enable_peripheral(rcc::Peripheral::ADC);

    // Configure ADC Channel(Which one?) as analog input ===>
    // GPIO pin: 1 or 0?
    // GPIO mode: GPIO_Mode_AN
    // GPIO PuPd: GPIO_PuPd_NOPULL
    // Init GPIO

    gpio::GPIO::enable(gpio::Group::C);
    let mut pa0 = gpio::Port::new(0, gpio::Group::C); // Correct pin?

    // pa0.set_function(gpio::AlternateFunction::One);
    // pa0.set_speed(gpio::Speed::High);
    pa0.set_mode(gpio::Mode::Analog); // Is this right?
    pa0.set_type(gpio::Type::PushPull); // Should this be OpenDrain or PushPull?
    pa0.set_pull(gpio::Pull::Neither);

    let mut adc1 = Adc::new();

    // ADC DeInit (?)
    // Initialize ADC structure (?)

    // Configure ADC options ===>
    // Resolution: 12 bits?
    // Continuous Conversion Mode: Enable
    // ExternalTrigConvEdge (?): None
    // DataAlign: Right
    // ScanDirection: Upward
    // Init ADC

    // ADC channel config (239.5 Cycles as sampling time)

    // ADC Calibration (Get calibration factor for ADC1)
    // Enable ADC Peripheral (ADC1)
    // While loop spin: Wait for ADRDY flag
    // ADC start of conversion (ADC1)
}

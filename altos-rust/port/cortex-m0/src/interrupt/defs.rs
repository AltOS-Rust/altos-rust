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

// Defines all the perpherials that have interrupts.
#[derive(Copy, Clone)]
pub enum Hardware {
    WWDG = 0,
    PVDVDDIO2 = 1,
    RTC = 2,
    FLASH = 3,
    RCCCRS = 4,
    EXTI01 = 5,
    EXTI23 = 6,
    EXTI415 = 7,
    TSC = 8,
    DMACH1 = 9,
    DMACH23 = 10,
    DMACH4PLUS = 11,
    ADCCOMP = 12,
    TIM1BRKUP = 13,
    TIM1CC = 14,
    TIM2 = 15,
    TIM3 = 16,
    TIM6 = 17,
    TIM7 = 18,
    TIM14 = 19,
    TIM15 = 20,
    TIM16 = 21,
    TIM17 = 22,
    I2C1 = 23,
    I2C2 = 24,
    SPI1 = 25,
    SPI2 = 26,
    USART1 = 27,
    USART2 = 28,
    USART3PLUS = 29,
    CECCAN = 30,
    USB = 31,
}

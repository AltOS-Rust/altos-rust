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

pub use self::imp::*;

#[cfg(target_arch="arm")]
mod imp {
    #[inline(always)]
    pub unsafe fn dmb() {
        asm!("dmb"
            : /* no outputs */
            : /* no inputs */
            : /* no clobbers */
            : "volatile"
        );
    }

    #[inline(always)]
    pub unsafe fn dsb() {
        asm!("dsb"
            : /* no outputs */
            : /* no inputs */
            : /* no clobbers */
            : "volatile"
        );
    }

    #[inline(always)]
    pub unsafe fn bkpt() {
        asm!("bkpt"
            : /* no outputs */
            : /* no inputs */
            : /* no clobbers */
            : "volatile"
        );
    }

    #[inline(always)]
    pub unsafe fn enable_interrupts() {
        asm!("cpsie i"
            : /* no outputs */
            : /* no inputs */
            : /* no clobbers */
            : "volatile"
        );
    }

    #[inline(always)]
    pub unsafe fn disable_interrupts() {
        asm!("cpsid i"
            : /* no outputs */
            : /* no inputs */
            : /* no clobbers */
            : "volatile"
        );
    }

    #[inline(always)]
    pub unsafe fn wfi() {
        asm!("wfi"
            : /* no outputs */
            : /* no inputs */
            : /* no clobbers */
            : "volatile"
        );
    }

    pub unsafe fn get_control() -> usize {
        let result: usize;
        asm!("mrs $0, CONTROL"
            : "=r"(result)
            : /* no inputs */
            : /* no clobbers */
            : "volatile"
        );
        result
    }
}

#[cfg(not(target_arch="arm"))]
mod imp {
    #[inline(always)]
    pub unsafe fn dmb() {}

    #[inline(always)]
    pub unsafe fn dsb() {}

    #[inline(always)]
    pub unsafe fn bkpt() {}

    #[inline(always)]
    pub unsafe fn enable_interrupts() {}

    #[inline(always)]
    pub unsafe fn disable_interrupts() {}

    #[inline(always)]
    pub unsafe fn wfi() {}

    #[inline(always)]
    pub unsafe fn get_control() -> usize { 0 }
}

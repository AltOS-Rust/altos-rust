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

//! This module handles defining exception handlers.

// NOTE: When using system calls in these handlers, use the `sys_*` versions of those calls. With
// the `svc` feature enabled, system calls are handled through the `sv_call` handler. These system
// calls essentially just call the `sys_*` versions, but we can guarantee at that point that it's
// an interrupt free environment. If, however, you do execute an `svc` instruction while either
// within another interrupt handler or with interrupts disabled you will encounter a hard fault.
//
// If the `svc` feature is not enabled, these system calls have the same behavior, so there should
// be no difference.

#[cfg(feature="serial")]
mod usart;

use arm::asm::bkpt;
use altos_core::syscall;

// Interrupt vector table
#[cfg(not(test))]
#[cfg(target_arch="arm")]
#[doc(hidden)]
#[export_name="_EXCEPTIONS"]
pub static EXCEPTIONS: [Option<unsafe extern "C" fn()>; 46] = [
    Some(default_handler),  // NMI: 1
    Some(default_handler),  // Hard Fault: 2
    Some(default_handler),  // Memory Management Fault: 3
    Some(default_handler),  // Bus Fault: 4
    Some(default_handler),  // Usage Fault: 5
    None,                   // Reserved: 6
    None,                   // Reserved: 7
    None,                   // Reserved: 8
    None,                   // Reserved: 9
    Some(sv_call_handler),  // SVCall: 10
    None,                   // Reserved for Debug: 11
    None,                   // Reserved: 12
    Some(pend_sv_handler),  // PendSV: 13
    Some(systick_handler),  // SysTick: 14
    Some(default_handler),  // Window Watchdog: 15
    Some(default_handler),  // PVD_VDDIO2: 16
    Some(default_handler),  // Real Time Clock: 17
    Some(default_handler),  // Flash global: 18
    Some(default_handler),  // RCC and CRS global: 19
    Some(default_handler),  // EXTI Line[1:0]: 20
    Some(default_handler),  // EXTI Line[3:2]: 21
    Some(default_handler),  // EXTI Line[15:4]: 22
    Some(default_handler),  // Touch Sensing: 23
    Some(default_handler),  // DMA channel 1: 24
    Some(default_handler),  // DMA channel 2 and 3 and DMA2 channel 1 and 2: 25
    Some(default_handler),  // DMA channel 4,5,6,7 and DMA2 channel 3,4,5: 26
    Some(default_handler),  // ADC and COMP (ADC combined with EXTI lines 21 and 22): 27
    Some(default_handler),  // TIM1 break, update, trigger, communication: 28
    Some(default_handler),  // TIM1 capture compare: 29
    Some(default_handler),  // TIM2 global: 30
    Some(default_handler),  // TIM3 global: 31
    Some(default_handler),  // TIM6 global and DAC underrun: 32
    Some(default_handler),  // TIM7 global: 33
    Some(default_handler),  // TIM14 global: 34
    Some(default_handler),  // TIM15 global: 35
    Some(default_handler),  // TIM16 global: 36
    Some(default_handler),  // TIM17 global: 37
    Some(default_handler),  // I2C1 global (combined with EXTI Line 23): 38
    Some(default_handler),  // I2C2 global: 39
    Some(default_handler),  // SPI1 global: 40
    Some(default_handler),  // SPI2 global: 41
    Some(default_handler),  // USART1 global (combined with EXTI Line 25): 42
    Some(usart2_handler),  // USART2 global (combined with EXTI Line 26): 43
    Some(default_handler),  // USART3,4,5,6,7,8 (combined with EXTI Line 28): 44
    Some(default_handler),  // CEC and CAN global (combined with EXTI Line 27): 45
    Some(default_handler),  // USB (combined with EXTI Line 18): 46
];


unsafe extern "C" fn default_handler() {
    let instruction: usize;
    asm!("mrs r0, PSP
        ldr $0, [r0, #24]"
        : "=r"(instruction)
        : /* no inputs */
        : "r0"
        : "volatile"
    );
    kprintln!("Hard fault at instruction: {:x}", instruction);
    loop { bkpt() };
}

/// Supervisor Call
///
/// This call enters priviledged mode to provide system call services to application code. It uses
/// a special calling convention specific to the Altos-Rust kernel.
///
/// System calls in the Altos-Rust kernel have varying numbers of arguments, when a system call is
/// initiated, the system call number will be passed in as `r0`, all arguments to the system call
/// will be passed in with registers starting from `r1`. So the first argument would go in `r1`,
/// the second argument in `r2` and so on, even past the regular argument registers (`r0`-`r3`) for
/// the `aapcs` calling convention. This is to avoid having to get arguments off of the stack.
#[naked]
unsafe extern "C" fn sv_call_handler() {
    #[cfg(target_arch="arm")]
    #[cfg(feature="svc")]
    asm!(
        concat!(
            "push {r7, lr}\n", /* Save link register for return */
            //"bkpt\n", /* FIXME: FOR TESTING */
            "cmp r0, #11\n", /* Make sure we're within the jump table */
            "bhi svc_unknown\n",
            "adr r7, JUMP_TABLE\n",
            "lsls r0, r0, #2\n",
            "add r0, r7, r0\n",
            "ldr r0, [r0]\n",
            "mov pc, r0\n",

        ".align 4\n",
        "JUMP_TABLE:\n",
            ".word SVC_0\n",
            ".word SVC_1\n",
            ".word SVC_2\n",
            ".word SVC_3\n",
            ".word SVC_4\n",
            ".word SVC_5\n",
            ".word SVC_6\n",
            ".word SVC_7\n",
            ".word SVC_8\n",
            ".word SVC_9\n",

        "SVC_0:\n", /* exit (void) */
            "bl sys_exit\n",
            "b svc_end\n",

        "SVC_1:\n", /* sched_yield (void) */
            "bl sys_sched_yield\n",
            "b svc_end\n",

        "SVC_2:\n", /* sleep (wchan) */
            "mov r0, r1\n",
            "bl sys_sleep\n",
            "b svc_end\n",

        "SVC_3:\n", /* sleep_for (wchan, delay) */
            "mov r0, r1\n",
            "mov r1, r2\n",
            "bl sys_sleep_for\n",
            "b svc_end\n",

        "SVC_4:\n", /* wake (wchan) */
            "mov r0, r1\n",
            "bl sys_wake\n",
            "b svc_end\n",

        "SVC_5:\n", /* mutex_lock (lock) -> bool */
            "mov r0, r1\n",
            "bl sys_mutex_lock\n",
            "b svc_return\n",

        "SVC_6:\n", /* mutex_try_lock (lock) -> bool */
            "mov r0, r1\n",
            "bl sys_mutex_try_lock\n",
            "b svc_return\n",

        "SVC_7:\n", /* mutex_unlock (lock) */
            "mov r0, r1\n",
            "bl sys_mutex_unlock\n",
            "b svc_end\n",

        "SVC_8:\n", /* condvar_wait (condvar, lock) */
            "mov r0, r1\n",
            "mov r1, r2\n",
            "bl sys_condvar_wait\n",
            "b svc_end\n",

        "SVC_9:\n", /* condvar_broadcast (condvar, lock) */
            "mov r0, r1\n",
            "mov r1, r2\n",
            "bl sys_condvar_wait\n",
            "b svc_end\n",

        "svc_return:\n", /* we have a value to return, return value in r0 */
            "mrs r7, psp\n", /* assume we are called from user code */
            "str r0, [r7]\n", /* replace old r0 value with our new one */
        "svc_unknown:\n",
        "svc_end:\n",
            "pop {r7, pc}\n" /* restore r7 and return */
        )
        : /* no outputs */
        : /* no inputs */
        : /* no clobbers */
        : "volatile"
    );
    #[cfg(not(feature="svc"))]
    default_handler();
}

unsafe extern "C" fn systick_handler() {
    use time;

    syscall::system_tick();
    time::system_tick();
}

/// Tell OS to context switch tasks, this should be set to the lowest priority so that all
/// other interrupts are serviced first.
#[naked]
unsafe extern "C" fn pend_sv_handler() {
    #[cfg(target_arch="arm")]
    asm!(
        concat!(
            "cpsid i\n", /* disable interrupts for context switch */

            /* Normally, PendSV gets cleared when the interrupt starts, BUT, there is a very small
            * chance that if another interrupt arrives as the hardware is saving the context of the
            * scratch registers we could go to that interrupt instead (look up 'late arriving' with
            * regards to interrupt optimizations). If this happens and that interrupt happens to
            * set the PendSV interrupt pending then on exit we will come back to this handler with
            * the PendSV bit set. It will not get cleared automatically for us, so on exit of this
            * handler we will re-enter this handler. This will cause an extra context switch, which
            * may not be an issue, but if a critical task should run it will miss its timeslice.
            * It's best just to be safe and clear it manually, even at the extra overhead that it
            * brings to context switching */
            "movs r0, #1\n",
            "lsls r0, r0, #27\n", /* set the bit-mask */
            "ldr r1, ics_reg\n", /* get the address of the interrupt control status register */
            "str r0, [r1]\n", /* clear the PendSV bit */

            "mrs r0, psp\n", /* move program stack pointer into r0 */

            "ldr r3, current_task_const\n", /* get the location of the current task struct */
            "ldr r2, [r3]\n",

            "subs r0, r0, #32\n", /* make space for remaining low registers
                (r0-r3 saved automatically) */
            "str r0, [r2]\n",     /* save new top of stack */
            "stmia r0!, {r4-r7}\n", /* store the low registers */
            "mov r4, r8\n", /* store the high registers */
            "mov r5, r9\n",
            "mov r6, r10\n",
            "mov r7, r11\n",
            "stmia r0!, {r4-r7}\n",

            "push {r3, r14}\n", /* store pointer to current task and lr on main stack */
            "bl switch_context\n",
            "pop {r2, r3}\n", /* pointer to current task now in r2, lr goes in r3 */

            "ldr r1, [r2]\n",
            "ldr r0, [r1]\n", /* get the task's top of stack in r0 */
            "adds r0, r0, #16\n", /* move to the high registers first */
            "ldmia r0!, {r4-r7}\n", /* pop the high registers */
            "mov r8, r4\n",
            "mov r9, r5\n",
            "mov r10, r6\n",
            "mov r11, r7\n",

            "msr psp, r0\n", /* store the new top of stack into program stack pointer */

            "subs r0, r0, #32\n", /* go back for the low registers not automatically stored */
            "ldmia r0!, {r4-r7}\n",

            "cpsie i\n", /* re-enable interrupts */
            "bx r3\n", /* return from context switch */

            ".align 4\n",
            "current_task_const: .word CURRENT_TASK\n",
            "ics_reg: .word 0xe000ed04\n"
        )
        : /* no outputs */
        : /* no inputs */
        : /* no clobbers */
        : "volatile"
    );
}

// Interrupt handler for Usart2
unsafe extern "C" fn usart2_handler() {
    #[cfg(feature="serial")]
    {
        use peripheral::usart::{UsartX, Usart};
        use self::usart::{usart_tx, usart_rx};
        // Bits set in this register are stored in the usart2 in use, and
        // are reflected in the address for this usart2 variable as well.
        let usart2 = Usart::new(UsartX::Usart2);
        usart_rx(usart2);
        usart_tx(usart2);
    }
    #[cfg(not(feature="serial"))]
    default_handler();
}

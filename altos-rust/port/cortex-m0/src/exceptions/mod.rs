// exceptions/mod.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

use arm::asm::bkpt;
use altos_core::syscall;
use time;

#[link_section = ".exceptions"]
#[cfg(not(test))]
#[cfg(target_arch="arm")]
#[no_mangle]
pub static EXCEPTIONS: [Option<fn()>; 14] = [Some(default_handler),  // NMI
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
                                              Some(pend_sv_handler),  // PendSV
                                              Some(systick_handler)]; // SysTick
                                              


fn default_handler() {
    unsafe { bkpt(); }
}

fn systick_handler() {
  syscall::system_tick();
  time::system_tick();
}

/// Tell OS to context switch tasks, this should be set to the lowest priority so that all other
/// interrupts are serviced first
#[naked]
fn pend_sv_handler() {
  unsafe {
    #[cfg(target_arch="arm")]
    asm!(
      concat!(
        "cpsid i\n", /* disable interrupts for context switch */

        /* Normally, PendSV gets cleared when the interrupt starts, BUT, there is a very small
         * chance that if another interrupt arrives as the hardware is saving the context of the
         * scratch registers we could go to that interrupt instead (look up 'late arriving' with
         * regards to interrupt optimizations). If this happens and that interrupt happens to set
         * the PendSV interrupt pending then on exit we will come back to this handler with the
         * PendSV bit set. It will not get cleared automatically for us, so on exit of this handler
         * we will re-enter this handler. This will cause an extra context switch, which may not be
         * an issue, but if a critical task should run it will miss its timeslice. It's best just
         * to be safe and clear it manually, even at the extra overhead that it brings to context
         * switching */
        "movs r0, #1\n",
        "lsls r0, r0, #27\n", /* set the bit-mask */
        "ldr r1, ics_reg\n", /* get the address of the interrupt control status register */
        "str r0, [r1]\n", /* clear the PendSV bit */

        "mrs r0, psp\n", /* move program stack pointer into r0 */
       
        "ldr r3, current_task_const\n", /* get the location of the current task struct */
        "ldr r2, [r3]\n",

        "subs r0, r0, #32\n", /* make space for the remaining low registers (r0-r3 saved
                                automatically) */
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
        "ics_reg: .word 0xe000ed04\n")
    : /* no outputs */
    : /* no inputs */
    : /* no clobbers */
    : "volatile");
  }
}

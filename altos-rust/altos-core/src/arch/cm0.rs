// arch/cm0.rs
// AltOS Rust
//
// Created by Daniel Seitz on 1/7/17

use volatile::Volatile;
use task::args::Args;
use alloc::boxed::Box;

pub fn yield_cpu() {
  const ICSR_ADDR: usize = 0xE000_ED04;
  const PEND_SV_SET: usize = 0b1 << 28;

  unsafe {
    let mut reg = Volatile::new(ICSR_ADDR as *const usize);
    *reg |= PEND_SV_SET;
  }
}

pub fn initialize_stack(mut stack_ptr: Volatile<usize>, code: fn(&mut Args), args: &Box<Args>) -> usize {
  const INITIAL_XPSR: usize = 0x0100_0000;
  unsafe {
    // Offset added to account for way MCU uses stack on entry/exit of interrupts
    stack_ptr -= 4;
    stack_ptr.store(INITIAL_XPSR); /* xPSR */
    stack_ptr -= 4;
    stack_ptr.store(code as usize); /* PC */
    stack_ptr -= 4;
    stack_ptr.store(exit_error as usize); /* LR */
    stack_ptr -= 20; /* R12, R3, R2, R1 */
    stack_ptr.store(&**args as *const _ as usize); /* R0 */
    stack_ptr -= 32; /* R11..R4 */
    stack_ptr.as_ptr() as usize
  }
}

#[inline(never)]
pub fn start_first_task() {
  unsafe {
    #[cfg(target_arch="arm")]
    asm!(
      concat!(
          "ldr r2, current_task_const_2\n", /* get location of current_task */
          "ldr r3, [r2]\n",
          "ldr r0, [r3]\n",

          "adds r0, #32\n", /* discard everything up to r0 */
          "msr psp, r0\n", /* this is the new top of stack to use for the task */

          "movs r0, #2\n", /* switch to the psp stack */
          "msr CONTROL, r0\n", /* we're using psp instead of msp now */

          "isb\n", /* instruction barrier */

          "pop {r0-r5}\n", /* pop the registers that are saved automatically */
          "mov lr, r5\n", /* lr is now in r5, so put it back where it belongs */
          "pop {r3}\n", /* pop return address (old pc) into r3 */
          "pop {r2}\n", /* pop and discard xPSR */
          "cpsie i\n", /* first task has its context, so interrupts can be enabled */
          "bx r3\n", /* start executing user code */

           ".align 4\n",
          "current_task_const_2: .word CURRENT_TASK\n")
      : /* no outputs */
      : /* no inputs */
      : /* no clobbers */
      : "volatile");
  }
}

pub fn in_kernel_mode() -> bool {
  const MAIN_STACK: usize = 0b00;
  const _PROGRAM_STACK: usize = 0b10;
  unsafe {
    let stack_mask: usize;
    asm!("mrs $0, CONTROL\n" /* get the stack control mask */
      : "=r"(stack_mask)
      : /* no inputs */
      : /* no clobbers */
      : "volatile");
    stack_mask == MAIN_STACK
  }
}

pub fn begin_critical() -> usize {
  let primask: usize;
  unsafe {
    asm!(
      concat!(
        "mrs $0, PRIMASK\n",
        "cpsid i\n")
      : "=r"(primask)
      : /* no inputs */
      : /* no clobbers */
      : "volatile");
  }
  primask
}

pub fn end_critical(primask: usize) {
  unsafe {
    #[cfg(target_arch="arm")]
    asm!("msr PRIMASK, $0"
      : /* no outputs */
      : "r"(primask)
      : /* no clobbers */
      : "volatile");
  }
}

fn exit_error() {
  unsafe {
    #[cfg(target_arch="arm")]
    asm!("bkpt");
    loop{}
  }
}

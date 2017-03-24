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

//! This module is used to provide stubs for the architecture layer for testing.

use volatile::Volatile;
use task::args::Args;
use alloc::boxed::Box;
use sync::{RawMutex, CondVar};
use sched;
use syscall;

pub fn yield_cpu() {
    sched::switch_context();
}

pub fn initialize_stack(stack_ptr: Volatile<usize>, _code: fn(&mut Args), _args: &Box<Args>)
    -> usize {

    stack_ptr.as_ptr() as usize
}

pub fn start_first_task() {
    // no-op
}

pub fn in_kernel_mode() -> bool {
    // no-op
    true
}

pub fn begin_critical() -> usize {
    // no-op
    0
}

pub fn end_critical(_mask: usize) {
    // no-op
}

pub fn syscall0(call: u32) -> usize {
    match call {
        syscall::SYS_EXIT => syscall::sys_exit(),
        syscall::SYS_SCHED_YIELD => syscall::sys_sched_yield(),
        _ => panic!("Invalid syscall code for syscall0: {}", call),
    }
    return 0;
}

pub fn syscall1(call: u32, arg1: usize) -> usize {
    match call {
        syscall::SYS_SLEEP => syscall::sys_sleep(arg1),
        syscall::SYS_WAKE => syscall::sys_wake(arg1),
        syscall::SYS_MX_LOCK => {
            let lock = unsafe { &*(arg1 as *const RawMutex) };
            return syscall::sys_mutex_lock(lock) as usize;
        },
        syscall::SYS_MX_TRY_LOCK => {
            let lock = unsafe { &*(arg1 as *const RawMutex) };
            return syscall::sys_mutex_try_lock(lock) as usize;
        },
        syscall::SYS_MX_UNLOCK => {
            let lock = unsafe { &*(arg1 as *const RawMutex) };
            syscall::sys_mutex_unlock(lock);
        },
        syscall::SYS_CV_BROADCAST => {
            let condvar = unsafe { &*(arg1 as *const CondVar) };
            syscall::sys_condvar_broadcast(condvar);
        },
        _ => panic!("Invalid syscall code for syscall1: {}", call),
    }
    return 0;
}

pub fn syscall2(call: u32, arg1: usize, arg2: usize) -> usize {
    match call {
        syscall::SYS_SLEEP_FOR => syscall::sys_sleep_for(arg1, arg2),
        syscall::SYS_CV_WAIT => {
            let condvar = unsafe { &*(arg1 as *const CondVar) };
            let lock = unsafe { &*(arg2 as *const RawMutex) };
            syscall::sys_condvar_wait(condvar, lock);
        },
        _ => panic!("Invalid syscall code for syscall2: {}", call),
    }
    return 0;
}

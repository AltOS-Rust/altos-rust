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

//! Task creation
//!
//! This module contains the functions used to create tasks and modify them within the kernel.

pub mod args;
mod stack;
mod control;

pub use self::control::{TaskHandle, TaskControl, Delay, State, Priority};
pub use self::control::NUM_PRIORITIES;

use args::Args;

#[doc(hidden)]
pub fn init_idle_task() {
    use sched::PRIORITY_QUEUES;
    use queue::Node;
    use alloc::boxed::Box;
    const INIT_TASK_STACK_SIZE: usize = 256;

    let task = TaskControl::new(idle_task_code, Args::empty(), INIT_TASK_STACK_SIZE, Priority::__Idle, "idle");

    PRIORITY_QUEUES[task.priority()].enqueue(Box::new(Node::new(task)));
}

fn idle_task_code(_args: &mut Args) {
    use syscall::sched_yield;

    loop {
        sched_yield();
    }
}

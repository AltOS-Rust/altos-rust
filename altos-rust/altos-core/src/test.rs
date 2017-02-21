// test.rs
// AltOS Rust
//
// Created by Daniel Seitz on 1/14/17

//! This module provides a testing framework for the AltOS operating system to help test features
//! of the operating system.

macro_rules! assert_not {
    ($cond:expr) => { assert!(!$cond); };
    ($cond:expr, $($arg:tt)+) => { assert!(!$cond $(, $arg)+); }
}

use sched::{CURRENT_TASK, SLEEP_QUEUE, DELAY_QUEUE, OVERFLOW_DELAY_QUEUE, PRIORITY_QUEUES,
            NORMAL_TASK_COUNTER};
use sync::{SpinMutex, SpinGuard};
use task::{Priority, TaskControl, TaskHandle, State, Delay};
use task::args::Args;
use atomic::Ordering;

static TEST_LOCK: SpinMutex<()> = SpinMutex::new(());

pub fn set_up() -> SpinGuard<'static, ()> {
    let guard = TEST_LOCK.lock();
    SLEEP_QUEUE.remove_all();
    DELAY_QUEUE.remove_all();
    OVERFLOW_DELAY_QUEUE.remove_all();
    NORMAL_TASK_COUNTER.store(0, Ordering::Relaxed);
    for queue in PRIORITY_QUEUES.iter() {
        queue.remove_all();
    }
    unsafe { CURRENT_TASK = None };
    guard
}

pub fn create_test_task(stack_size: usize, priority: Priority, name: &'static str)
-> TaskControl {
    TaskControl::new(test_task, Args::empty(), stack_size, priority, name)
}

pub fn create_and_schedule_test_task(stack_size: usize, priority: Priority, name: &'static str)
-> TaskHandle {
    ::syscall::new_task(test_task, Args::empty(), stack_size, priority, name)
}

pub fn convert_handle_to_task_control(handle: TaskHandle) -> &'static TaskControl {
    unsafe { ::std::mem::transmute::<TaskHandle, &TaskControl>(handle) }
}

pub fn current_task() -> Option<&'static mut TaskControl> {
    unsafe { CURRENT_TASK.as_mut().map(|task| &mut ***task) }
}

pub fn create_two_tasks() -> (TaskHandle, TaskHandle) {
    let handle_1 = create_and_schedule_test_task(512, Priority::Normal, "test task 1");
    let handle_2 = create_and_schedule_test_task(512, Priority::Normal, "test task 2");
    (handle_1, handle_2)
}

//Set a task to be permanently blocked
pub fn block_current_task(delay_type: Delay) {
    current_task().unwrap().state = State::Blocked;
    current_task().unwrap().delay_type = delay_type;
}

fn test_task(_args: &mut Args) {}

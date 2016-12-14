// lib.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

#![feature(const_fn)]
#![no_std]
#![allow(dead_code)]

extern crate cortex_m0;

use cortex_m0::peripheral::gpio;
use cortex_m0::kernel::sync::{Mutex, MutexGuard};
use cortex_m0::kernel::sync::CondVar;

use cortex_m0::kernel::timer;
use cortex_m0::kernel::task::args::{Args, Builder};
use cortex_m0::kernel::task;
use cortex_m0::kernel::task::TaskHandle;
use cortex_m0::kernel::alloc::Box;
use cortex_m0::arm;

#[no_mangle]
// FIXME: Unmangle and make private again
pub static TEST_MUTEX: Mutex<u32> = Mutex::new(0);
pub static TEST_CONDVAR: CondVar = CondVar::new();

#[no_mangle]
pub fn application_entry() -> ! {
  let mut args = Builder::new(1);
  let guard = Box::new(TEST_MUTEX.lock());

  args = args.add_arg_box(guard);

  task::new_task(condvar_waiter, args.finalize(), 512, task::Priority::Critical, "condvar wait task");
  task::new_task(condvar_notifier, Args::empty(), 512, task::Priority::Critical, "condvar notify task");
  //task::new_task(test_task_1, 512, task::Priority::Critical, "first task");
  //task::new_task(test_task_2, 512, task::Priority::Critical, "second task");
  //task::new_task(test_task_3, 512, task::Priority::Critical, "third task");
  //task::new_task(mutex_task_1, args.finalize(), 1024, task::Priority::Critical, "first mutex task");
  //task::new_task(mutex_task_2, Args::empty(), 1024, task::Priority::Critical, "second mutex task");
  //task::new_task(delay_task, 512, task::Priority::Critical, "delay task");
  //task::new_task(frequency_task_1, 512, task::Priority::Critical, "frequency task 1");
  //task::new_task(frequency_task_2, 512, task::Priority::Critical, "frequency task 2");
  //task::new_task(preempt_task_1, 512, task::Priority::Critical, "preempt task 1");
  //task::new_task(preempt_task_2, 512, task::Priority::Critical, "preempt task 2");
  //task::new_task(arg_task, args2.finalize(), 512, task::Priority::Critical, "arg task");
  //let handle = task::new_task(to_destroy, Args::empty(), 512, task::Priority::Critical, "to destroy");
  //args = args.add_arg(&handle as *const _ as usize);
  //task::new_task(destroy_task, args.finalize(), 512, task::Priority::Critical, "destroy task");
  task::start_scheduler();

  loop { unsafe { arm::asm::bkpt() }; }
}

fn condvar_waiter(args: &mut Args) {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  let mut guard = unsafe { *args.pop_box::<MutexGuard<'static, u32>>() };
  loop {
    guard = TEST_CONDVAR.wait(guard);
    pb3.set(); 
    timer::Timer::delay_ms(*guard as usize);
    pb3.reset();
    timer::Timer::delay_ms(*guard as usize);
  }
}

fn condvar_notifier(_args: &mut Args) {
  loop {
    let mut guard = TEST_MUTEX.lock();
    if *guard >= 2000 {
      *guard = 100;
    }
    else {
      *guard += 100;
    }
    TEST_CONDVAR.notify_all();
    drop(guard);
    timer::Timer::delay_ms(4000);
  }
}

fn delay_task() {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  loop {
    pb3.set();
    timer::Timer::delay_ms(100);
    pb3.reset();
    timer::Timer::delay_ms(100);
  }
}

fn mutex_task_1(_args: &Args) {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  let mut value = 0;
  loop {
    value += 0x1;
    value = value & 0xFFFF;
    let mut guard = TEST_MUTEX.lock();
    if value == 0xFFFF {
      pb3.set();
      timer::Timer::delay_ms(2000);
      pb3.reset();
    }
    *guard = *guard & 0xFFFF0000;
    *guard = *guard | value;
    drop(guard);
  }
}

fn mutex_task_2(_args: &Args) {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  let mut value = 0;
  loop {
    value += 0x10000;
    value = value & 0xFFFF0000;
    let mut guard = TEST_MUTEX.lock();
    if value == 0xFFFF0000 {
      for _ in 0..10 {
        pb3.set();
        timer::Timer::delay_ms(100);
        pb3.reset();
        timer::Timer::delay_ms(100);
      }
    }
    *guard = *guard & 0xFFFF;
    *guard = *guard | value;
    drop(guard);
  }
}

fn test_task_1() {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  loop {
    for _ in 0..5 {
      pb3.set();
      timer::Timer::delay_ms(100);
      pb3.reset();
      timer::Timer::delay_ms(100);
    }
  }
}

fn test_task_2() {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  loop {
    for _ in 0..3 {
      pb3.set();
      timer::Timer::delay_ms(500);
      pb3.reset();
      timer::Timer::delay_ms(500);
    }
  }
}

fn test_task_3() {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  loop {
    for _ in 0..10 {
      pb3.set();
      timer::Timer::delay_ms(50);
      pb3.reset();
      timer::Timer::delay_ms(50);
    }
  }
}

fn frequency_task_1() {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  let delay = 500;
  loop {
    pb3.set();
    timer::Timer::delay_ms(delay);
    pb3.reset();
    timer::Timer::delay_ms(delay);
  }
}

fn frequency_task_2() {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  let mut delay = 250;
  loop {
    pb3.set();
    timer::Timer::delay_ms(delay);
    pb3.reset();
    timer::Timer::delay_ms(delay);
    delay += 10;
    if delay > 750 {
      delay = 250;
    }
  }
}

fn preempt_task_1() {
  let mut value: usize = 0;
  loop {
    value += 1;
    if value == value {} // Silence unused warning
  }
}

fn preempt_task_2() {
  let mut value: usize = !0;
  loop {
    value -= 1;
    if value == value {} // Silence unused warning
  }
}

fn arg_task(args: &mut Args) {
  let rate = args.pop_num();
  let multiplier = args.pop_num();
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  loop {
    pb3.set();
    timer::Timer::delay_ms(rate * multiplier);
    pb3.reset();
    timer::Timer::delay_ms(rate * multiplier);
  }
}

fn destroy_task(args: &mut Args) {
  let handle = unsafe { args.pop_box::<TaskHandle>() };
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  loop {
    pb3.set();
    timer::Timer::delay_ms(1000);
    pb3.reset();
    timer::Timer::delay_ms(1000);
    handle.destroy();
  }
}

fn to_destroy(_args: &Args) {
  let pb3 = gpio::Port::new(3, gpio::Group::B);
  loop {
    pb3.set();
    timer::Timer::delay_ms(100);
    pb3.reset();
    timer::Timer::delay_ms(100);
  }
}

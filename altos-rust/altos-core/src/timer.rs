// timer.rs
// AltOSRust
//
// Created by Daniel Seitz on 11/30/16

//! System time handling.
//!
//! This module helps keep track of the system time and how much time has passed.

// TODO: Come back to this module and rethink this design...

use volatile::Volatile;
use task;

static mut TIME: Timer = Timer::new();

/// A type containing information about the time passed since the start of the system.
#[derive(Copy, Clone)]
pub struct Timer {
  /// Number of seconds that have passed.
  pub sec: usize,
  
  /// Number of milliseconds that have passed.
  pub msec: usize,
}

impl Timer {
  /// Create a new timer initialized to 0 sec, 0 msec.
  const fn new() -> Self {
    Timer {
      sec: 0,
      msec: 0,
    }
  }

  /// Tick the system timer by 1 millisecond.
  ///
  /// This method should only be called by the system tick interrupt handler.
  #[doc(hidden)]
  pub fn tick() {
    unsafe {
      TIME.msec += 1;
      if TIME.msec % 1000 == 0 {
        TIME.sec += 1;
      }
    }
  }

  /// Gets the current system timer.
  pub fn get_current() -> Timer {
    unsafe { TIME }
  }

  /// Delays a task for a certain number of milliseconds.
  ///
  /// This method takes a `usize` argument for the number of milliseconds to delay the currently
  /// running task.
  pub fn delay_ms(ms: usize) {
    unsafe {
      let v_msec = Volatile::new(&TIME.msec);
      let start: usize = v_msec.load();
      let mut remaining = *v_msec - start;
      while remaining < ms {
        task::sleep_for(task::FOREVER_CHAN, ms - remaining);
        remaining = *v_msec - start;
      }
    }
  }

  /// Delays a task for a certain number of seconds.
  ///
  /// This method takes a `usize` argument for the number of seconds to delay the currently running
  /// task.
  pub fn delay_s(s: usize) {
    // TODO: Check for overflow and handle correctly
    Self::delay_ms(s * 1000);
  }
}

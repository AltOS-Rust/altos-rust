// time.rs
// AltOS Rust
//
// Created by Daniel Seitz on 1/7/17

use altos_core::sync::Mutex;
use altos_core::syscall;
use altos_core::atomic::{AtomicUsize, ATOMIC_USIZE_INIT, Ordering};
use core::ops::{Add, AddAssign, Sub};

static SYSTEM_TIME: Mutex<Time> = Mutex::new(Time::new());
static MS_RESOLUTION: AtomicUsize = ATOMIC_USIZE_INIT;

/// Get the current system time.
pub fn now() -> Time {
  let time = SYSTEM_TIME.lock();
  time.clone()
}

/// Delay a task for a certain number of milliseconds.
///
/// This method takes a `usize` argument for the number of milliseconds to delay the currently
/// running task.
#[inline(never)]
pub fn delay_ms(ms: usize) {
  let ms_res = get_resolution();
  if ms_res == 0 {
    panic!("delay_ms - the time ms_resolution has not been set!");
  }
  syscall::sleep_for(syscall::FOREVER_CHAN, ms * ms_res);
}

pub fn delay_s(s: usize) {
  // FIXME: Handle overflow
  delay_ms(s * 1000);
}

/// Set the ms resolution of the ticks.
///
/// This should only be called once upon initialization of the system. Setting this after the
/// system has been running for a while could cause some tasks that are delayed to wake up too
/// early.
///
/// # Example
///
/// ```rust,no_run
/// use altos_core::time::{self, Time};
///
/// // Assuming we tick 2 times every ms...
/// time::set_resolution(2);
/// // now every other tick we will increment the system timer by 1 ms
///
/// time::tick();
/// time::tick();
///
/// assert_eq!(Time::now().msec, 1);
/// ```
pub fn set_resolution(new: usize) {
  MS_RESOLUTION.store(new, Ordering::Relaxed);
}

pub fn get_resolution() -> usize {
  MS_RESOLUTION.load(Ordering::Relaxed)
}

// This should only get called by the system tick interrupt handler
#[doc(hidden)]
pub fn system_tick() {
  static mut TICKS: usize = 0;
  // We know this is safe because it should only be called by the system tick handler which can
  // only be running on one thread at a time.
  unsafe {
    TICKS += 1;
    if TICKS % get_resolution() == 0 {
      match SYSTEM_TIME.try_lock() {
        // If someone else is holding the lock, we'll just have to continue on, this could cause
        // some drift in our time measurement
        Some(mut time) => time.increment(),
        None => {}
      }
    }
  }
}

/// A type containing information about the time passed since the start of the system.
#[derive(Copy, Clone)]
pub struct Time {
  /// Number of seconds that have passed.
  pub sec: usize,
  
  /// Number of milliseconds that have passed.
  pub msec: usize,
}

impl Time {
  /// Create a new timer initialized to 0 sec, 0 msec.
  const fn new() -> Self {
    Time {
      sec: 0,
      msec: 0,
    }
  }

  /// Increment the system time by 1 ms, incrementing the seconds as well if our ms rolls over.
  fn increment(&mut self) {
    let increment = Time {
      sec: 0,
      msec: 1,
    };
    *self += increment;
  }
}

impl Add<Time> for Time {
  type Output = Time;

  fn add(mut self, rhs: Time) -> Self::Output {
    self.sec += rhs.sec;
    self.msec += rhs.msec;
    if self.msec >= 1000 {
      self.sec += 1;
      self.msec %= 1000;
    }
    self
  }
}

impl AddAssign<Time> for Time {
  fn add_assign(&mut self, rhs: Time) {
    *self = *self + rhs;
  }
}

impl Sub<Time> for Time {
  type Output = Time;

  fn sub(mut self, rhs: Time) -> Self::Output {
    // TODO: Figure out how to handle subtracting a bigger time from a smaller time... represent
    //  seconds as an isize instead? (Then we lose a lot of our number space, and we have to check
    //  for overflow)
    self.sec -= rhs.sec;
    if self.msec > rhs.msec {
      self.msec -= rhs.msec;
    }
    else {
      self.sec -= 1;
      self.msec = 1000 - (rhs.msec - self.msec)
    }
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_times() {
    let time1 = Time { sec: 100, msec: 10 };
    let time2 = Time { sec: 200, msec: 20 };

    let time3 = time1 + time2;

    assert_eq!(time3.sec, 300);
    assert_eq!(time3.msec, 30);
  }

  #[test]
  fn add_times_overflowing() {
    let time1 = Time { sec: 100, msec: 900 };
    let time2 = Time { sec: 100, msec: 200 };

    let time3 = time1 + time2;

    assert_eq!(time3.sec, 201);
    assert_eq!(time3.msec, 100);
  }
}

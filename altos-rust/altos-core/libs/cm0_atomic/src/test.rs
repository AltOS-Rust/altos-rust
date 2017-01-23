// test.rs
// AltOSRust
//
// Created by Daniel Seitz on 1/15/17

macro_rules! atomic {
  { $( $code:expr );*; } => {{
    $(
      $code;
    )*
  }};
  { $last:expr } => {{ $last }}
}

// arm.rs
// AltOSRust
//
// Created by Daniel Seitz on 1/15/17

macro_rules! start_critical {
  ($var:ident) => {{
    unsafe {
      #![cfg(target_arch="arm")]
      asm!(
        concat!(
          "mrs $0, PRIMASK\n",
          "cpsid i\n")
        : "=r"($var)
        : /* no inputs */
        : /* no clobbers */
        : "volatile");
    }
  }}
}

macro_rules! end_critical {
  ($var:ident) => {{
    unsafe {
      #![cfg(target_arch="arm")]
      asm!("msr PRIMASK, $0"
        : /* no outputs */
        : "r"($var)
        : /* no clobbers */
        : "volatile");
    }
  }}
}

macro_rules! atomic {
  { $( $code:expr );*; } => {{
    let primask: u32;
    start_critical!(primask);
    $(
      $code;
    )*
    end_critical!(primask);
  }};
  { $last:expr } => {{
    let primask: u32;
    start_critical!(primask);
    let result = $last;
    end_critical!(primask);
    result
  }}
}

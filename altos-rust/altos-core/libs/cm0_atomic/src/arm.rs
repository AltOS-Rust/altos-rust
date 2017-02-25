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

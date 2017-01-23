// init.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/9/16

// FIXME: Try to see if there's a better way to handle this for testing
// We do this cfg for testing purposes, this allows doctests to run without any compilation errors.
#![cfg(not(test))]

//! Contains functions used for initialization of the kernel

/// Initialize the heap so memory can be dynamically allocated
///
/// # Examples
///
/// ```rust,no_run
/// use altos_core::init::init_heap;
///
/// // An example for the Cortex-M0 processor
///
/// #[cfg(target_arch="arm")]
/// unsafe {
///   let heap_start: usize;
///   let heap_size: usize;
///   asm!(
///     concat!(
///       "ldr r0, =_heap_start\n",
///       "ldr r1, =_heap_end\n",
///
///       "subs r2, r1, r0\n")
///     : "={r0}"(heap_start), "={r2}"(heap_size)
///     : /* no inputs */
///     : "r0", "r1", "r2"
///     : "volatile"
///   );
///   init_heap(heap_start, heap_size);
///   }
/// ```
pub fn init_heap(heap_start: usize, heap_size: usize) {
  ::allocator::init_heap(heap_start, heap_size);
}

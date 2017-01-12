// task/args.rs
// AltOSRust
//
// Created by Daniel Seitz on 12/7/16

//! Arguments used in tasks.
//!
//! This module contains implementations for structs that help pass arguments into a task. The
//! `ArgsBuilder` struct provides an interface specifying what values the arguments to a task should
//! have. Begin by specifying how many arguments a task should take by creating a new `ArgsBuilder`
//! with that capacity, and use the `add_box()` and `add_num()` methods to give each 
//! argument a value. Once you have added all the arguments required, call the `finalize()` method 
//! to finish up the creation and return a usable `Args` object. For example:
//!
//! ```rust,no_run
//! use altos_core::args::{ArgsBuilder, Args};
//! use altos_core::Priority;
//! use altos_core::syscall::new_task;
//!
//! let mut args = ArgsBuilder::new(2);
//! args.add_num(100)
//!     .add_num(500);
//!
//! new_task(test_task, args.finalize(), 512, Priority::Normal, "args");
//!
//! fn test_task(args: &mut Args) {
//!   let first = args.pop_num(); // first = 100
//!   let second = args.pop_num(); // second = 400
//!   loop {}
//! }
//! ```

use collections::Vec;
use alloc::boxed::Box;

type RawPtr = usize;

/// An Args Builder.
///
/// Use this to construct a new list of arguments to pass into a task. The arguments should be
/// either a pointer to an object or a word length integer.
pub struct ArgsBuilder {
  cap: usize,
  len: usize,
  vec: Vec<RawPtr>,
}

impl ArgsBuilder {
  /// Returns an empty `Args` object.
  ///
  /// Use this if the task you are creating should not take any arguments.
  pub fn empty() -> Args {
    Args::empty()
  }

  /// Creates a new builder with the specified capacity.
  ///
  /// The number of arguments for a task should be known before hand in order to avoid unnecessary
  /// reallocations. Attempting to exceed this capacity will panic the kernel.
  pub fn new(cap: usize) -> Self {
    ArgsBuilder { 
      cap: cap,
      len: 0,
      vec: Vec::with_capacity(cap),
    }
  }

  /// Adds an object argument to the list of arguments.
  ///
  /// The argument is a box containing some object. When using the arguments within the task 
  /// you must know the type and order of each argument and cast them manually to the correct 
  /// object.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::args::ArgsBuilder;
  ///
  /// let mut args = ArgsBuilder::new(2);
  /// args.add_box(Box::new(400u16)).add_box(Box::new(100u32));
  /// ```
  ///
  /// # Panics
  ///
  /// This method will panic if you attempt to add more arguments than the capacity allocated.
  #[inline(never)]
  pub fn add_box<T>(&mut self, arg: Box<T>) -> &mut Self {
    if self.len >= self.cap {
      panic!("ArgsBuilder::add_arg - added too many arguments!");
    }
    unsafe { 
      let cell = self.vec.get_unchecked_mut(self.len);
      *cell = Box::into_raw(arg) as usize;
    }
    self.len += 1;
    self
  }

  /// Adds an integer value to the list of arguments.
  ///
  /// The argument should be a `usize` value. When using the arguments within the task you must
  /// know the type and order of each argument and cast them manually to the correct type.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::args::ArgsBuilder;
  ///
  /// let mut args = ArgsBuilder::new(2);
  /// args.add_num(500).add_num(100);
  /// ```
  ///
  /// # Panics
  ///
  /// This method will panic if you attempt to add more arguments than the capacity allocated.
  pub fn add_num(&mut self, arg: usize) -> &mut Self {
    if self.len >= self.cap {
      panic!("ArgsBuilder::add_copy - added too many arguments!");
    }
    unsafe {
      let cell = self.vec.get_unchecked_mut(self.len);
      *cell = arg;
    }
    self.len += 1;
    self
  }

  /// Returns a finalized `Args` object.
  ///
  /// After adding all the arguments that are required for the task, call this method to finalize
  /// the construction of the object. The finalized object will be used in the creation of a new
  /// task.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::args::ArgsBuilder;
  ///
  /// let mut args = ArgsBuilder::new(2);
  /// args.add_num(100).add_num(500);
  /// let finalized_args = args.finalize();
  /// ```
  pub fn finalize(mut self) -> Args {
    unsafe { self.vec.set_len(self.len) };
    Args::new(self.vec)  
  }
}

/// An object representing all of the arguments passed into a task.
/// 
/// This object contains a list of raw integer values that can be either interpreted as integer
/// values or raw pointer values that can be later casted into references. The task must know the
/// order and type of arguments passed into it in order to correctly interpret them. Unfortunately
/// we can not keep type safety across the task initialization barrier in order to keep tasks
/// uniform.
pub struct Args {
  args: Vec<RawPtr>,
}

impl Args {
  /// Returns an empty `Args` object.
  ///
  /// Use this when a task doesn't require any arguments.
  pub fn empty() -> Self {
    Args { args: Vec::with_capacity(0) }
  }

  /// Returns the next argument interpreted as a boxed object.
  ///
  /// This method unsafely casts the raw pointer value stored in the arguments list as a boxed
  /// value. If you cast it to the incorrect value you will be pointing at garbage or some unknown
  /// structure.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::args::ArgsBuilder;
  /// use altos_core::alloc::boxed::Box;
  ///
  /// struct Data(usize);
  ///
  /// let mut args = ArgsBuilder::new(1);
  ///
  /// args.add_box(Box::new(Data(100)));
  ///
  /// let mut my_args = args.finalize();
  ///
  /// let my_data: Box<Data> = unsafe { my_args.pop_box::<Data>() };
  /// ```
  ///
  /// # Panics
  ///
  /// This method will panic if there are no more arguments to retrieve.
  pub unsafe fn pop_box<T>(&mut self) -> Box<T> {
    let ptr = self.args.pop().unwrap();
    Box::from_raw(ptr as *mut T)
  }

  /// Returns the next argument interpreted as a number.
  ///
  /// This method is safe because even if the argument is not the type you intended, you cannot
  /// unsafely dereference any arbitrary memory addresses.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use altos_core::args::ArgsBuilder;
  ///
  /// let mut args = ArgsBuilder::new(1);
  ///
  /// args.add_num(100);
  ///
  /// let mut my_args = args.finalize();
  ///
  /// let my_data: usize = my_args.pop_num();
  /// ```
  ///
  /// # Panics
  ///
  /// This method will panic if there are no more arguments to retrieve.
  pub fn pop_num(&mut self) -> usize {
    self.args.pop().unwrap()
  }

  fn new(args: Vec<RawPtr>) -> Self {
    Args { args: args }
  }
}

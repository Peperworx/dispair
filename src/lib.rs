//! # [`Dispair`]
//! Dispair (Disp-Err) is a zero-dependency (other than `std`) library that provides a simple wrapper struct that implements `Error` for any type that implements both `Debug` and `Display`.
//! ```rust
//!use dispair::Dispair;
//!use std::error::Error;
//!
//!#[derive(Debug)]
//!struct TestStruct;
//!
//!impl core::fmt::Display for TestStruct {
//!    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!        core::fmt::Debug::fmt(&self, f)
//!    }
//!}
//!
//!fn main() {
//!    
//!    // Wrap `TestStruct` (which doesn't implement `Error`) in `Dispair` and return it.
//!    let err = Dispair(TestStruct);
//!    
//!    // Display just passes through to the wrapped value
//!    assert_eq!(format!("{}", err), "TestStruct");
//!    
//!    // Debug, however, does show `Dispair`.
//!    assert_eq!(format!("{:?}", err), "Dispair(TestStruct)");
//!
//!    // We should be able to convert it into a `Box<dyn Error>`
//!    let err_boxed: Box<dyn Error> = Box::new(err);
//!
//!    // And this should print nicely
//!    assert_eq!(err_boxed.to_string(), "TestStruct");
//!}
//!```
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use std::{fmt::{Display, Debug}, error::Error};

/// # [`Dispair`]
/// Wraps a given type that implements [`Display`] and [`Debug`] and implements [`Error`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dispair<E: Display + Debug>(pub E);

impl<E: Display + Debug> Display for Dispair<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<E: Display + Debug> Error for Dispair<E> {}


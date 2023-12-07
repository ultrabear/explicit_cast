//! A crate that provides a simple interface to primitive ints to allow explicit widening,
//! truncating, and sign casting.
//!
//! To get started, import the prelude:
//! ```
//! use explicit_cast::prelude::*;
//!
//! assert_eq!(5u8.widen::<u16>().sign_cast().widen::<i32>().truncate::<i8>(), 5i8);
//! ```
//! 
//! # Stability
//! This crate is 1.0 as in being **stable and or finished**, as there is no other functionality to be had than
//! allowing explicit casting of integers. As such, a prelude has been included that imports [`Widen`],
//! [`Truncate`], and [`SignCast`] for you. **No new methods** will be added to these traits, and **no
//! new traits** will be added to the prelude, without a 2.0 release, that theoretically should never
//! happen.
//!
//! Documentation updates may be published under a 1.0.X patch release, but no new functionality is
//! planned.

#![no_std]
#![forbid(unsafe_code)]
#![warn(clippy::alloc_instead_of_core, clippy::std_instead_of_alloc)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(clippy::module_name_repetitions)]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

mod codegen;

/// Sealed module
mod sealed {
    /// A bog standard Sealed trait
    pub trait Sealed {}

    /// implements Sealed for many items at once
    macro_rules! sealed {
        ($($t:ty),+) => {
            $(impl Sealed for $t {})*
        }
    }

    sealed!(u8, u16, u32, u64, u128);
    sealed!(i8, i16, i32, i64, i128);
}

use sealed::Sealed;

/// The inner trait of [`Widen`] that allows it to have a generic function signature.
///
/// This may be useful to import yourself if you wish to use it in API's, but it is only a
/// byproduct of this crate.
pub trait WidenFrom<T>: Sealed {
    /// Widens into [`Self`] from a smaller integer
    fn widen_from(v: T) -> Self;
}

/// The inner trait of [`Truncate`] that allows it to have a generic function signature.
///
/// This may be useful to import yourself if you wish to use it in API's, but it is only a
/// byproduct of this crate.
pub trait TruncateFrom<T>: Sealed {
    /// Truncates into [`Self`] from a larger integer
    fn truncate_from(v: T) -> Self;
}

/// Trait to sign cast an integer to/from signed/unsigned
///
/// This is better than `as` casting because:
/// - It is explicitly only casting signs, and will not change integer width
/// - It is method chainable
pub trait SignCast: Sealed {
    /// The target type after casting signs
    type SignCasted;

    /// Casts the an unsigned integer to a signed integer, or a signed integer to an unsigned
    /// integer.
    ///
    /// # Examples
    /// ```
    /// # use explicit_cast::SignCast;
    /// let casted: u8 = (-1i8).sign_cast();
    /// assert_eq!(casted, 0xff); // signed repr is Like That
    /// ```
    /// But this wont compile:
    /// ```compile_fail
    /// # use explicit_cast::SignCast;
    /// let casted: u8 = 0i16.sign_cast();
    /// ```
    fn sign_cast(self) -> Self::SignCasted;
}

/// Trait to truncate an integer from a larger size.
///
/// This is better than `as` casting because:
/// - It is explicitly a truncating operation, and will *only* truncate
/// - It only supports similar signs, i/e `u16` to `i8` will *not* compile
/// - It is method chainable
/// - You can use turbofishy or type inference :D
///
/// Error messages should also be clear in the event of an invalid operation, so you will not be
/// left wondering what went wrong, this is mostly thanks to rusts great error messages though
pub trait Truncate: Sealed + Sized {
    /// Truncates an integer to a smaller integer
    ///
    /// # Examples
    /// ```
    /// # use explicit_cast::Truncate;
    /// let u8_val = 0u16.truncate::<u8>();
    /// ```
    /// But this wont compile:
    /// ```compile_fail
    /// # use explicit_cast::Truncate;
    /// let val: u32 = 0u16.truncate();
    /// ```
    fn truncate<T: TruncateFrom<Self>>(self) -> T;
}

/// Trait to widen an integer from a smaller size, either zero extending or sign extending
/// depending on whether the integer is signed.
///
/// This is better than `as` casting because:
/// - It is explicitly a widening operation, and will *only* widen
/// - It only supports similar signs, i/e `u8` to `i16` will *not* compile
/// - It is method chainable
/// - You can use turbofishy :D or type inference, unlike [`into`](Into::into) which only supports type inference
///
/// Error messages should also be clear in the event of an invalid operation, so you will not be
/// left wondering what went wrong, this is mostly thanks to rusts great error messages though
pub trait Widen: Sealed + Sized {
    /// Widens an integer to a larger integer
    ///
    /// # Examples
    /// ```
    /// # use explicit_cast::Widen;
    /// let u16_val = 0u8.widen::<u16>();
    /// ```
    /// But this wont compile:
    /// ```compile_fail
    /// # use explicit_cast::Widen;
    /// let val: u16 = 0u32.widen();
    /// ```
    fn widen<T: WidenFrom<Self>>(self) -> T;
}

pub mod prelude {
    //! The prelude to this crate, includes [`SignCast`], [`Truncate`], and [`Widen`] imported for
    //! you
    pub use crate::{SignCast, Truncate, Widen};
}

#[test]
fn cast_works() {
    assert_eq!(
        0u8.sign_cast().widen::<i32>().sign_cast().truncate::<u16>(),
        0
    );
}

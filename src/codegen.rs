//! Submodule containing all the macros that generate `SignCast`, `Widen`, and `Truncate` implementations

use crate::{SignCast, Truncate, TruncateFrom, Widen, WidenFrom};

/// Implements `WidenFrom` for integer types, note that the argument order to this macro is critical
macro_rules! widen_from_order {
    ($t:ty, $($from:ty),+) => {
        $(
        impl WidenFrom<$from> for $t {
            #[inline]
            fn widen_from(v: $from) -> $t { <$t>::from(v) }
        }
        )*
        widen_from_order!($($from),+);
    };

    ($t:ty) => {};
}

widen_from_order!(u128, u64, u32, u16, u8);
widen_from_order!(i128, i64, i32, i16, i8);

/// Implements `TruncateFrom` for integer types, note that the argument order to this macro is critical
macro_rules! truncate_from_order {
    ($t:ty, $($from:ty),+) => {
        $(
        impl TruncateFrom<$from> for $t {
            #[inline]
            fn truncate_from(v: $from) -> $t { v as $t }
        }
        )*
        truncate_from_order!($($from),+);
    };

    ($t:ty) => {};
}

truncate_from_order!(u8, u16, u32, u64, u128);
truncate_from_order!(i8, i16, i32, i64, i128);

/// Implements `SignCast` for integer pairs, where each pair can cast into each other
macro_rules! sign_cast_pairs {
    ($(($t1:ty, $t2:ty)),+) => {
        $(
        impl SignCast for $t1 {
            type SignCasted = $t2;

            #[inline]
            fn sign_cast(self) -> Self::SignCasted { self as $t2 }
        }

        impl SignCast for $t2 {
            type SignCasted = $t1;

            #[inline]
            fn sign_cast(self) -> Self::SignCasted { self as $t1 }
        }
        )*
    };
}

sign_cast_pairs!((u8, i8), (u16, i16), (u32, i32), (u64, i64), (u128, i128));

/// Implements `Truncate` for each integer using the `TruncateFrom` bound
macro_rules! impl_truncate {

    ($($t:ty),+) => {
        $(
        impl Truncate for $t {
            #[inline]
            fn truncate<T: TruncateFrom<Self>>(self) -> T {
                T::truncate_from(self)
            }
        }
        )*
    }
}

impl_truncate!(u8, u16, u32, u64, u128);
impl_truncate!(i8, i16, i32, i64, i128);


/// Implements `Widen` for each integer using the `WidenFrom` bound
macro_rules! impl_widen {

    ($($t:ty),+) => {
        $(
        impl Widen for $t {
            #[inline]
            fn widen<T: WidenFrom<Self>>(self) -> T {
                T::widen_from(self)
            }
        }
        )*
    }
}

impl_widen!(u8, u16, u32, u64, u128);
impl_widen!(i8, i16, i32, i64, i128);

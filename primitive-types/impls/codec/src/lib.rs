// Copyright 2015-2018 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Parity Codec serialization support for uint and fixed hash.

#![cfg_attr(not(feature = "std"), no_std)]

pub use ustd::{mem, prelude::*, slice};

#[doc(hidden)]
pub use parity_codec as codec;

/// Add Parity Codec serialization support to an integer created by `construct_uint!`.
#[macro_export]
macro_rules! impl_uint_codec {
    ($name: ident, $len: expr) => {
        impl $crate::codec::Encode for $name {
            fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
                let mut bytes = [0u8; $len * 8];
                self.to_little_endian(&mut bytes);
                bytes.using_encoded(f)
            }
        }

        impl $crate::codec::Decode for $name {
            fn decode<I: $crate::codec::Input>(input: &mut I) -> Option<Self> {
                <[u8; $len * 8] as $crate::codec::Decode>::decode(input)
                    .map(|b| $name::from_little_endian(&b))
            }
        }
    };
}

/// Add Parity Codec serialization support to a fixed-sized hash type created by `construct_fixed_hash!`.
#[macro_export]
macro_rules! impl_fixed_hash_codec {
    ($name: ident, $len: expr) => {
        impl $crate::codec::Encode for $name {
            fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
                self.0.using_encoded(f)
            }
        }
        impl $crate::codec::Decode for $name {
            fn decode<I: $crate::codec::Input>(input: &mut I) -> Option<Self> {
                <[u8; $len] as $crate::codec::Decode>::decode(input).map($name)
            }
        }
    };
}

/// Add Parity Codec serialization extension for some array types.
#[macro_export]
macro_rules! impl_fixed_hash_codec_ext {
    ( $( $t:ty ),* ) => { $(
        impl $crate::codec::Encode for $t {
            fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
                let size = $crate::mem::size_of::<$t>();
                let value_slice = unsafe {
                    let ptr = self as *const _ as *const u8;
                    if size != 0 {
                        $crate::slice::from_raw_parts(ptr, size)
                    } else {
                        &[]
                    }
                };
                f(value_slice)
            }
        }

        impl $crate::codec::Decode for $t {
            fn decode<I: $crate::codec::Input>(input: &mut I) -> Option<Self> {
                let size = $crate::mem::size_of::<$t>();
                assert!(size > 0, "EndianSensitive can never be implemented for a zero-sized type.");
                let mut val: $t = unsafe { $crate::mem::zeroed() };

                unsafe {
                    let raw: &mut [u8] = $crate::slice::from_raw_parts_mut(
                        &mut val as *mut $t as *mut u8,
                        size
                    );
                    if input.read(raw) != size { return None }
                }
                Some(val)
            }
        }
    )* }
}

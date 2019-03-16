// Copyright 2015-2018 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Primitive types shared by Substrate and Parity Ethereum.
//!
//! Those are uint types `U128`, `U256` and `U512`, and fixed hash types `H160`,
//! `H256` and `H512`, with optional serde serialization, parity-codec and
//! rlp encoding.

#![cfg_attr(not(feature = "std"), no_std)]

mod error;

pub use self::error::{Error, TryFrom, TryInto, Never};

#[macro_use]
extern crate uint;

#[macro_use]
extern crate fixed_hash;

#[cfg(feature = "impl-serde")]
#[macro_use]
extern crate impl_serde;

#[cfg(feature = "impl-codec")]
#[macro_use]
extern crate impl_codec;

#[cfg(feature = "impl-rlp")]
#[macro_use]
extern crate impl_rlp;

construct_uint! {
	/// 64-bit unsigned integer.
	pub struct U64(1);
}
construct_uint! {
	/// 128-bit unsigned integer.
	pub struct U128(2);
}
construct_uint! {
	/// 256-bit unsigned integer.
	pub struct U256(4);
}
construct_uint! {
	/// 512-bits unsigned integer.
	pub struct U512(8);
}

construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 4 bytes (32 bits) size.
    pub struct H32(4);
}
construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 6 bytes (48 bits) size.
    pub struct H48(6);
}
construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 8 bytes (64 bits) size.
    pub struct H64(8);
}
construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 16 bytes (128 bits) size.
    pub struct H128(16);
}
construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.
	pub struct H160(20);
}
construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
	pub struct H256(32);
}
construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 33 bytes (264 bits) size.
    pub struct H264(33);
}
construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 64 bytes (512 bits) size.
	pub struct H512(64);
}
construct_fixed_hash! {
    /// Fixed-size uninterpreted hash type with 65 bytes (520 bits) size.
    pub struct H520(65);
}

#[cfg(feature = "impl-serde")]
mod serde {
	use super::*;

	impl_uint_serde!(U64, 1);
	impl_uint_serde!(U128, 2);
	impl_uint_serde!(U256, 4);
	impl_uint_serde!(U512, 8);

	impl_fixed_hash_serde!(H32, 4);
	impl_fixed_hash_serde!(H48, 6);
	impl_fixed_hash_serde!(H64, 8);
	impl_fixed_hash_serde!(H128, 16);
	impl_fixed_hash_serde!(H160, 20);
	impl_fixed_hash_serde!(H256, 32);
	impl_fixed_hash_serde!(H264, 33);
	impl_fixed_hash_serde!(H512, 64);
	impl_fixed_hash_serde!(H520, 65);
}

#[cfg(feature = "impl-codec")]
mod codec {
	use super::*;

	impl_uint_codec!(U64, 1);
	impl_uint_codec!(U128, 2);
	impl_uint_codec!(U256, 4);
	impl_uint_codec!(U512, 8);

	impl_fixed_hash_codec!(H32, 4);
	impl_fixed_hash_codec!(H48, 6);
	impl_fixed_hash_codec!(H64, 8);
	impl_fixed_hash_codec!(H128, 16);
	impl_fixed_hash_codec!(H160, 20);
	impl_fixed_hash_codec!(H256, 32);
	impl_fixed_hash_codec!(H264, 33);
	impl_fixed_hash_codec!(H512, 64);
	impl_fixed_hash_codec!(H520, 65);
}

#[cfg(feature = "impl-rlp")]
mod rlp {
	use super::*;

	impl_uint_rlp!(U64, 1);
	impl_uint_rlp!(U128, 2);
	impl_uint_rlp!(U256, 4);
	impl_uint_rlp!(U512, 8);

	impl_fixed_hash_rlp!(H32, 4);
	impl_fixed_hash_rlp!(H48, 6);
	impl_fixed_hash_rlp!(H64, 8);
	impl_fixed_hash_rlp!(H128, 16);
	impl_fixed_hash_rlp!(H160, 20);
	impl_fixed_hash_rlp!(H256, 32);
	impl_fixed_hash_rlp!(H264, 33);
	impl_fixed_hash_rlp!(H512, 64);
	impl_fixed_hash_rlp!(H520, 65);
}

impl_fixed_hash_conversions!(H256, H160);

pub trait BigEndianHash {
	type Uint;

	fn from_uint(val: &Self::Uint) -> Self;
	fn into_uint(&self) -> Self::Uint;
}

macro_rules! impl_uint_conversions {
	($hash: ident, $uint: ident) => {
		impl BigEndianHash for $hash {
			type Uint = $uint;

			fn from_uint(value: &$uint) -> Self {
				let mut ret = $hash::zero();
				value.to_big_endian(ret.as_bytes_mut());
				ret
			}

			fn into_uint(&self) -> $uint {
				$uint::from(self.as_ref() as &[u8])
			}
		}
	}
}

impl_uint_conversions!(H64, U64);
impl_uint_conversions!(H128, U128);
impl_uint_conversions!(H256, U256);
impl_uint_conversions!(H512, U512);

impl U256 {
	/// Multiplies two 256-bit integers to produce full 512-bit integer
	/// No overflow possible
	#[inline(always)]
	pub fn full_mul(self, other: U256) -> U512 {
		U512(uint_full_mul_reg!(U256, 4, self, other))
	}
}

impl From<U256> for U512 {
	fn from(value: U256) -> U512 {
		let U256(ref arr) = value;
		let mut ret = [0; 8];
		ret[0] = arr[0];
		ret[1] = arr[1];
		ret[2] = arr[2];
		ret[3] = arr[3];
		U512(ret)
	}
}

impl TryFrom<U256> for U128 {
	type Error = Error;

	fn try_from(value: U256) -> Result<U128, Error> {
		let U256(ref arr) = value;
		if arr[2] | arr[3] != 0 {
			return Err(Error::Overflow);
		}
		let mut ret = [0; 2];
		ret[0] = arr[0];
		ret[1] = arr[1];
		Ok(U128(ret))
	}
}

impl TryFrom<U512> for U256 {
	type Error = Error;

	fn try_from(value: U512) -> Result<U256, Error> {
		let U512(ref arr) = value;
		if arr[4] | arr[5] | arr[6] | arr[7] != 0 {
			return Err(Error::Overflow);
		}
		let mut ret = [0; 4];
		ret[0] = arr[0];
		ret[1] = arr[1];
		ret[2] = arr[2];
		ret[3] = arr[3];
		Ok(U256(ret))
	}
}

impl TryFrom<U512> for U128 {
	type Error = Error;

	fn try_from(value: U512) -> Result<U128, Error> {
		let U512(ref arr) = value;
		if arr[2] | arr[3] | arr[4] | arr[5] | arr[6] | arr[7] != 0 {
			return Err(Error::Overflow);
		}
		let mut ret = [0; 2];
		ret[0] = arr[0];
		ret[1] = arr[1];
		Ok(U128(ret))
	}
}

impl From<U128> for U512 {
	fn from(value: U128) -> U512 {
		let U128(ref arr) = value;
		let mut ret = [0; 8];
		ret[0] = arr[0];
		ret[1] = arr[1];
		U512(ret)
	}
}

impl From<U128> for U256 {
	fn from(value: U128) -> U256 {
		let U128(ref arr) = value;
		let mut ret = [0; 4];
		ret[0] = arr[0];
		ret[1] = arr[1];
		U256(ret)
	}
}

impl<'a> From<&'a U256> for U512 {
	fn from(value: &'a U256) -> U512 {
		let U256(ref arr) = *value;
		let mut ret = [0; 8];
		ret[0] = arr[0];
		ret[1] = arr[1];
		ret[2] = arr[2];
		ret[3] = arr[3];
		U512(ret)
	}
}

impl<'a> TryFrom<&'a U512> for U256 {
	type Error = Error;

	fn try_from(value: &'a U512) -> Result<U256, Error> {
		let U512(ref arr) = *value;
		if arr[4] | arr[5] | arr[6] | arr[7] != 0 {
			return Err(Error::Overflow);
		}
		let mut ret = [0; 4];
		ret[0] = arr[0];
		ret[1] = arr[1];
		ret[2] = arr[2];
		ret[3] = arr[3];
		Ok(U256(ret))
	}
}

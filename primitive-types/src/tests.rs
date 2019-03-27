#[cfg(feature = "serde")]
mod serde_tests {
    use crate::*;
    use serde_json as ser;

    macro_rules! test_serialize_hash {
        ($name: ident, $test_name: ident) => {
            #[test]
            fn $test_name() {
                let tests = vec![
                    ($name::from_low_u64_be(0), "0"),
                    ($name::from_low_u64_be(2), "2"),
                    ($name::from_low_u64_be(15), "f"),
                    ($name::from_low_u64_be(16), "10"),
                    ($name::from_low_u64_be(1_000), "3e8"),
                    ($name::from_low_u64_be(100_000), "186a0"),
                    ($name::from_low_u64_be(u64::max_value()), "ffffffffffffffff"),
                ];

                let align_0_len = $name::len_bytes() * 2;
                for (number, expected) in tests {
                    let expected = format!("0x{:0>width$}", expected, width = align_0_len);
                    assert_eq!(
                        format!("{:?}", expected),
                        ser::to_string_pretty(&number).unwrap(),
                    );
                    assert_eq!(number, ser::from_str(&format!("{:?}", expected)).unwrap());
                }

                // Invalid examples
                let invalid_example = format!("0x{:0>width$}", "g", width = align_0_len);
                assert!(ser::from_str::<$name>(&format!("{:?}", invalid_example))
                    .unwrap_err()
                    .is_data());
                assert!(ser::from_str::<$name>(r#""""#).unwrap_err().is_data());
                assert!(ser::from_str::<$name>(r#""0""#).unwrap_err().is_data());
                assert!(ser::from_str::<$name>(r#""10""#).unwrap_err().is_data());
            }
        };
    }

    test_serialize_hash!(H64, test_serialize_h64);
    test_serialize_hash!(H128, test_serialize_h128);
    test_serialize_hash!(H160, test_serialize_h160);
    test_serialize_hash!(H256, test_serialize_h256);
    test_serialize_hash!(H264, test_serialize_h264);
    test_serialize_hash!(H512, test_serialize_h512);
    test_serialize_hash!(H520, test_serialize_h520);
    test_serialize_hash!(H1024, test_serialize_h1024);
    test_serialize_hash!(H2048, test_serialize_h2048);

    #[test]
    fn test_serialize_hash_large_values() {
        assert_eq!(
            ser::to_string_pretty(&H2048::from([255u8; 256])).unwrap(),
            "\"0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"",
        );
        let ser_from_str = |s: &str| ser::from_str::<H2048>(s).unwrap_err().is_data();
        assert!(ser_from_str(
            "\"0x1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
             ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\"",
        ));
    }

    macro_rules! test_serialize_uint {
        ($name: ident, $test_name: ident) => {
            #[test]
            fn $test_name() {
                let tests = vec![
                    ($name::from(0), "0x0"),
                    ($name::from(1), "0x1"),
                    ($name::from(2), "0x2"),
                    ($name::from(10), "0xa"),
                    ($name::from(15), "0xf"),
                    ($name::from(15), "0xf"),
                    ($name::from(16), "0x10"),
                    ($name::from(1_000), "0x3e8"),
                    ($name::from(100_000), "0x186a0"),
                    ($name::from(u64::max_value()), "0xffffffffffffffff"),
                ];

                for (number, expected) in tests {
                    assert_eq!(
                        format!("{:?}", expected),
                        ser::to_string_pretty(&number).unwrap(),
                    );
                    assert_eq!(number, ser::from_str(&format!("{:?}", expected)).unwrap());
                }

                // Invalid examples
                assert!(ser::from_str::<$name>(r#""0x""#).unwrap_err().is_data());
                assert!(ser::from_str::<$name>(r#""0xg""#).unwrap_err().is_data());
                assert!(ser::from_str::<$name>(r#""""#).unwrap_err().is_data());
                assert!(ser::from_str::<$name>(r#""0""#).unwrap_err().is_data());
                assert!(ser::from_str::<$name>(r#""10""#).unwrap_err().is_data());
            }
        };
    }

    test_serialize_uint!(U64, test_serialize_u64);
    test_serialize_uint!(U128, test_serialize_u128);
    test_serialize_uint!(U256, test_serialize_u256);
    test_serialize_uint!(U512, test_serialize_u512);

    #[test]
    fn test_serialize_uint_large_values() {
        assert_eq!(
            ser::to_string_pretty(&!U256::zero()).unwrap(),
            r#""0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff""#
        );
        let ser_from_str = |s: &str| ser::from_str::<U256>(s).unwrap_err().is_data();
        assert!(ser_from_str(
            r#""0x1ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff""#
        ));
    }
}

#[cfg(feature = "codec")]
mod codec_tests {
//
//    use impl_codec::codec;
//
//    #[test]
//    fn test_codec_uint() {
//
//    }
//
//    #[test]
//    fn test_codec_hash() {}
}

#[cfg(feature = "rlp")]
mod rlp_tests {
    use ustd::{cmp, fmt, prelude::*};

    use impl_rlp::rlp;

    use super::helper;
    use crate::{H160, U256};

    struct ETestPair<T>(T, Vec<u8>);

    impl<T: rlp::Encodable> ETestPair<T> {
        pub fn run_encode_test(self) {
            let res = rlp::encode(&self.0);
            assert_eq!(&res[..], &self.1[..]);
        }
    }

    struct DTestPair<T>(T, Vec<u8>);

    impl<T: rlp::Decodable + fmt::Debug + cmp::Eq> DTestPair<T> {
        pub fn run_decode_test(self) {
            let res: Result<T, rlp::DecoderError> = rlp::decode(&self.1);
            assert!(res.is_ok());
            assert_eq!(&res.unwrap(), &self.0);
        }
    }

    #[test]
    fn test_rlp_codec_u256() {
        let tests = vec![
            (U256::from(0u64), vec![0x80u8]),
            (
                U256::from(0x0100_0000u64),
                vec![0x84, 0x01, 0x00, 0x00, 0x00],
            ),
            (
                U256::from(0xffff_ffffu64),
                vec![0x84, 0xff, 0xff, 0xff, 0xff],
            ),
            (
                helper::u256_from_hex_str(
                    "8090a0b0c0d0e0f00910203040506077000000000000000100000000000012f0",
                ),
                vec![
                    0xa0, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0, 0x09, 0x10, 0x20, 0x30,
                    0x40, 0x50, 0x60, 0x77, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0xf0,
                ],
            ),
        ];
        for (u256, vec_u8) in tests {
            ETestPair(u256, vec_u8.clone()).run_encode_test();
            DTestPair(u256, vec_u8).run_decode_test();
        }
    }

    #[test]
    fn test_rlp_codec_h160() {
        let tests = vec![(
            H160::from_slice(&helper::vec_u8_from_hex_str(
                "ef2d6d194084c2de36e0dabfce45d046b37d1106",
            )),
            vec![
                0x94, 0xef, 0x2d, 0x6d, 0x19, 0x40, 0x84, 0xc2, 0xde, 0x36, 0xe0, 0xda, 0xbf, 0xce,
                0x45, 0xd0, 0x46, 0xb3, 0x7d, 0x11, 0x06,
            ],
        )];
        for (h160, vec_u8) in tests {
            ETestPair(h160, vec_u8.clone()).run_encode_test();
            DTestPair(h160, vec_u8).run_decode_test();
        }
    }
}

#[test]
fn test_fixed_arrays_roundtrip() {
    let raw = helper::u256_from_hex_str("7094875209347850239487502394881");
    let array: [u8; 32] = raw.into();
    let new_raw = array.into();
    assert_eq!(raw, new_raw);
}

#[test]
fn test_u256_multi_full_mul() {
    use ustd::u64::MAX;

    use crate::{U256, U512};

    assert_eq!(
        U512([0, 0, 0, 0, 0, 0, 0, 0]),
        U256([0, 0, 0, 0]).full_mul(U256([0, 0, 0, 0]))
    );
    assert_eq!(
        U512([1, 0, 0, 0, 0, 0, 0, 0]),
        U256([1, 0, 0, 0]).full_mul(U256([1, 0, 0, 0]))
    );
    assert_eq!(
        U512([25, 0, 0, 0, 0, 0, 0, 0]),
        U256([5, 0, 0, 0]).full_mul(U256([5, 0, 0, 0]))
    );
    assert_eq!(
        U512([0, 0, 25, 0, 0, 0, 0, 0]),
        U256([0, 5, 0, 0]).full_mul(U256([0, 5, 0, 0]))
    );
    assert_eq!(
        U512([0, 0, 0, 16, 0, 0, 0, 0]),
        U256([0, 0, 0, 4]).full_mul(U256([4, 0, 0, 0]))
    );
    assert_eq!(
        U512([0, 0, 0, 10, 0, 0, 0, 0]),
        U256([0, 0, 0, 5]).full_mul(U256([2, 0, 0, 0]))
    );
    assert_eq!(
        U512([0, 0, 0, 10, 0, 0, 0, 0]),
        U256([0, 0, 2, 0]).full_mul(U256([0, 5, 0, 0]))
    );
    assert_eq!(
        U512([0, 0, 0, 9, 0, 0, 0, 0]),
        U256([0, 3, 0, 0]).full_mul(U256([0, 0, 3, 0]))
    );
    assert_eq!(
        U512([0, 0, 0, 0, 48, 0, 0, 0]),
        U256([0, 0, 8, 0]).full_mul(U256([0, 0, 6, 0]))
    );
    assert_eq!(
        U512([0, 27, 0, 0, 0, 0, 0, 0]),
        U256([9, 0, 0, 0]).full_mul(U256([0, 3, 0, 0]))
    );
    assert_eq!(
        U512([1, MAX - 1, 0, 0, 0, 0, 0, 0]),
        U256([MAX, 0, 0, 0]).full_mul(U256([MAX, 0, 0, 0]))
    );
    assert_eq!(
        U512([0, 1, MAX - 1, 0, 0, 0, 0, 0]),
        U256([0, MAX, 0, 0]).full_mul(U256([MAX, 0, 0, 0]))
    );
    assert_eq!(
        U512([1, MAX, MAX - 1, 0, 0, 0, 0, 0]),
        U256([MAX, MAX, 0, 0]).full_mul(U256([MAX, 0, 0, 0]))
    );
    assert_eq!(
        U512([1, MAX, MAX - 1, 0, 0, 0, 0, 0]),
        U256([MAX, 0, 0, 0]).full_mul(U256([MAX, MAX, 0, 0]))
    );
    assert_eq!(
        U512([1, 0, MAX - 1, MAX, 0, 0, 0, 0]),
        U256([MAX, MAX, 0, 0]).full_mul(U256([MAX, MAX, 0, 0]))
    );
    assert_eq!(
        U512([1, MAX, MAX, MAX - 1, 0, 0, 0, 0]),
        U256([MAX, 0, 0, 0]).full_mul(U256([MAX, MAX, MAX, 0]))
    );
    assert_eq!(
        U512([1, MAX, MAX, MAX - 1, 0, 0, 0, 0]),
        U256([MAX, MAX, MAX, 0]).full_mul(U256([MAX, 0, 0, 0]))
    );
    assert_eq!(
        U512([1, MAX, MAX, MAX, MAX - 1, 0, 0, 0]),
        U256([MAX, 0, 0, 0]).full_mul(U256([MAX, MAX, MAX, MAX]))
    );
    assert_eq!(
        U512([1, MAX, MAX, MAX, MAX - 1, 0, 0, 0]),
        U256([MAX, MAX, MAX, MAX]).full_mul(U256([MAX, 0, 0, 0]))
    );
    assert_eq!(
        U512([1, 0, MAX, MAX - 1, MAX, 0, 0, 0]),
        U256([MAX, MAX, MAX, 0]).full_mul(U256([MAX, MAX, 0, 0]))
    );
    assert_eq!(
        U512([1, 0, MAX, MAX - 1, MAX, 0, 0, 0]),
        U256([MAX, MAX, 0, 0]).full_mul(U256([MAX, MAX, MAX, 0]))
    );
    assert_eq!(
        U512([1, 0, MAX, MAX, MAX - 1, MAX, 0, 0]),
        U256([MAX, MAX, MAX, MAX]).full_mul(U256([MAX, MAX, 0, 0]))
    );
    assert_eq!(
        U512([1, 0, MAX, MAX, MAX - 1, MAX, 0, 0]),
        U256([MAX, MAX, 0, 0]).full_mul(U256([MAX, MAX, MAX, MAX]))
    );
    assert_eq!(
        U512([1, 0, 0, MAX - 1, MAX, MAX, 0, 0]),
        U256([MAX, MAX, MAX, 0]).full_mul(U256([MAX, MAX, MAX, 0]))
    );
    assert_eq!(
        U512([1, 0, 0, MAX, MAX - 1, MAX, MAX, 0]),
        U256([MAX, MAX, MAX, 0]).full_mul(U256([MAX, MAX, MAX, MAX]))
    );
    assert_eq!(
        U512([1, 0, 0, MAX, MAX - 1, MAX, MAX, 0]),
        U256([MAX, MAX, MAX, MAX]).full_mul(U256([MAX, MAX, MAX, 0]))
    );
    assert_eq!(
        U512([1, 0, 0, 0, MAX - 1, MAX, MAX, MAX]),
        U256([MAX, MAX, MAX, MAX]).full_mul(U256([MAX, MAX, MAX, MAX]))
    );
    assert_eq!(
        U512([0, 0, 0, 0, 0, 0, 1, MAX - 1]),
        U256([0, 0, 0, MAX]).full_mul(U256([0, 0, 0, MAX]))
    );
    assert_eq!(
        U512([0, 0, 0, MAX, 0, 0, 0, 0]),
        U256([1, 0, 0, 0]).full_mul(U256([0, 0, 0, MAX]))
    );
    assert_eq!(
        U512([5, 10, 15, 20, 0, 0, 0, 0]),
        U256([1, 2, 3, 4]).full_mul(U256([5, 0, 0, 0]))
    );
    assert_eq!(
        U512([0, 6, 12, 18, 24, 0, 0, 0]),
        U256([1, 2, 3, 4]).full_mul(U256([0, 6, 0, 0]))
    );
    assert_eq!(
        U512([0, 0, 7, 14, 21, 28, 0, 0]),
        U256([1, 2, 3, 4]).full_mul(U256([0, 0, 7, 0]))
    );
    assert_eq!(
        U512([0, 0, 0, 8, 16, 24, 32, 0]),
        U256([1, 2, 3, 4]).full_mul(U256([0, 0, 0, 8]))
    );
    assert_eq!(
        U512([5, 16, 34, 60, 61, 52, 32, 0]),
        U256([1, 2, 3, 4]).full_mul(U256([5, 6, 7, 8]))
    );
}

mod helper {
    use ustd::prelude::*;

    use rustc_hex::FromHex;

    use crate::U256;

    pub fn u256_from_hex_str(value: &str) -> U256 {
        let bytes: Vec<u8> = if value.len() % 2 == 0 {
            value.from_hex().unwrap()
        } else {
            format!("0{}", value).from_hex().unwrap()
        };
        let bytes: &[u8] = &bytes;
        U256::from(bytes)
    }

    pub fn vec_u8_from_hex_str(value: &str) -> Vec<u8> {
        FromHex::from_hex(value).unwrap()
    }
}

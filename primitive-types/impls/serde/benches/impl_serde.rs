// Copyright 2019 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! benchmarking for impl_serde
//! should be started with:
//! ```bash
//! cargo bench
//! ```

#![allow(
    clippy::assign_op_pattern,
    clippy::ptr_offset_with_cast,
    clippy::range_plus_one,
    clippy::transmute_ptr_to_ptr
)]

#[macro_use]
extern crate uint;

use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use impl_serde::impl_uint_serde;

construct_uint! {
    pub struct U256(4);
}

impl_uint_serde!(U256, 4);

criterion_group!(impl_serde, u256_to_hex);
criterion_main!(impl_serde);

fn u256_to_hex(c: &mut Criterion) {
    c.bench(
        "u256_to_hex",
        ParameterizedBenchmark::new(
            "",
            |b, x| b.iter(|| black_box(serde_json::to_string(&x))),
            vec![
                U256::from(0),
                U256::from(100),
                U256::from(u32::max_value()),
                U256::from(u64::max_value()),
                U256::from(u128::max_value()),
                U256([1, 2, 3, 4]),
            ],
        ),
    );
}

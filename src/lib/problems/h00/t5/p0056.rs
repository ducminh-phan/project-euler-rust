//! A googol (`10^100`) is a massive number: one followed by one-hundred zeros;
//! `100^100` is almost unimaginably large: one followed by two-hundred zeros.
//! Despite their size, the sum of the digits in each number is only `1`.
//!
//! Considering natural numbers of the form, `a^b`, where `a, b < 100`, what is
//! the maximum digital sum?

use itertools::iproduct;
use num::BigUint;

use crate::numbers::sum_of_digits;

pub fn solve() {
    let limit = 100u32;
    let result = iproduct!(1..limit, 1..limit)
        .map(|(a, b)| BigUint::from(a).pow(b))
        .map(sum_of_digits)
        .max()
        .unwrap();
    println!("{result}");
}

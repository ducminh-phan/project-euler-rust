//! `2**15 = 32768` and the sum of its digits is `3 + 2 + 7 + 6 + 8 = 26.`
//!
//! What is the sum of the digits of the number `2**1000`?

use num::BigUint;

use crate::numbers::sum_of_digits;

pub fn solve() -> crate::Answer {
    let n = num::pow(BigUint::from(2u8), 1000);
    sum_of_digits(n).into()
}

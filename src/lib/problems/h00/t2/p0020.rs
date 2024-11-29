//! `n!` means `n * (n-1) * ... * 3 * 2 * 1`.
//!
//! For example, `10! = 3628800`, and the sum of the digits in the number `10!`
//! is `3 + 6 + 2 + 8 + 8 + 0 + 0 = 27`.
//!
//! Find the sum of the digits in the number `100!`.

use num::BigUint;

use crate::numbers::sum_of_digits;

pub fn solve() -> crate::Answer {
    let mut p = BigUint::from(1u8);
    for n in 2..=100u8 {
        p *= BigUint::from(n);
    }

    sum_of_digits(p).into()
}

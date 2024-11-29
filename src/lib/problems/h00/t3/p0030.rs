//! Surprisingly there are only three numbers that can be written as the sum of
//! fourth powers of their digits:
//!
//! ```text
//! 1634 = 1^4 + 6^4 + 3^4 + 4^4
//! 8208 = 8^4 + 2^4 + 0^4 + 8^4
//! 9474 = 9^4 + 4^4 + 7^4 + 4^4
//! ```
//!
//! As `1 = 1^4` is not a sum it is not included.
//!
//! The sum of these numbers is 1634 + 8208 + 9474 = 19316
//!
//! Find the sum of all the numbers that can be written as the sum of fifth
//! powers of their digits.

use crate::numbers::digits;

pub fn solve() -> crate::Answer {
    // Assume that a number "n" has "k" digits and is the sum of fifth powers of
    // its digits => 10^(k-1) < n < k * 9^5

    let k = (2..)
        .find(|k| 10u32.pow(k - 1) > (*k) * 9u32.pow(5))
        .unwrap()
        - 1;
    dbg!(k);

    let nums = (2..10u32.pow(k))
        .filter(|n| *n == sum_digit_powers(*n, 5))
        .collect::<Vec<_>>();
    dbg!(&nums);

    nums.iter().sum::<u32>().into()
}

fn sum_digit_powers(n: u32, p: u32) -> u32 {
    digits(n).iter().map(|d| (*d as u32).pow(p)).sum::<u32>()
}

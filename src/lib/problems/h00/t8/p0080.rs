//! It is well known that if the square root of a natural number is not an
//! integer, then it is irrational. The decimal expansion of such square roots
//! is infinite without any repeating pattern at all.
//!
//! The square root of two is `1.41421356237309504880...`, and the digital sum
//! of the first one hundred decimal digits is `475`.
//!
//! For the first one hundred natural numbers, find the total of the digital
//! sums of the first one hundred decimal digits for all the irrational square roots.

use num::integer::Roots;
use num::BigUint;

use crate::numbers::is_square;

pub fn solve() -> crate::Answer {
    (2..100)
        .filter(|n| !is_square(*n))
        // We need to include the integer part
        .map(|n| sqrt_digits_sum(n, 99) + n.sqrt())
        .sum::<u64>()
        .into()
}

fn sqrt_digits_sum(n: u64, precision: u64) -> u64 {
    let mut n = BigUint::from(n);

    let mut total = 0;
    let mut sqrt = n.sqrt();

    for _ in 0..=precision {
        // We could compute digit as n.sqrt() - sqrt, however,
        // 10 multiplications is somewhat faster than 1 sqrt
        let mut digit = 0u64;
        while (sqrt.clone() + digit + 1u64).pow(2) <= n {
            digit += 1;
        }

        total += digit;
        sqrt += digit;

        sqrt *= 10u64;
        n *= 100u64;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt_digits_sum() {
        assert_eq!(sqrt_digits_sum(2, 100), 481);
    }
}

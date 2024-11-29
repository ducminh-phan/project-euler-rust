//! Euler discovered the remarkable quadratic formula: `n^2 + n + 41`.
//!
//! It turns out that the formula will produce `40` primes for the consecutive
//! integer values `0 <= n <= 39`. However, when `n = 40`,
//! `40^2 + 40 + 41 = 40(40 + 1) + 41` is divisible by `41`, and certainly when
//! `n = 41`, `41^2 + 41 + 41` is clearly divisible by `41`.
//!
//! The incredible formula `n^2 - 79n + 1601` was discovered, which produces
//! `80` primes for the consecutive values `0 <= n <= 79`. The product of the
//! coefficients, `-79` and `1601`, is `-126479`.
//!
//! Considering quadratics of the form `n^2 + an + b`, where `|a| < 1000` and
//! `|b| <= 1000`, where `|n|` is the modulus/absolute value of `n`.
//!
//! Find the product of the coefficients, `a` and `b`, for the quadratic
//! expression that produces the maximum number of primes for consecutive values
//! of `n`, starting with `n = 0`.</p>

use itertools::iproduct;

use crate::primes::is_prime;

const MAX_A: i64 = 999;
const MAX_B: i64 = 1000;

pub fn solve() -> crate::Answer {
    fn eval(n: u64, a: i64, b: i64) -> i64 {
        (n * n) as i64 + a * (n as i64) + b
    }

    /// f(a, b) is the first non-negative integer n such that n^2 + an + b
    /// is not a prime.
    fn f(a: i64, b: i64) -> u64 {
        let max_n = b.unsigned_abs();
        (0..=max_n)
            .find(|n| {
                let c = eval(*n, a, b);
                c <= 0 || !is_prime(c as u64)
            })
            .unwrap_or(0)
    }

    let (a, b) = iproduct!(-MAX_A..=MAX_A, -MAX_B..MAX_B)
        .max_by_key(|(a, b)| f(*a, *b))
        .unwrap();
    dbg!(a, b);

    (a * b).into()
}

//! The sum of the primes below 10 is `2 + 3 + 5 + 7 = 17`.
//!
//! Find the sum of all the primes below two million.

use crate::primes::{PrimeSet, Primes};

pub fn solve() -> crate::Answer {
    Primes::new()
        .iter()
        .take_while(|p| *p < 2_000_000)
        .sum::<u64>()
        .into()
}

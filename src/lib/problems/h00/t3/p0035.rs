//! The number, 197, is called a circular prime because all rotations of the
//! digits: 197, 971, and 719, are themselves prime.
//!
//! There are thirteen such primes below 100:
//!
//! ```text
//! 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//! ```
//!
//! How many circular primes are there below one million?

use std::collections::HashSet;
use std::sync::LazyLock;

use crate::numbers::n_digits;
use crate::primes::{PrimeSet, Primes};

// Store the list of primes upto 1 million
static PRIMES: LazyLock<HashSet<u64>> = LazyLock::new(|| {
    Primes::new()
        .iter()
        .take_while(|p| *p < 1_000_000)
        .collect::<HashSet<_>>()
});

pub fn solve() {
    let result = PRIMES
        .iter()
        .filter(|p| generate_rotations(**p).iter().all(|r| PRIMES.contains(r)))
        .count();

    println!("{result}");
}

fn generate_rotations(p: u64) -> Vec<u64> {
    let n_digits = n_digits(p);

    (0..n_digits)
        .map(|q| {
            let a = 10u64.pow(q);
            let b = 10u64.pow(n_digits - q);
            (p % a) * b + p / a
        })
        .collect()
}

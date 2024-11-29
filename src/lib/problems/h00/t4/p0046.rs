//! It was proposed by Christian Goldbach that every odd composite number can be
//! written as the sum of a prime and twice a square.
//!
//! ```text
//!  9 =  7 + 2 * 1^2
//! 15 =  7 + 2 * 2^2
//! 21 =  3 + 2 * 3^2
//! 25 =  7 + 2 * 3^2
//! 27 = 19 + 2 * 2^2
//! 33 = 31 + 2 * 1^2
//! ```
//!
//! It turns out that the conjecture was false.
//!
//! What is the smallest odd composite that cannot be written as the sum of a
//! prime and twice a square?

use crate::numbers::is_square;
use crate::primes::{PrimeSet, Primes};

pub fn solve() -> crate::Answer {
    let mut primes = Primes::new();
    while primes.list().is_empty() || primes.list().last() < Some(&10) {
        primes.expand()
    }

    loop {
        let ps = primes.list();
        let last = *ps.last().unwrap();
        let min_n = ps.get(ps.len() - 2).unwrap() + 2;

        for n in (min_n..last).step_by(2) {
            if ps[1..]
                .iter()
                .filter(|p| **p < n)
                .all(|p| !is_square((n - p) / 2))
            {
                return n.into();
            }
        }

        primes.expand();
    }
}

//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
//! we can see that the 6th prime is 13.
//!
//! What is the 10001st prime number?

use crate::primes::{PrimeSet, Primes};

pub fn solve() -> crate::Answer {
    Primes::new().iter().take(10001).last().unwrap().into()
}

//! It is possible to write ten as the sum of primes in exactly five different
//! ways:
//!
//! ```text
//! 7 + 3
//! 5 + 5
//! 5 + 3 + 2
//! 3 + 3 + 2 + 2
//! 2 + 2 + 2 + 2 + 2
//! ```
//!
//! What is the first value which can be written as the sum of primes in over
//! five thousand different ways?

use crate::misc::coins_sum;
use crate::primes::{PrimeSet, Primes};

pub fn solve() {
    let primes = Primes::new()
        .iter()
        .take_while(|p| *p < 100)
        .map(|p| p as usize)
        .collect::<Vec<_>>();

    let result = (1usize..).find(|n| coins_sum(&primes, *n) > 5000).unwrap();
    println!("{result}");
}

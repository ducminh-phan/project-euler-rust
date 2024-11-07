//! The first two consecutive numbers to have two distinct prime factors are:
//!
//! ```text
//! 14 = 2 * 7
//! 15 = 3 * 5
//! ```
//!
//! The first three consecutive numbers to have three distinct prime factors
//! are:
//!
//! ```text
//! 644 = 2^2 *  7 * 23
//! 645 = 3   *  5 * 43
//! 646 = 2   * 17 * 19
//! ```
//!
//! Find the first four consecutive integers to have four distinct prime factors
//! each. What is the first of these numbers?

use itertools::Itertools;

use crate::numbers::factor;
use crate::primes::Primes;

pub fn solve() {
    let mut primes = Primes::new();
    let mut prime_factor_counts: [usize; 4] = [0; 4];

    for n in 2u64.. {
        prime_factor_counts[(n % 4) as usize] = factor(n, &mut primes).len();
        if prime_factor_counts.iter().all_equal_value() == Ok(&4) {
            println!("{}", n - 3);
            return;
        }
    }
}

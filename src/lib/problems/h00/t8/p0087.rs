//! The smallest number expressible as the sum of a prime square, prime cube,
//! and prime fourth power is `28`. In fact, there are exactly four numbers
//! below fifty that can be expressed in such a way:
//!
//! ```text
//! 28 = 2^2 + 2^3 + 2^4
//! 33 = 3^2 + 2^3 + 2^4
//! 49 = 5^2 + 2^3 + 2^4
//! 47 = 2^2 + 3^3 + 2^4
//! ```
//!
//! How many numbers below fifty million can be expressed as the sum of a prime
//! square, prime cube, and prime fourth power?

use itertools::{iproduct, Itertools};
use num::integer::Roots;

use crate::primes::{PrimeSet, Primes};

pub fn solve() {
    let max = 5e7 as u64;
    let max_a = max.sqrt();
    let max_b = max.cbrt();
    let max_c = max.nth_root(4);

    let primes_a = Primes::new()
        .iter()
        .take_while(|p| *p <= max_a)
        .collect_vec();

    let primes_b = primes_a.iter().take_while(|p| **p <= max_b);
    let primes_c = primes_a.iter().take_while(|p| **p <= max_c);

    let result = iproduct!(primes_a.iter(), primes_b, primes_c)
        .map(|(a, b, c)| a.pow(2) + b.pow(3) + c.pow(4))
        .filter(|p| *p < max)
        .unique()
        .count();

    println!("{result}");
}

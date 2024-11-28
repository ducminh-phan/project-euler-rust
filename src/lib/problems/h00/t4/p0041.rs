//! We shall say that an n-digit number is pandigital if it makes use of all the
//! digits 1 to `n` exactly once. For example, 2143 is a 4-digit pandigital and
//! is also prime.
//!
//! What is the largest n-digit pandigital prime that exists?

use itertools::Itertools;

use crate::numbers::num_from_digits;
use crate::primes::is_prime;

pub fn solve() {
    // There is no 5-, 6-, 8- or 9-digit pandigital primes, as they are always
    // divisible by 3. We iterate over permutations of 1..7, then 1..4 in
    // reverse order to find the first prime.

    let result = [7u8, 4]
        .into_iter()
        .flat_map(|n| (1..=n).rev().permutations(n as usize))
        .map(num_from_digits)
        .find(|&d| is_prime(d))
        .unwrap();

    println!("{result}");
}

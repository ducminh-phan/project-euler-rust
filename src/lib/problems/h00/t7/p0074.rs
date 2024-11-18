//! The number `145` is well known for the property that the sum of the
//! factorial of its digits is equal to `145`:
//!
//! ```text
//! 1! + 4! + 5! = 1 + 24 + 120 = 145.
//! ```
//!
//! Perhaps less well known is `169`, in that it produces the longest chain of
//! numbers that link back to `169`; it turns out that there are only three such
//! loops that exist:
//!
//! ```text
//! 169 -> 363601 -> 1454 -> 169
//! 871 -> 45361 -> 871
//! 872 -> 45362 -> 872
//! ```
//!
//! It is not difficult to prove that EVERY starting number will eventually get
//! stuck in a loop. For example,
//!
//! ```text
//! 69 -> 363600 -> 1454 -> 169 -> 363601 (-> 1454)
//! 78 -> 45360 -> 871 -> 45361 (-> 871)
//! 540 -> 145 (-> 145)
//! ```
//!
//! Starting with `69` produces a chain of five non-repeating terms, but the
//! longest non-repeating chain with a starting number below one million is
//! sixty terms.
//!
//! How many chains, with a starting number below one million, contain exactly
//! sixty non-repeating terms?

use std::collections::HashSet;

use crate::numbers::digits;

const DIGIT_FACTORIALS: [u64; 10] =
    [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

pub fn solve() {
    let ceiling = 1e6 as u64;
    let result = (1..ceiling).filter(|n| find_chain_length(*n) == 60).count();
    println!("{result}");
}

fn sum_of_digits_factorials(n: u64) -> u64 {
    digits(n)
        .iter()
        .map(|d| DIGIT_FACTORIALS[*d as usize])
        .sum()
}

fn find_chain_length(mut n: u64) -> usize {
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&n) {
            break;
        }

        visited.insert(n);
        n = sum_of_digits_factorials(n);
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_digits_factorials() {
        assert_eq!(sum_of_digits_factorials(145), 145);
        assert_eq!(sum_of_digits_factorials(69), 363600);
    }

    #[test]
    fn test_find_chain_length() {
        assert_eq!(find_chain_length(69), 5);
        assert_eq!(find_chain_length(540), 2);
    }
}

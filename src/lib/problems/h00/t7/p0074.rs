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

use std::sync::{LazyLock, Mutex};

use crate::numbers::digits;

const DIGIT_FACTORIALS: [u64; 10] =
    [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

const CACHE_SIZE: usize = 1e6 as usize;
static CHAIN_LENGTH_CACHE: LazyLock<Mutex<Vec<usize>>> =
    LazyLock::new(|| Mutex::new(vec![0; CACHE_SIZE]));

pub fn solve() -> crate::Answer {
    let ceiling = 1e6 as u64;
    (1..ceiling)
        .filter(|n| find_chain_length(*n) == 60)
        .count()
        .into()
}

fn sum_of_digits_factorials(n: u64) -> u64 {
    digits(n)
        .iter()
        .map(|d| DIGIT_FACTORIALS[*d as usize])
        .sum()
}

fn find_chain_length(mut n: u64) -> usize {
    {
        let cache = CHAIN_LENGTH_CACHE.lock().unwrap();
        if let Some(&cached) = cache.get(n as usize) {
            if cached > 0 {
                return cached;
            }
        }
    }

    let mut chain = vec![n];
    loop {
        n = sum_of_digits_factorials(n);

        let mut cache = CHAIN_LENGTH_CACHE.lock().unwrap();
        if let Some(&cached) = cache.get(n as usize) {
            if cached > 0 {
                // Now the chain is n0 -> n1 -> ... -> n(k+1) (cached)
                // Add the cached size to the chain
                let k = chain.len();
                chain
                    .iter()
                    .enumerate()
                    .filter(|(_, &n)| n < CACHE_SIZE as u64)
                    .for_each(|(i, n)| cache[*n as usize] = cached + k - i);

                return cached + k;
            }
        }

        if let Some(x) = chain.iter().position(|&x| x == n) {
            // Now we have the chain n0 -> n1 -> ... -> n(k+1) = n(x) for x <= k
            // => The chain length for n0 is k, for n1 is k-1,...
            //    And the chain length for n(x)...n(k) is k - x
            let k = chain.len();
            chain
                .iter()
                .enumerate()
                .filter(|(_, &n)| n < CACHE_SIZE as u64)
                .for_each(|(i, n)| {
                    cache[*n as usize] = k - i.min(x);
                });
            return k;
        }

        chain.push(n);
    }
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
        assert_eq!(find_chain_length(363600), 4);
        assert_eq!(find_chain_length(1454), 3);
        assert_eq!(find_chain_length(169), 3);
        assert_eq!(find_chain_length(363601), 3);

        assert_eq!(find_chain_length(540), 2);
        assert_eq!(find_chain_length(145), 1);

        // sum_of_digits_factorials(999) will go over CACHE_SIZE
        assert_eq!(find_chain_length(999), 47);
    }
}

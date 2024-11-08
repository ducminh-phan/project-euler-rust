//! The number 3797 has an interesting property. Being prime itself, it is
//! possible to continuously remove digits from left to right, and remain prime
//! at each stage: 3797, 797, 97, and 7. Similarly we can work from right to
//! left: 3797, 379, 37, and 3.
//!
//! Find the sum of the only eleven primes that are both truncatable from left
//! to right and right to left.
//!
//! NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

use std::collections::VecDeque;

use crate::primes::is_prime;

pub fn solve() {
    // first digit must be 2, 3, 5, 7
    // last digit must be 3, 7
    // middle digits must be 1, 3, 7, 9

    let mut truncatable_primes: Vec<u64> = vec![];

    // We only push right-truncatable primes into the queue
    let mut deque: VecDeque<u64> = VecDeque::from([2, 3, 5, 7]);
    while let Some(p) = deque.pop_front() {
        // We could prune by checking that p % 10 must be 3 or 7
        if is_left_truncatable_prime(p) {
            truncatable_primes.push(p);
        }

        for next_digit in [1, 3, 7, 9] {
            let next_p = 10 * p + next_digit;
            if is_prime(next_p) {
                deque.push_back(next_p);
            }
        }
    }

    dbg!(&truncatable_primes);

    let r = truncatable_primes.iter().sum::<u64>();
    println!("{}", r);
}

fn is_left_truncatable_prime(p: u64) -> bool {
    if p < 10 {
        return false;
    }

    let n_digits = (p as f64).log10().floor() as u32 + 1;
    (1..n_digits).all(|n| is_prime(p % 10u64.pow(n)))
}
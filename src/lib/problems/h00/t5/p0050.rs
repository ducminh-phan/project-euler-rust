//! The prime `41`, can be written as the sum of six consecutive primes:
//!
//! ```text
//! 41 = 2 + 3 + 5 + 7 + 11 + 13.
//! ```
//!
//! This is the longest sum of consecutive primes that adds to a prime below
//! one-hundred.
//!
//! The longest sum of consecutive primes below one-thousand that adds to a
//! prime, contains `21` terms, and is equal to `953`.
//!
//! Which prime, below one-million, can be written as the sum of the most
//! consecutive primes?

use crate::primes::{is_prime, PrimeSet, Primes};

pub fn solve() {
    let result = solve_with_n(1_000_000);
    println!("{result}");
}

fn solve_with_n(limit: u64) -> u64 {
    let mut primes = Primes::new();
    let prefix_sums = [0u64]
        .into_iter()
        .chain(primes.iter())
        .take_while(|&p| p < limit)
        .scan(0, |sum, i| {
            *sum += i;
            Some(*sum)
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut j = 1;
    let mut r = (0, 0);
    // Two pointers: for each pair (i, j) such that p[j] - p[i] < limit,
    // consider i1 = i...(j - r), we don't need to consider i1 >= j - r,
    // as j - i1 <= r does not yield a better result
    'outer: while j < prefix_sums.len() {
        for i1 in i..(j - r.0) {
            let diff = prefix_sums[j] - prefix_sums[i1];

            if is_prime(diff) {
                r = r.max((j - i1, diff));

                // Now we find a prime consecutive sum, continuing would only
                // yield a shorter subsequence => skip to next outer iteration
                continue 'outer;
            }
        }

        if prefix_sums[j] - prefix_sums[i] <= limit {
            j += 1;
            if j >= prefix_sums.len() {
                break 'outer;
            }
        }

        while prefix_sums[j] - prefix_sums[i] > limit {
            i += 1;
        }
    }

    dbg!(r);
    r.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_with_n() {
        assert_eq!(solve_with_n(100), 41);
        assert_eq!(solve_with_n(1000), 953);
    }
}

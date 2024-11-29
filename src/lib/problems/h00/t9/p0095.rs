//! The proper divisors of a number are all the divisors excluding the number
//! itself. For example, the proper divisors of `28` are `1`, `2`, `4`, `7`,
//! and `14`. As the sum of these divisors is equal to `28`, we call it a
//! perfect number.
//!
//! Interestingly the sum of the proper divisors of `220` is `284` and the sum
//! of the proper divisors of `284` is `220`, forming a chain of two numbers.
//! For this reason, `220` and `284` are called an amicable pair.
//!
//! Perhaps less well known are longer chains. For example, starting with
//! `12496`, we form a chain of five numbers:
//!
//! ```text
//! 12496 -> 14288 -> 15472 -> 14536 -> 14264 (-> 12496 -> ...)
//! ```
//!
//! Since this chain returns to its starting point, it is called an amicable
//! chain.
//!
//! Find the smallest member of the longest amicable chain with no element
//! exceeding one million.

use itertools::Itertools;

pub fn solve() -> crate::Answer {
    let max_n = 1e6 as usize;
    let mut divisors = vec![vec![1]; max_n + 1];

    for divisor in 2..=max_n {
        (2..)
            .map(|factor| factor * divisor)
            .take_while(|n| *n <= max_n)
            .for_each(|n| divisors[n].push(divisor))
    }

    let sum_of_divisors = divisors
        .iter()
        .map(|ds| ds.iter().sum::<usize>())
        .collect_vec();
    let mut visited = vec![false; max_n + 1];
    let mut result = 0usize;
    let mut max_length = 0;

    'outer: for n in 2..=max_n {
        if visited[n] {
            continue;
        }

        let mut chain = vec![n];
        visited[n] = true;

        let mut curr = n;
        loop {
            curr = sum_of_divisors[curr];

            if curr > max_n {
                continue 'outer;
            }

            if visited[curr] {
                break;
            }

            chain.push(curr);
            visited[curr] = true;
        }

        // There are cases like
        // 9464 -> 12496 -> 14288 -> 15472 -> 14536 -> 14264 (-> 12496)
        // We need to find where the loop starts
        if let Some((loop_start_idx, _)) =
            chain.iter().find_position(|x| **x == curr)
        {
            let loop_size = chain.len() - loop_start_idx;

            if loop_size > max_length {
                result = *chain[loop_start_idx..].iter().min().unwrap();
                max_length = loop_size
            }
        }
    }

    result.into()
}

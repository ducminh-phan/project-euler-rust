//! The following iterative sequence is defined for the set of positive integers:
//!
//! * `n -> n/2` (n is even)
//! * `n -> 3n + 1` (n is odd)
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//! ```text
//! 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1
//! ```
//!
//! It can be seen that this sequence (starting at 13 and finishing at 1) contains
//! 10 terms. Although it has not been proved yet (Collatz Problem), it is thought
//! that all starting numbers finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! NOTE: Once the chain starts the terms are allowed to go above one million.

fn find_collatz_sequence_length(n: u64) -> u64 {
    let mut n = n;
    let mut count = 1;

    loop {
        match n % 2 {
            0 => n /= 2,
            _ => n = 3 * n + 1,
        };
        count += 1;

        if n == 1 {
            break;
        }
    }

    count
}

pub fn solve() {
    let result = (1..(1e6 as u64))
        .map(|n| (n, find_collatz_sequence_length(n)))
        .max_by_key(|(_, len)| *len)
        .unwrap();

    println!("{:?}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_collatz_sequence_length() {
        assert_eq!(find_collatz_sequence_length(4), 3);
        assert_eq!(find_collatz_sequence_length(13), 10);
    }
}

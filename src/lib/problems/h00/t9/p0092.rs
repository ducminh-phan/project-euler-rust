//! A number chain is created by continuously adding the square of the digits
//! in a number to form a new number until it has been seen before.
//!
//! For example,
//!
//! ```text
//! 44 -> 32 -> 13 -> 10 -> 1 -> 1
//! 85 -> 89 -> 145 -> 42 -> 20 -> 4 -> 16 -> 37 -> 58 -> 89
//! ```
//!
//! Therefore any chain that arrives at `1` or `89` will become stuck in an
//! endless loop. What is most amazing is that EVERY starting number will
//! eventually arrive at `1` or `89`.
//!
//! How many starting numbers below ten million will arrive at `89`?

use crate::numbers::digits;

pub fn solve() {
    let max_n = 1e7 as usize;
    let mut cache = vec![0; max_n];
    cache[1] = 1;
    cache[89] = 89;

    let mut count = 0;

    for mut n in 2..max_n {
        let mut chain = vec![n];
        while n != 1 && n != 89 {
            n = sum_of_digit_squares(n);
            if cache[n] != 0 {
                n = cache[n];
                break;
            }

            chain.push(n);
        }

        for c in chain {
            cache[c] = n;
        }

        if n == 89 {
            count += 1;
        }
    }

    println!("{count}");
}

fn sum_of_digit_squares(n: usize) -> usize {
    digits(n).iter().map(|d| (*d as usize).pow(2)).sum()
}

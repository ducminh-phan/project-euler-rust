//! A perfect number is a number for which the sum of its proper divisors is
//! exactly equal to the number. For example, the sum of the proper divisors of
//! 28 would be `1 + 2 + 4 + 7 + 14 = 28`, which means that 28 is a perfect
//! number.
//!
//! A number `n` is called deficient if the sum of its proper divisors is less
//! than `n` and it is called abundant if this sum exceeds `n`.
//!
//! As 12 is the smallest abundant number, `1 + 2 + 3 + 4 + 6 = 16`, the
//! smallest number that can be written as the sum of two abundant numbers
//! is 24. By mathematical analysis, it can be shown that all integers greater
//! than 28123 can be written as the sum of two abundant numbers. However, this
//! upper limit cannot be reduced any further by analysis even though it is
//! known that the greatest number that cannot be expressed as the sum of two
//! abundant numbers is less than this limit.
//!
//! Find the sum of all the positive integers which cannot be written as the sum
//! of two abundant numbers.

use itertools::{iproduct, Itertools};

use crate::numbers::divisors;

const MAX: u64 = 28124;

pub fn solve() {
    // Find all abundant numbers between 12 and 28124.
    // Then find all numbers that can be written as the sum of these abundant
    // numbers. From that we can find the answer.

    let abundant_numbers = (12..=MAX)
        .filter(|n| divisors(*n).iter().sum::<u64>() > *n)
        .collect::<Vec<_>>();

    let bad_numbers =
        iproduct!(abundant_numbers.clone(), abundant_numbers.clone())
            .map(|(a, b)| a + b)
            .filter(|n| *n <= MAX)
            .unique()
            .collect::<Vec<_>>();

    let r = (1..=MAX).sum::<u64>() - bad_numbers.into_iter().sum::<u64>();

    println!("{}", r);
}

//! A palindromic number reads the same both ways.
//! The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 * 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

use itertools::Itertools;

use crate::numbers::is_palindrome;

pub fn solve() {
    let n = 3;
    let result = (10u32.pow(n - 1)..(10u32.pow(n) - 1))
        .combinations_with_replacement(2)
        .map(|items| items.iter().product::<u32>())
        .sorted()
        .rev()
        .find_or_first(is_palindrome)
        .unwrap();

    println!("{result}");
}

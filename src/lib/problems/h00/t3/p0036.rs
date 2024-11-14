//! The decimal number, `585 = 1001001001_2` (binary), is palindromic in both
//! bases.
//!
//! Find the sum of all numbers, less than one million, which are palindromic
//! in base 10 and base 2.
//!
//! (Please note that the palindromic number, in either base, may not include
//! leading zeros.)

use crate::numbers::digits;

pub fn solve() {
    let result = (1..1_000_000)
        .filter(|n| is_palindromic_base_2(*n) && is_palindromic_base_10(*n))
        .sum::<u32>();
    println!("{result}");
}

fn is_palindromic_base_10(n: u32) -> bool {
    let d = digits(n);
    d == d.clone().into_iter().rev().collect::<Vec<_>>()
}

fn is_palindromic_base_2(n: u32) -> bool {
    let d = num::BigUint::from(n).to_radix_le(2);
    d == d.clone().into_iter().rev().collect::<Vec<_>>()
}

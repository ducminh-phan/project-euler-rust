//! We shall say that an n-digit number is pandigital if it makes use of all
//! the digits 1 to n exactly once; for example, the 5-digit number, 15234, is
//! 1 through 5 pandigital.
//!
//! The product 7254 is unusual, as the identity, `39 * 186 = 7254`, containing
//! multiplicand, multiplier, and product is 1 through 9 pandigital.
//!
//! Find the sum of all products whose multiplicand/multiplier/product identity
//! can be written as a 1 through 9 pandigital.
//!
//! HINT: Some products can be obtained in more than one way so be sure to only
//! include it once in your sum.

use std::collections::HashSet;

use itertools::{iproduct, Itertools};

use crate::numbers::digits;

pub fn solve() {
    // There are two cases:
    // - 1-digit * 4-digit = 4-digit
    // - 2-digit * 3-digit = 4-digit

    let mut products = HashSet::new();

    products.extend(
        iproduct!(1..10, 1_000..10_000)
            .filter(|(a, b)| is_product_pandigital(*a, *b))
            .map(|(a, b)| a * b),
    );

    products.extend(
        iproduct!(10..100, 100..1_000)
            .filter(|(a, b)| is_product_pandigital(*a, *b))
            .map(|(a, b)| a * b),
    );

    let result: u32 = products.iter().sum();
    println!("{result}");
}

fn is_product_pandigital(a: u32, b: u32) -> bool {
    let mut all_digits = vec![];

    all_digits.extend(digits(a));
    all_digits.extend(digits(b));
    all_digits.extend(digits(a * b));

    all_digits.len() == 9
        && all_digits.iter().unique().count() == 9
        && !all_digits.contains(&0)
}

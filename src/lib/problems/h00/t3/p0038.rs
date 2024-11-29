//! Take the number 192 and multiply it by each of 1, 2, and 3:
//!
//! ```text
//! 192 * 1 = 192
//! 192 * 2 = 384
//! 192 * 3 = 576
//! ```
//!
//! By concatenating each product we get the 1 to 9 pandigital, `192384576`. We
//! will call `192384576` the concatenated product of 192 and `(1,2,3)`.
//!
//! The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4,
//! and 5, giving the pandigital, `918273645`, which is the concatenated product
//! of 9 and `(1,2,3,4,5)`.
//!
//! What is the largest 1 to 9 pandigital 9-digit number that can be formed as
//! the concatenated product of an integer with `(1, 2,..., n)` where `n > 1`?

use itertools::Itertools;

use crate::numbers::{digits, num_from_digits};

pub fn solve() -> crate::Answer {
    // There are 4 cases of k * (1...n):
    // - n = 2: k has 4 digits, 2 * k has 5 digits
    // - n = 3: k, 2 * k, 3 * k have 3 digits each
    // - n = 4: k, 2 * k, 3 * k have 2 digits, 4 * k has 3 digits
    // - n = 5: k has 1 digit

    // We will just iterate from 1 to 10000 (right-exclusive) and find possible
    // pandigital products

    (1..10_000)
        .flat_map(find_pandigital_product)
        .max()
        .unwrap()
        .into()
}

fn find_pandigital_product(k: u32) -> Option<u64> {
    let mut all_digits: Vec<u8> = vec![];
    for n in 1..=5 {
        all_digits.extend(digits(k * n).iter().rev());
        if all_digits.len() == 9 {
            break;
        }
        if all_digits.len() > 9 {
            return None;
        }
    }

    // The digits must be unique from 1 to 9
    if all_digits.iter().unique().count() != 9 || all_digits.contains(&0) {
        return None;
    }

    Some(num_from_digits(all_digits))
}

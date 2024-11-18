//! It is possible to write five as a sum in exactly six different ways:
//!
//! ```text
//! 4 + 1
//! 3 + 2
//! 3 + 1 + 1
//! 2 + 2 + 1
//! 2 + 1 + 1 + 1
//! 1 + 1 + 1 + 1 + 1
//! ```
//!
//! How many different ways can one hundred be written as a sum of at least two
//! positive integers?

use itertools::Itertools;

use crate::misc::coins_sum;

pub fn solve() {
    let n = 100;
    let result = coins_sum((1..n).collect_vec(), n);
    println!("{result}");
}

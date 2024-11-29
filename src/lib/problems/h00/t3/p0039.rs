//! If `p` is the perimeter of a right angle triangle with integral length
//! sides, `{a, b, c}`, there are exactly three solutions for `p = 120`.
//!
//! ```text
//! {20,48,52}, {24,45,51}, {30,40,50}
//! ```
//!
//! For which value of `p <= 1000`, is the number of solutions maximised?

use crate::misc::pythagorean_triplet_count_by_sum;

pub fn solve() -> crate::Answer {
    let counter = pythagorean_triplet_count_by_sum(1000);

    counter.iter().max_by_key(|(_, v)| *v).unwrap().0.into()
}

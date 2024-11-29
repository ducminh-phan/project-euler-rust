//! All square roots are periodic when written as continued fractions and can be
//! written in the form:
//!
//! ```text
//! sqrt(N) = a0 + 1 / (a1 + 1/(a2 + 1/(a3 + ...)))
//! ```
//!
//! For example, let us consider `sqrt(23)`:
//!
//! ```text
//! sqrt(23) = 4 + sqrt(23) - 4 = 4 + 1/(1/(sqrt(23) - 4))
//!     = 4 + 1/(1 + (sqrt(23) - 3)/7)
//! ```
//!
//! If we continue we would get the following expansion:
//!
//! ```text
//! sqrt(23) = 4 + 1/(1 + 1/(3 + 1/(1 + 1/(8 + ...))))
//! ```
//!
//! It can be seen that the sequence is repeating. For conciseness, we use the
//! notation `sqrt(23) = [4;(1,3,1,8)]`, to indicate that the block `(1,3,1,8)`
//! repeats indefinitely.
//!
//! The first ten continued fraction representations of (irrational) square roots are:
//!
//! * `sqrt(2) = [1; (2)]`, period=`1`
//! * `sqrt(3) = [1; (1, 2)]`, period=`2`
//! * `sqrt(5) = [2; (4)]`, period=`1`
//! * `sqrt(6) = [2; (2, 4)]`, period=`2`
//! * `sqrt(7) = [2; (1, 1, 1, 4)]`, period=`4`
//! * `sqrt(8) = [2; (1, 4)]`, period=`2`
//! * `sqrt(10) = [3; (6)]`, period=`1`
//! * `sqrt(11) = [3; (3, 6)]`, period=`2`
//! * `sqrt(12) = [3; (2, 6)]`, period=`2`
//! * `sqrt(13) = [3; (1, 1, 1, 1, 6)]`, period=`5`
//!
//! Exactly four continued fractions, for `N <= 13`, have an odd period.
//!
//! How many continued fractions for `N <= 10,000` have an odd period?

use crate::continued_fractions::compute_continued_fraction_sqrt;
use crate::numbers::is_square;

pub fn solve() -> crate::Answer {
    (2..=10_000)
        .filter(|n| !is_square(*n))
        .map(compute_continued_fraction_sqrt)
        .filter(|(_, cycle)| cycle.len() % 2 == 1)
        .count()
        .into()
}

//! The square root of `2` can be written as an infinite continued fraction.
//!
//! ```text
//! sqrt(2) = 1 + 1/(2 + 1/(2 + 1/(2 + 1/(2 + ...))))
//! ```
//!
//! The infinite continued fraction can be written, `sqrt(2) = [1; (2)]`, `(2)`
//! indicates that `2` repeats *ad infinitum*. In a similar way,
//! `sqrt(23) = [4; (1, 3, 1, 8)]`.
//!
//! It turns out that the sequence of partial values of continued fractions for
//! square roots provide the best rational approximations. Let us consider the
//! convergents for `sqrt(2)`.
//!
//! ```text
//! 1 + 1/2 = 3/2
//! 1 + 1/(2 + 1/2) = 7/5
//! 1 + 1/(1 + 1/(2 + 1/2)) = 17/12
//! 1 + 1/(1 + 1/(1 + 1/(2 + 1/2))) = 41/29
//! ```
//!
//! Hence the sequence of the first ten convergents for `sqrt(2)` are:
//!
//! ```text
//! 1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378,...
//! ```
//!
//! What is most surprising is that the important mathematical constant,  
//! ```text
//! e = [2; 1, 2, 1, 1, 4, 1, 1, 6, 1, ... , 1, 2k, 1, ...]
//! ```
//!
//! The first ten terms in the sequence of convergents for `e` are:
//!
//! ```text
//! 2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536,...
//! ```
//!
//! The sum of digits in the numerator of the `10`th convergent is
//! `1 + 4 + 5 + 7 = 17`.
//!
//! Find the sum of digits in the numerator of the `100`th convergent of the
//! continued fraction for `e`.

use crate::continued_fractions::Convergents;
use crate::numbers::sum_of_digits;

pub fn solve() -> crate::Answer {
    let iter = Box::new((1u8..).flat_map(|k| [1, 2 * k, 1]));
    let mut convergents = Convergents::from_iter(2u8, iter);

    let (n, _) = convergents.nth(99).unwrap();
    sum_of_digits(n).into()
}

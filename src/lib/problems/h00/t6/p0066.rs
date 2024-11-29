//! Consider quadratic Diophantine equations of the form:
//!
//! ```text
//! x^2 - Dy^2 = 1
//! ```
//!
//! For example, when `D=13`, the minimal solution in `x` is
//! `649^2 - 13 * 180^2 = 1`.
//!
//! It can be assumed that there are no solutions in positive integers when
//! `D` is square.
//!
//! By finding minimal solutions in `x` for `D = {2, 3, 5, 6, 7}`, we obtain the
//! following:
//!
//! ```text
//! 3^2 - 2 * 2^2 = 1
//! 2^2 - 3 * 1^2 = 1
//! 9^2 - 5 * 4^2 = 1
//! 5^2 - 6 * 2^2 = 1
//! 8^2 - 7 * 3^2 = 1
//! ```
//!
//! Hence, by considering minimal solutions in `x` for `D <= 7`, the largest `x`
//! is obtained when `D = 5`.
//!
//! Find the value of `D <= 1000` in minimal solutions of `x` for which the
//! largest value of `x` is obtained.

use num::BigUint;

use crate::continued_fractions::{
    compute_continued_fraction_sqrt, Convergents,
};
use crate::numbers::is_square;
use crate::utils::parse_env;

pub fn solve() -> crate::Answer {
    let ceiling = parse_env("CEILING", 1000);
    (2..=ceiling)
        .filter(|n| !is_square(*n))
        .map(|n| (n, find_fundamental_solution(n)))
        .max_by(|a, b| a.1 .0.cmp(&b.1 .0))
        .map(|(n, _)| n)
        .unwrap()
        .into()
}

/// https://en.wikipedia.org/wiki/Pell%27s_equation#Fundamental_solution_via_continued_fractions
fn find_fundamental_solution(n: u64) -> (BigUint, BigUint) {
    let (m, cycle) = compute_continued_fraction_sqrt(n);
    let period = cycle.len();
    let mut convergents = Convergents::from_cycle(m, &cycle);

    let r = if period % 2 == 0 {
        period - 1
    } else {
        2 * period - 1
    };

    convergents.nth(r).unwrap()
}

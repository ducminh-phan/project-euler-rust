//! The fraction `49/98` is a curious fraction, as an inexperienced
//! mathematician in attempting to simplify it may incorrectly believe that
//! `49/98 = 4/8`, which is correct, is obtained by cancelling the 9s.
//!
//! We shall consider fractions like, `30/50 = 3/5`, to be trivial examples.
//!
//! There are exactly four non-trivial examples of this type of fraction, less
//! than one in value, and containing two digits in the numerator and
//! denominator.
//!
//! If the product of these four fractions is given in its lowest common terms,
//! find the value of the denominator.

use itertools::iproduct;
use num::integer::gcd;

pub fn solve() {
    // We need to find the digits a, b, c such that ab/bc = a/c, or
    // c * (10a + b) = a * (10b + c)

    let digits = iproduct!(1..10, 1..10, 1..10)
        .filter(|(a, b, c)| c * (10 * a + b) == a * (10 * b + c) && a < b)
        .collect::<Vec<_>>();

    let fractions = digits
        .into_iter()
        .map(|(a, b, c)| (10 * a + b, 10 * b + c))
        .collect::<Vec<_>>();

    dbg!(&fractions
        .iter()
        .map(|(n, d)| format!("{n}/{d}"))
        .collect::<Vec<_>>());

    let (n, d) = fractions
        .into_iter()
        .reduce(|(n_acc, d_acc), (n, d)| (n_acc * n, d_acc * d))
        .unwrap();

    let result = d / gcd(n, d);
    println!("{result}");
}

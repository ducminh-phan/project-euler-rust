//! There are exactly ten ways of selecting three from five, `12345`:
//!
//! ```text
//! 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
//! ```
//!
//! In combinatorics, we use the notation, `5C3 = 10`.
//!
//! In general, `nCr = n! / (r!(n-r)!)`, where `r <= n`,
//! `n! = n * (n-1) * ... * 3 * 2 * 1`, and `0! = 1`.
//!
//! It is not until `n = 23`, that a value exceeds one-million:
//! `23C10 = 1144066`.
//!
//! How many, not necessarily distinct, values of `nCr` for `1 <= n <= 100`,
//! are greater than one-million?

pub fn solve() -> crate::Answer {
    let limit = 1e6 as u32;

    let mut result = 0;
    let mut pascal_row: Vec<u32> = vec![0, 1, 0];
    for i in 1..=100usize {
        let mut next_row = vec![0];
        next_row.extend(&pascal_row);
        for j in 1..=(i + 1) {
            let p = (pascal_row[j] + pascal_row[j - 1]).min(limit + 1);
            if p > limit {
                result += 1;
            }
            next_row[j] = p;
        }

        pascal_row = next_row;
    }

    result.into()
}

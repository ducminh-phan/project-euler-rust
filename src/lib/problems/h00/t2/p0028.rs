//! Starting with the number 1 and moving to the right in a clockwise direction,
//! a 5 by 5 spiral is formed as follows:
//!
//! ```text
//! 21 22 23 24 25
//! 20  7  8  9 10
//! 19  6  1  2 11
//! 18  5  4  3 12
//! 17 16 15 14 13
//! ```
//!
//! It can be verified that the sum of the numbers on the diagonals is 101.
//!
//! What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral
//! formed in the same way?

pub fn solve() {
    // 73 __ __ __ __ __ __ __ 81
    // __ 43 44 45 46 47 48 49 50
    // __ 42 21 22 23 24 25 26 51
    // __ 41 20  7  8  9 10 27 52
    // __ 40 19  6  1  2 11 28 53
    // __ 39 18  5  4  3 12 29 54
    // __ 38 17 16 15 14 13 30 55
    // __ 37 36 35 34 33 32 31 56
    // 65 __ __ __ __ __ __ __ 57

    // The formula for the sequence 1-3-13-31-57 is 4n^2 - 10n + 7.
    // The formula for the sum of the numbers at the corners is 4(4n^2 + n + 1).
    // The explicit formula for the answer is 2n * (8n^2 + 15n + 13)/3.
    // 1001 by 1001 spiral => n = (1001 - 1) / 2 = 500.

    let n = 500;

    // Don't forget to add 1 at the center
    let result = 2 * n * (8 * n * n + 15 * n + 13) / 3 + 1;

    println!("{result}");
}

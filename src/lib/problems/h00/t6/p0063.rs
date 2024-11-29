//! The `5`-digit number, `16807=7^5`, is also a fifth power. Similarly, the
//! `9`-digit number, `134217728=8^9`, is a ninth power.
//!
//! How many `n`-digit positive integers exist which are also an `n`th power?

pub fn solve() -> crate::Answer {
    // k is an n-th power
    // => k = b^n >= 1^n
    // k is an n-digit positive integers
    // => 10^n > k >= 10^(n-1)
    // => 1 <= b < 10 and b^n >= 10^(n-1)
    // => 1 <= b < 10 and n * log10(b) >= n-1

    (1u32..10)
        .flat_map(|b| {
            (1u32..).find(|&n| (n as f64) * (b as f64).log10() < (n - 1) as f64)
        })
        .map(|n| n - 1)
        .sum::<u32>()
        .into()
}

//! Find the sum of all the multiples of 3 or 5 below 1000.

pub fn solve() -> crate::Answer {
    (1..1000)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum::<u32>()
        .into()
}

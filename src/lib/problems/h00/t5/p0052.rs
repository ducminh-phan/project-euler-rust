//! It can be seen that the number, `125874`, and its double, `251748`, contain
//! exactly the same digits, but in a different order.
//!
//! Find the smallest positive integer, `x`, such that `2x`, `3x`, `4x`, `5x`,
//! and `6x`, contain the same digits.

use itertools::Itertools;

use crate::numbers::digits;

pub fn solve() {
    for i in 1u64.. {
        if (1..=6).map(|x| x * i).map(normalize).unique().count() == 1 {
            let result = i;
            dbg!(result);

            return;
        }
    }
}

fn normalize(n: u64) -> String {
    digits(n)
        .iter()
        .sorted()
        .map(|c| c.to_string())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize(125874), "124578");
        assert_eq!(normalize(251748), "124578");
    }
}

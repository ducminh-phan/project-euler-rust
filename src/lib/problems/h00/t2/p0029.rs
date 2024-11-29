//! Consider all integer combinations of `a^b` for `2 <= a <= 5` and
//! `2 <= b <= 5`. If they are then placed in numerical order, with any repeats
//! removed, we get the following sequence of 15 distinct terms:
//!
//! ```text
//! 4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125.
//! ```
//!
//! How many distinct terms are in the sequence generated by `a^b` for
//! `2 <= a <= 100` and `2 <= b <= 100`?

use std::collections::HashMap;

const MAX: u64 = 100;

pub fn solve() -> crate::Answer {
    // If a1^b1 = a2^b2, then a1 = a^p1, a2 = a^p2, b1*p1 = b2*p2 for some "a".
    // We will mark, e.g., 8^60 is a duplicate of 4^90, and 4^50 is a duplicate
    // of 2^100. To do this, first we create the map of a^p1 to (a, p1).
    let base_map = {
        let mut tmp = HashMap::<u64, (u64, u64)>::new();

        for a in 2..=10u64 {
            let max_p = 100f64.log(a as f64).floor() as u64;
            for b in 2..=max_p {
                tmp.entry(a.pow(b as u32)).or_insert((a, b));
            }
        }

        tmp.shrink_to_fit();
        tmp
    };

    let mut duplicates_count = 0;

    for (_a, (_p, q)) in base_map.iter() {
        for b in 2..=MAX {
            // Now a = p^q for some q >= 2, a^b = p^(bq). a^b is a duplicate if
            // there exists q1 < q such that a1 = p^q1, a1^b1 = a^b
            // => b1q1 = bq => q1 | bq
            if (1..*q).any(|q1| {
                b * q % q1 == 0 // q1 | bq
                && b * q / q1 <= MAX // and b1 <= 100
            }) {
                duplicates_count += 1;
            }
        }
    }

    let result = (MAX - 1).pow(2) - duplicates_count;
    result.into()
}

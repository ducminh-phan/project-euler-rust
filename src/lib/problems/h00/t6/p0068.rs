//! Consider the following "magic" 3-gon ring, filled with the numbers 1 to 6,
//! and each line adding to nine.
//!
//! ![](https://projecteuler.net/resources/images/0068_1.png?1678992052)  
//!
//! Working **clockwise**, and starting from the group of three with the
//! numerically lowest external node (4,3,2 in this example), each solution can
//! be described uniquely. For example, the above solution can be described by
//! the set: `4,3,2; 6,2,1; 5,1,3`.
//!
//! It is possible to complete the ring with four different totals: 9, 10, 11,
//! and 12. There are eight solutions in total.
//!
//! |**Total**| **Solution Set**  |
//! |---------|-------------------|
//! |    9    |4,2,3; 5,3,1; 6,1,2|
//! |    9    |4,3,2; 6,2,1; 5,1,3|
//! |   10    |2,3,5; 4,5,1; 6,1,3|
//! |   10    |2,5,3; 6,3,1; 4,1,5|
//! |   11    |1,4,6; 3,6,2; 5,2,4|
//! |   11    |1,6,4; 5,4,2; 3,2,6|
//! |   12    |1,5,6; 2,6,4; 3,4,5|
//! |   12    |1,6,5; 3,5,4; 2,4,6|
//!
//! By concatenating each group it is possible to form 9-digit strings; the
//! maximum string for a 3-gon ring is 432621513.
//!
//! Using the numbers 1 to 10, and depending on arrangements, it is possible
//! to form 16- and 17-digit strings. What is the maximum **16-digit** string
//! for a "magic" 5-gon ring?
//!
//! ![](https://projecteuler.net/resources/images/0068_2.png?1678992052)

use itertools::Itertools;

pub fn solve() {
    let result = (1..=10)
        .permutations(10)
        .map(to_array)
        .filter(is_min_first)
        .filter(is_magic)
        .map(to_string)
        .filter(|s| s.len() == 16)
        .max()
        .unwrap();

    println!("{result}");
}

fn to_array(permutation: Vec<u8>) -> [u8; 10] {
    permutation.try_into().unwrap()
}

fn is_magic([a1, a2, a3, a4, a5, b1, b2, b3, b4, b5]: &[u8; 10]) -> bool {
    [
        a1 + b1 + b2,
        a2 + b2 + b3,
        a3 + b3 + b4,
        a4 + b4 + b5,
        a5 + b5 + b1,
    ]
    .iter()
    .unique()
    .count()
        == 1
}

fn is_min_first(
    [a1, a2, a3, a4, a5, _b1, _b2, _b3, _b4, _b5]: &[u8; 10],
) -> bool {
    &a1 == [a1, a2, a3, a4, a5].iter().min().unwrap()
}

fn to_string([a1, a2, a3, a4, a5, b1, b2, b3, b4, b5]: [u8; 10]) -> String {
    [a1, b1, b2, a2, b2, b3, a3, b3, b4, a4, b4, b5, a5, b5, b1]
        .iter()
        .join("")
}

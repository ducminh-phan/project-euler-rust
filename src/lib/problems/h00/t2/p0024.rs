//! A permutation is an ordered arrangement of objects. For example, 3124 is one
//! possible permutation of the digits 1, 2, 3 and 4. If all of the permutations
//! are listed numerically or alphabetically, we call it lexicographic order.
//! The lexicographic permutations of 0, 1 and 2 are:
//! ```text
//! 012   021   102   120   201   210
//! ```
//! What is the millionth lexicographic permutation of the digits
//! 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

use itertools::Itertools;
use num::pow;

pub fn solve() -> crate::Answer {
    // https://github.com/starblue/permutations/blob/bc925efc3cb4fe759ae8523884605368098b4546/src/permutations.rs#L18
    let index: usize = pow(10, 6) - 1;
    let n: usize = 10;
    let len = (2..=n).product::<usize>();

    let mut permutation = Vec::new();
    let mut elements = (0..n).collect::<Vec<_>>();
    let mut divisor = len;
    let mut k = n;
    let mut i = index;
    while k > 0 {
        // divisor is (n-1)!
        divisor /= k;

        // j is the index in the current list of elements to fill
        // into `permutation`
        let j = i / divisor;

        permutation.push(elements.remove(j));

        // Mutate after filling one element
        i %= divisor;
        k -= 1;
    }

    permutation.iter().join("").into()
}

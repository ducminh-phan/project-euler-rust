//! A spider, S, sits in one corner of a cuboid room, measuring `6` by `5` by
//! `3`, and a fly, F, sits in the opposite corner. By travelling on the
//! surfaces of the room the shortest "straight line" distance from S to F is
//! `10` and the path is shown on the diagram.
//!
//! ![](https://projecteuler.net/resources/images/0086.png?1678992052)  
//!
//! However, there are up to three "shortest" path candidates for any given
//! cuboid and the shortest route doesn't always have integer length.
//!
//! It can be shown that there are exactly `2060` distinct cuboids, ignoring
//! rotations, with integer dimensions, up to a maximum size of `M` by `M` by
//! `M`, for which the shortest route has integer length when `M = 100`. This is
//! the least value of `M` for which the number of solutions first exceeds two
//! thousand; the number of solutions when `M = 99` is `1975`.
//!
//! Find the least value of `M` such that the number of solutions first exceeds
//! one million.

use crate::numbers::is_square;
use crate::utils::parse_env;

pub fn solve() {
    let ceiling: u64 = parse_env("CEILING", 1_000_000);
    let (result, _) = (1..)
        .map(|m| (m, count_cuboids(m)))
        .scan((0, 0), |state, (m, c)| {
            *state = (m, state.1 + c);
            Some(*state)
        })
        .find(|(_, c)| *c > ceiling)
        .unwrap();

    println!("{result}");
}

fn count_cuboids(a: u64) -> u64 {
    // Assuming a >= b >= c, the shortest route is always
    // sqrt(a^2 + (b + c)^2).
    // Therefore, we need to find s such that 1 <= s < 2*a and a^2 + s^2
    // is a square, then find the number of pairs (b, c) such that
    // b + c = s and a >= b >= c, which is (min(a, s-1) - (s+1)/2 + 1).

    (1..2 * a)
        .filter(|s| is_square(a * a + s * s))
        .map(|s| a.min(s - 1) - (s + 1) / 2 + 1)
        .sum::<u64>()
}

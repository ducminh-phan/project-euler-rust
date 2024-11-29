//! It is easily proved that no equilateral triangle exists with integral
//! length sides and integral area. However, the almost equilateral triangle
//! `5`-`5`-`6` has an area of `12` square units.
//!
//! We shall define an almost equilateral triangle to be a triangle for which
//! two sides are equal and the third differs by no more than one unit.
//!
//! Find the sum of the perimeters of all almost equilateral triangles with
//! integral side lengths and area and whose perimeters do not exceed one
//! billion (`1,000,000,000`).

use crate::numbers::is_square;
use crate::utils::parse_env;

pub fn solve() -> crate::Answer {
    // Let a-a-(a±1) be the sides of the triangle, we can prove by contradiction
    // that `a` must be an odd number. Let b = 2k be the base of the triangle.
    // => a = 2k ± 1, perimeter = 6k ± 2

    let max_p = parse_env("MAX_P", 1e9 as u64);
    (3u64..)
        .take_while(|k| 6 * k - 2 <= max_p)
        .flat_map(|k| [(k, 2 * k + 1), (k, 2 * k - 1)])
        .filter(|(k, a)| is_square(a * a - k * k))
        .map(|(k, a)| 2 * (k + a))
        .sum::<u64>()
        .into()
}

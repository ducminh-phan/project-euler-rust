//! The points `P(x1, y1)` and `Q(x2, y2)` are plotted at integer co-ordinates
//! and are joined to the origin, `O(0,0)`, to form triangle `OPQ`.
//!
//! ![](https://projecteuler.net/resources/images/0091_1.png?1678992052)  
//!
//! There are exactly fourteen triangles containing a right angle that can be
//! formed when each co-ordinate lies between `0` and `2` inclusive; that is,
//! `0 <= x1, y1, x2, y2 <= 2`.
//!
//! ![](https://projecteuler.net/resources/images/0091_2.png?1678992052)  
//!
//! Given that `0 <= x1, y1, x2, y2 <= 50`, how many right triangles can be
//! formed?

use std::iter::repeat_n;

use itertools::Itertools;

pub fn solve() {
    let result = repeat_n(0..=50, 4)
        .multi_cartesian_product()
        .map(convert_type)
        .filter(|&[x1, y1, x2, y2]| {
            // Points must be distinct, and we need to account for
            // point1-point2 symmetry
            (x1, y1) > (x2, y2) && (x1, y1) != (0, 0) && (x2, y2) != (0, 0)
        })
        .map(|[x1, y1, x2, y2]| {
            (
                x1 * x1 + y1 * y1,
                x2 * x2 + y2 * y2,
                (x1 - x2).pow(2) + (y1 - y2).pow(2),
            )
        })
        .filter(|&(a, b, c)| a + b == c || b + c == a || c + a == b)
        .count();

    println!("{result}");
}

fn convert_type(v: Vec<i32>) -> [i32; 4] {
    v.try_into().unwrap()
}

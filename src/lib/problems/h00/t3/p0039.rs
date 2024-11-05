//! If `p` is the perimeter of a right angle triangle with integral length
//! sides, `{a, b, c}`, there are exactly three solutions for `p = 120`.
//!
//! ```text
//! {20,48,52}, {24,45,51}, {30,40,50}
//! ```
//!
//! For which value of `p <= 1000`, is the number of solutions maximised?

use std::collections::{HashMap, HashSet};

use num::integer::Roots;

const MAX_P: u32 = 1000;

pub fn solve() {
    // a = k * (m^2 - n^2), b = 2 * k * m * n, c = k * (m^2 + n^2)
    // => MAX_P >= p = 2 * k * m * (m + n) > 2k * m^2
    // => MAX_P / 4 > k * m^2
    // => m^2 < MAX_P / 4

    let mut counter = HashMap::new();
    let mut visited = HashSet::new();

    for m in 2..=(MAX_P.sqrt() / 2) {
        let max_k = MAX_P / 2 / m.pow(2);
        for k in 1..=max_k {
            for n in 1..m {
                let mut sides = vec![
                    k * (m * m - n * n),
                    2 * k * m * n,
                    k * (m * m + n * n),
                ];
                if sides[0] > sides[1] {
                    sides.swap(0, 1);
                }

                let p = 2 * k * m * (m + n);

                if !visited.contains(&sides) && p <= MAX_P {
                    counter.entry(p).and_modify(|v| *v += 1).or_insert(1);
                }

                visited.insert(sides);
            }
        }
    }

    let r = counter.iter().max_by_key(|(_, v)| *v).unwrap();
    println!("{}", *r.0);
}

use std::collections::{HashMap, HashSet};

use num::integer::Roots;

pub fn pythagorean_triplet_count_by_sum(ceiling: u64) -> HashMap<u64, u64> {
    // a = k * (m^2 - n^2), b = 2 * k * m * n, c = k * (m^2 + n^2)
    // => MAX_P >= p = 2 * k * m * (m + n) > 2k * m^2
    // => MAX_P / 2 > k * m^2
    // => m^2 < MAX_P / 2

    let mut counter = HashMap::new();
    let mut visited = HashSet::new();

    for m in 2..=(ceiling / 2).sqrt() {
        let max_k = ceiling / 2 / m.pow(2);
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

                if !visited.contains(&sides) && p <= ceiling {
                    counter.entry(p).and_modify(|v| *v += 1).or_insert(1);
                }

                visited.insert(sides);
            }
        }
    }

    counter
}

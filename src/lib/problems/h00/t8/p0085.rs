//! By counting carefully it can be seen that a rectangular grid measuring
//! `3` by `2` contains eighteen rectangles:
//!
//! ![](https://projecteuler.net/resources/images/0085.png?1678992052)
//!
//! Although there exists no rectangular grid that contains exactly two million
//! rectangles, find the area of the grid with the nearest solution.

pub fn solve() {
    // A rectangle of size m * n contains m*(m + 1) * n*(n + 1) / 4 rectangles.
    // W.L.O.G, assuming m >= n, then n <= max_n.
    let (_, max_n) = solve_eq(8e6f64.sqrt());
    dbg!(max_n);

    let (result, _) = (1..=max_n)
        .map(|n| (n, solve_eq(8e6f64 / (n * (n + 1)) as f64)))
        .flat_map(|(n, (m1, m2))| [(n, m1), (n, m2)])
        .map(|(n, m)| (m * n, m * (m + 1) * n * (n + 1) / 4))
        .min_by_key(|(_, count)| (count - (2e6 as i32)).abs())
        .unwrap();

    println!("{result}");
}

/// Solve the equation x * (x + 1) ≈ k, the answer is (x1, x2), where
/// x1 * (x1 + 1) < k < x2 * (x2 + 1)
fn solve_eq(k: f64) -> (i32, i32) {
    let x = (4. * k + 1.).sqrt() / 2. - 0.5;
    (x.floor() as i32, x.ceil() as i32)
}

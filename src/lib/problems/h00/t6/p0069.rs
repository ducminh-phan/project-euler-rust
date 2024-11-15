//! Euler's totient function, `Φ(n)` (sometimes called the phi function), is
//! defined as the number of positive integers not exceeding `n` which are
//! relatively prime to `n`. For example, as `1`, `2`, `4`, `5`, `7`, and `8`,
//! are all less than or equal to nine and relatively prime to nine, `Φ(9)=6`.
//!
//! | `n`|**Relatively Prime**|  **`Φ(n)`**  |  **`n/Φ(n)`**  |
//! | ---|--------------------|--------------|----------------|
//! |  2 |         1          |      1       |       2        |
//! |  3 |        1,2         |      2       |      1.5       |
//! |  4 |        1,3         |      2       |       2        |
//! |  5 |      1,2,3,4       |      4       |      1.25      |
//! |  6 |        1,5         |      2       |       3        |
//! |  7 |    1,2,3,4,5,6     |      6       |   1.1666...    |
//! |  8 |      1,3,5,7       |      4       |       2        |
//! |  9 |    1,2,4,5,7,8     |      6       |      1.5       |
//! | 10 |      1,3,7,9       |      4       |      2.5       |
//!
//! It can be seen that `n = 6` produces a maximum `n/Φ(n)` for `n <= 10`.
//!
//! Find the value of `n <= 1,000,000` for which `n/Φ(n)` is a maximum.

pub fn solve() {
    let ceiling = 1e6 as u64;
    let phi = compute_phi_to_n(ceiling);
    let (result, q) = (1..=ceiling)
        .map(|n| (n, (n as f64) / phi[n as usize] as f64))
        .max_by(|(_, a), (_, b)| a.total_cmp(b))
        .unwrap();

    dbg!(q);

    println!("{result}");
}

fn compute_phi_to_n(n: u64) -> Vec<u64> {
    let mut phi: Vec<u64> = (0..=(n + 1)).collect();
    for i in 2..=(n) {
        let i_size = i as usize;
        if phi[i_size] == i {
            for j in (i..=n).step_by(i_size) {
                phi[j as usize] -= phi[j as usize] / i;
            }
        }
    }

    phi
}

//! By using each of the digits from the set, `{1, 2, 3, 4}`, exactly once, and
//! making use of the four arithmetic operations (`+, -, *, /`) and
//! brackets/parentheses, it is possible to form different positive integer
//! targets.
//!
//! For example,
//!
//! ```text
//!  8 = (4 * (1 + 3)) / 2
//! 14 = 4 * (3 + 1 / 2)
//! 19 = 4 * (2 + 3) - 1
//! 36 = 3 * 4 * (2 + 1)
//! ```
//!
//! Note that concatenations of the digits, like `12 + 34`, are not allowed.
//!
//! Using the set, `{1, 2, 3, 4}`, it is possible to obtain thirty-one different
//! target numbers of which `36` is the maximum, and each of the numbers `1` to
//! `28` can be obtained before encountering the first non-expressible number.
//!
//! Find the set of four distinct digits, `a < b < c < d`, for which the longest
//! set of consecutive positive integers, `1` to `n`, can be obtained, giving
//! your answer as a string: `abcd`.

use std::collections::HashSet;

use itertools::Itertools;

pub fn solve() {
    let (ds, _) = (1..10)
        .combinations(4)
        .map(|ds| {
            let ds_as_f64 = ds.iter().map(|n| *n as f64).collect_vec();
            (ds, evaluate(&ds_as_f64))
        })
        .map(|(ds, values)| (ds, find_max_n(&values)))
        .max_by_key(|(_, max_n)| *max_n)
        .unwrap();

    let result = ds.into_iter().join("");
    println!("{result}");
}

fn evaluate(nums: &[f64]) -> Vec<u64> {
    let mut results = HashSet::new();
    evaluate_inner(nums, &mut results);

    results.remove(&0);
    results.into_iter().sorted().collect()
}

fn evaluate_inner(nums: &[f64], results: &mut HashSet<u64>) {
    if nums.len() == 1 && is_integer(nums[0]) {
        results.insert(nums[0] as u64);
        return;
    }

    nums.iter()
        .enumerate()
        .combinations(2)
        .map(convert_type)
        .map(|[(i, m), (j, n)]| {
            let remaining = nums
                .iter()
                .enumerate()
                .filter_map(
                    |(k, x)| if k != i && k != j { Some(*x) } else { None },
                )
                .collect_vec();

            let mut news = vec![m + n, (m - n).abs(), m * n];
            if *n != 0. {
                news.push(m / n);
            }
            if *m != 0. {
                news.push(n / m);
            }

            (remaining, news)
        })
        .flat_map(|(remaining, news)| {
            news.iter()
                .map(|new| {
                    let mut nums = remaining.clone();
                    nums.push(*new);
                    nums
                })
                .collect_vec()
        })
        .for_each(|nums| evaluate_inner(&nums, results));
}

fn is_integer(n: f64) -> bool {
    n.fract().abs() < 1e-6
}

fn convert_type<T: std::fmt::Debug>(v: Vec<T>) -> [T; 2] {
    v.try_into().unwrap()
}

fn find_max_n(nums: &[u64]) -> usize {
    if let Some((i, _)) =
        nums.iter().enumerate().find(|&(i, n)| *n > i as u64 + 1)
    {
        i
    } else {
        nums.len()
    }
}

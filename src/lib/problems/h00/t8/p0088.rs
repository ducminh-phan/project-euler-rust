//! A natural number, `N`, that can be written as the sum and product of a given
//! set of at least two natural numbers, `{a(1), a(2),..., a(k)}` is
//! called a product-sum number:
//!
//! ```text
//! N = a(1) + a(2) + ... + a(k) = a(1) * a(2) * ... * a(k)
//! ```
//!
//! For example, `6 = 1 + 2 + 3 = 1 * 2 * 3`.
//!
//! For a given set of size, `k`, we shall call the smallest `N` with this
//! property a minimal product-sum number. The minimal product-sum numbers for
//! sets of size, `k = 2, 3, 4, 5`, and `6` are as follows.
//!
//! * `k = 2: 4 = 2 * 2 = 2 + 2`
//! * `k = 3: 6 = 1 * 2 * 3 = 1 + 2 + 3`
//! * `k = 4: 8 = 1 * 1 * 2 * 4 = 1 + 1 + 2 + 4`
//! * `k = 5: 8 = 1 * 1 * 2 * 2 * 2 = 1 + 1 + 2 + 2 + 2`
//! * `k = 6: 12 = 1 * 1 * 1 * 1 * 2 * 6 = 1 + 1 + 1 + 1 + 2 + 6`
//!
//! Hence for `2 <= k <= 6`, the sum of all the minimal product-sum numbers is
//! `4 + 6 + 8 + 12 = 30`; note that `8` is only counted once in the sum.
//!
//! In fact, as the complete set of minimal product-sum numbers for
//! `2 <= k <= 12` is `{4, 6, 8, 12, 15, 16}`, the sum is `61`.
//!
//! What is the sum of all the minimal product-sum numbers for
//! `2 <= k <= 12000`?

use itertools::Itertools;

use crate::utils::parse_env;

pub fn solve() -> crate::Answer {
    let max_k: u64 = parse_env("MAX_K", 12000);

    // s[k] is the minimal k-product-sum number
    let mut s = vec![u64::MAX; max_k as usize + 1];

    // Let m(k) is the minimal product-sum number for k, then m(k) <= 2k,
    // as 1^(k-2) * 2 * k = (k-2) * 1 + 2 + k.
    for n in 4..=max_k * 2 {
        for k in find_k(n) {
            if k > max_k {
                continue;
            }

            s[k as usize] = s[k as usize].min(n)
        }
    }

    s[2..].iter().unique().sum::<u64>().into()
}

/// Find all values of `k` such that `n` can be represented as the product and
/// sum of `k` numbers.
fn find_k(n: u64) -> Vec<u64> {
    find_k_generic(n, n, 0, 2)
}

/// Generic and recursive version of `find_k`, where we are given `p` and `s`
/// and check if `p` and `s` can be written as product and sum of `k` numbers,
/// respectively. For each factor `d` of `p`, we check recursively for `p/d`
/// and `s-d` and add `1` (for `d`) to the result.
fn find_k_generic(
    product: u64,
    sum: u64,
    depth: u64,
    min_factor: u64,
) -> Vec<u64> {
    if product == 1 {
        // k = sum as we need to write `sum` as the sum of all 1s.
        return vec![sum];
    }

    let mut result = (min_factor..)
        .take_while(|&d| d * d <= product)
        .filter(|d| product % d == 0)
        .flat_map(|d| find_k_generic(product / d, sum - d, depth + 1, d))
        .map(|k| k + 1)
        .collect_vec();

    if depth > 0 {
        // Note that we always have product <= sum, and we can write:
        // product = product * 1^(sum - product)
        // sum = product + (sum - product) * 1
        result.push(sum - product + 1);
    }

    result
}

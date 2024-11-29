//! NOTE: This problem is a more challenging version of Problem 81.
//!
//! The minimal path sum in the `5` by `5` matrix below, by starting in any cell
//! in the left column and finishing in any cell in the right column, and only
//! moving up, down, and right, is indicated in red and bold; the sum is equal
//! to `994`.
//!
//! ```text
//!  131    673   *234*  *103*  *018*
//! *201*  *096*  *342*   965    150
//!  630    803    746    422    111
//!  537    699    497    121    956
//!  805    732    524    037    331
//! ```
//!
//! Find the minimal path sum from the left column to the right column in
//! `assets/0082_matrix.txt`, a 31K text file containing an `80` by `80` matrix.

use itertools::Itertools;

use crate::utils::read_file;

pub fn solve() -> crate::Answer {
    let matrix = read_file("assets/0082_matrix.txt", '\n')
        .iter()
        .map(|line| {
            line.split(',').flat_map(|w| w.parse::<u64>()).collect_vec()
        })
        .collect_vec();

    // Transpose the matrix as working row-wise is easier
    let mut matrix = transpose(matrix);

    for row_idx in 0..matrix.len() {
        matrix[row_idx] = compute_new_row(&matrix, row_idx);
    }

    matrix.last().unwrap().iter().min().unwrap().into()
}

fn compute_new_row(matrix: &[Vec<u64>], row_idx: usize) -> Vec<u64> {
    let n = matrix.len();
    let row = &matrix[row_idx];

    let prev_row = if row_idx >= 1 {
        &matrix[row_idx - 1]
    } else {
        &vec![0; n]
    };
    let from_up = prev_row
        .iter()
        .zip(row)
        .map(|(prev, curr)| prev + curr)
        .collect_vec();

    let from_left = from_up
        .iter()
        .zip(row)
        .scan(u64::MAX, |acc, (up, curr)| {
            *acc = (*acc).saturating_add(*curr).min(*up);
            Some(*acc)
        })
        .collect_vec();

    let from_right = from_up
        .iter()
        .zip(row)
        .rev()
        .scan(u64::MAX, |acc, (up, curr)| {
            *acc = (*acc).saturating_add(*curr).min(*up);
            Some(*acc)
        })
        .collect_vec()
        .into_iter()
        .rev()
        .collect_vec();

    from_left
        .iter()
        .zip(from_right)
        .map(|(left, right)| (*left).min(right))
        .collect_vec()
}

fn transpose<T>(m: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!m.is_empty());
    let len = m[0].len();
    let mut iters: Vec<_> = m.into_iter().map(|r| r.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|r| r.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

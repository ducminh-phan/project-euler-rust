//! By starting at the top of the triangle below and moving to adjacent numbers
//! on the row below, the maximum total from top to bottom is 23.
//! ```text
//!  [3]
//!  [7] 4
//!  2   [4] 6
//!  8   5   [9] 3
//! ```
//!
//! That is, `3 + 7 + 4 + 9 = 23`.
//!
//! Find the maximum total from top to bottom of the triangle below.

fn main() {
    #[allow(clippy::zero_prefixed_literal)]
    let triangle = vec![
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20, 04, 82, 47, 65],
        vec![19, 01, 23, 75, 03, 34],
        vec![88, 02, 77, 73, 07, 63, 67],
        vec![99, 65, 04, 28, 06, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23],
    ];

    let mut max_by_elements = vec![vec![triangle[0][0]]];

    for row_idx in 1..triangle.len() {
        max_by_elements.push(vec![]);

        for col_idx in 0..=row_idx {
            let triangle_value = triangle[row_idx][col_idx];

            let mut candidates = vec![];

            // Up-left element exists
            if col_idx > 0 {
                candidates.push(
                    triangle_value + max_by_elements[row_idx - 1][col_idx - 1],
                )
            }

            // Up element exists
            if col_idx < row_idx {
                candidates.push(
                    triangle_value + max_by_elements[row_idx - 1][col_idx],
                )
            }

            let current_max = candidates.iter().max().unwrap();

            max_by_elements[row_idx].push(*current_max)
        }
    }

    let result = max_by_elements.last().unwrap().iter().max().unwrap();
    println!("{}", result)
}

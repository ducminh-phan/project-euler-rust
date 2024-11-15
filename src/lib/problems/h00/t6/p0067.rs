//! By starting at the top of the triangle below and moving to adjacent numbers
//! on the row below, the maximum total from top to bottom is 23.
//!
//! ```text
//! 3  
//! 7 4  
//! 2 4 6  
//! 8 5 9 3
//! ```
//!
//! That is, 3 + 7 + 4 + 9 = 23.
//!
//! Find the maximum total from top to bottom in `assets/0067_triangle.txt`, a
//! 15K text file containing a triangle with one-hundred rows.

use std::fs::OpenOptions;
use std::io::BufRead;

// noinspection DuplicatedCode
// We just duplicate the solution from p0018
pub fn solve() {
    let triangle = read_triangle();

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
    println!("{result}");
}

fn read_triangle() -> Vec<Vec<u64>> {
    let file = OpenOptions::new()
        .read(true)
        .open("assets/0067_triangle.txt")
        .unwrap();
    let mut buf = String::new();
    let mut reader = std::io::BufReader::new(file);

    let mut triangle: Vec<Vec<u64>> = vec![];
    while reader.read_line(&mut buf).unwrap() > 0 {
        let row = buf
            .trim_end()
            .split(' ')
            .flat_map(|s| s.parse())
            .collect::<Vec<_>>();

        triangle.push(row);
        buf.clear();
    }

    triangle
}

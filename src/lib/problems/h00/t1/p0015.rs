//! Starting in the top left corner of a `2 * 2` grid, and only being able
//! to move to the right and down, there are exactly 6 routes to the bottom
//! right corner.
//!
//! How many such routes are there through a `20 * 20` grid?
//!
//! In fact, the number of routes in an `a * b` grid is the number of ways
//! to choose `b` turns right in `a + b` turns. So the formula is
//! ```text
//! aC(a+b) = (a + b)! / (a! * b!)
//! ```

const SIZE: usize = 21;

pub fn solve() {
    let mut grid = [[0u64; SIZE]; SIZE];
    for i in 0..SIZE {
        grid[i][0] = 1;
        grid[0][i] = 1;
    }

    for i in 1..SIZE {
        for j in 1..SIZE {
            grid[i][j] = grid[i - 1][j] + grid[i][j - 1]
        }
    }

    println!("{}", grid[SIZE - 1][SIZE - 1])
}

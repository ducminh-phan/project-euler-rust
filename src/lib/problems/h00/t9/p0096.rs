//! Su Doku (Japanese meaning *number place*) is the name given to a popular
//! puzzle concept. Its origin is unclear, but credit must be attributed to
//! Leonhard Euler who invented a similar, and much more difficult, puzzle idea
//! called Latin Squares. The objective of Su Doku puzzles, however, is to
//! replace the blanks (or zeros) in a 9 by 9 grid in such that each row,
//! column, and 3 by 3 box contains each of the digits 1 to 9. Below is an
//! example of a typical starting puzzle grid and its solution grid.
//!
//! ![](https://projecteuler.net/project/images/p096_1.png)
//!
//! ![](https://projecteuler.net/project/images/p096_2.png)
//!
//! A well constructed Su Doku puzzle has a unique solution and can be solved
//! by logic, although it may be necessary to employ "guess and test" methods
//! in order to eliminate options (there is much contested opinion over this).
//! The complexity of the search determines the difficulty of the puzzle; the
//! example above is considered *easy* because it can be solved by straight
//! forward direct deduction.
//!
//! The 6K text file, `assets/0096_sudoku.txt`, contains fifty different Su Doku
//! puzzles ranging in difficulty, but all with unique solutions (the first
//! puzzle in the file is the example above).
//!
//! By solving all fifty puzzles find the sum of the 3-digit numbers found in
//! the top left corner of each solution grid; for example, 483 is the 3-digit
//! number found in the top left corner of the solution grid above.

use std::collections::HashSet;

use itertools::Itertools;

use crate::numbers::num_from_digits;
use crate::utils::read_file;

pub fn solve() -> crate::Answer {
    read_file("assets/0096_sudoku.txt", '\n')
        .iter()
        .chunks(10)
        .into_iter()
        // Skip numbering lines
        .map(|chunk| chunk.skip(1))
        .map(|mut chunk| chunk.join(""))
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .map(|data| Sudoku(data.try_into().unwrap()))
        .map(|mut sudoku| {
            brute_solve(&mut sudoku);
            num_from_digits(&sudoku.0[..3])
        })
        .sum::<u64>()
        .into()
}

struct Sudoku([u8; 9 * 9]);

impl Sudoku {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.0[x + 9 * y]
    }

    fn set(&mut self, x: usize, y: usize, val: u8) {
        self.0[x + 9 * y] = val;
    }

    fn clear(&mut self, x: usize, y: usize) {
        self.0[x + 9 * y] = 0;
    }

    fn is_solved(&self) -> bool {
        for index in 0..9 {
            // Lambda function used as an argument to `map` method of an iterator
            // (And this language is just as performant as C. Amazing!)
            let different_digits: HashSet<_> = (0..9)
                .map(|col_index| self.get(col_index, index))
                .filter(|d| *d != 0)
                .collect();
            if different_digits.len() < 9 {
                return false;
            }

            let different_digits: HashSet<_> = (0..9)
                .map(|row_index| self.get(index, row_index))
                .filter(|d| *d != 0)
                .collect();
            if different_digits.len() < 9 {
                return false;
            }
        }

        for block_index in 0..9 {
            let bx = block_index / 3;
            let by = block_index % 3;
            let different_digits: HashSet<_> = (0..9)
                .map(|i| {
                    let x = i / 3;
                    let y = i % 3;
                    self.get(x + 3 * bx, y + 3 * by)
                })
                .filter(|d| *d != 0)
                .collect();
            if different_digits.len() < 9 {
                return false;
            }
        }

        true
    }
}

enum SudokuBruteSolveResult {
    Solved,
    NotYet,
}

fn index_to_coords(index: usize) -> (usize, usize) {
    let x = index % 9;
    let y = index / 9;
    (x, y)
}

fn potential_digits(sudoku: &Sudoku, index: usize) -> HashSet<u8> {
    let (x, y) = index_to_coords(index);

    // We'll start with a full set and then filter "manually"
    let mut res = HashSet::from_iter(1..=9);

    for i in 0..9 {
        // No same digits in the same row!
        if i != x {
            match sudoku.get(i, y) {
                0 => (),
                other => {
                    res.remove(&other);
                }
            }
        }

        // No same digits in the same column!
        if i != y {
            match sudoku.get(x, i) {
                0 => (),
                other => {
                    res.remove(&other);
                }
            }
        }
    }

    // No same digits in the same block!
    let bx = x / 3;
    let by = y / 3;
    let local_x = x % 3;
    let local_y = y % 3;
    for other_x in 0..3 {
        for other_y in 0..3 {
            if other_x != local_x && other_y != local_y {
                match sudoku.get(bx * 3 + other_x, by * 3 + other_y) {
                    0 => (),
                    other => {
                        res.remove(&other);
                    }
                }
            }
        }
    }

    res
}

fn brute_solve_aux(
    sudoku: &mut Sudoku,
    index: usize,
) -> SudokuBruteSolveResult {
    // Base case: we went through the whole board!
    if index >= 9 * 9 {
        if sudoku.is_solved() {
            return SudokuBruteSolveResult::Solved;
        }
        return SudokuBruteSolveResult::NotYet;
    }

    let (x, y) = index_to_coords(index);

    let current = sudoku.get(x, y);
    // If there's a digit already it means it must have been an original hint
    // since we clean up after ourselves when we backtrack.
    // We skip over it, since we can't modify it!
    if current > 0 {
        brute_solve_aux(sudoku, index + 1)
    }
    // Opportunity to insert a digit
    else {
        for this_digit in potential_digits(sudoku, index) {
            sudoku.set(x, y, this_digit);
            if let SudokuBruteSolveResult::Solved =
                brute_solve_aux(sudoku, index + 1)
            {
                return SudokuBruteSolveResult::Solved;
            }
        }

        // Clean up after ourselves
        sudoku.clear(x, y);

        // Since at this point we did not find a solution, we return:
        SudokuBruteSolveResult::NotYet
    }
}

fn brute_solve(sudoku: &mut Sudoku) {
    brute_solve_aux(sudoku, 0);
}

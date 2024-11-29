//! Triangle, pentagonal, and hexagonal numbers are generated by the following
//! formulae:
//!
//! Triangle: `T_n = n * (n + 1)/2, 1, 3, 6, 10, 15,...`
//!
//! Pentagonal: `P_n = n * (3n - 1)/2, 1, 5, 12, 22, 35,...`
//!
//! Hexagonal: `H_n = n * (2n - 1), 1, 6, 15, 28, 45,...`
//!
//! It can be verified that `T(285) = P(165) = H(143) = 40755`.
//!
//! Find the next triangle number that is also pentagonal and hexagonal.

use crate::numbers::{is_pentagonal_number, is_triangle_number};

pub fn solve() -> crate::Answer {
    for n in 144.. {
        let t_n = n * (2 * n - 1);
        if is_triangle_number(t_n) && is_pentagonal_number(t_n) {
            return t_n.into();
        }
    }

    panic!("No solution found!");
}

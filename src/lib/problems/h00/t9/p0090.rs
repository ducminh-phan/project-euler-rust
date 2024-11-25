//! Each of the six faces on a cube has a different digit (`0` to `9`) written
//! on it; the same is done to a second cube. By placing the two cubes
//! side-by-side in different positions we can form a variety of `2`-digit
//! numbers.
//!
//! For example, the square number `64` could be formed:
//!
//! ![](https://projecteuler.net/resources/images/0090.png?1678992052)  
//!
//! In fact, by carefully choosing the digits on both cubes it is possible to
//! display all of the square numbers below one-hundred: `01`, `04`, `09`, `16`,
//! `25`, `36`, `49`, `64`, and `81`.
//!
//! For example, one way this can be achieved is by placing `{0, 5, 6, 7, 8, 9}`
//! on one cube and `{1, 2, 3, 4, 8, 9}` on the other cube.
//!
//! However, for this problem we shall allow the `6` or `9` to be turned
//! upside-down so that an arrangement like `{0, 5, 6, 7, 8, 9}` and
//! `{1, 2, 3, 4, 6, 7}` allows for all nine square numbers to be displayed;
//! otherwise it would be impossible to obtain `09`.
//!
//! In determining a distinct arrangement we are interested in the digits on
//! each cube, not the order.
//!
//! * `{1, 2, 3, 4, 5, 6}` is equivalent to `{3, 6, 4, 1, 2, 5}`
//! * `{1, 2, 3, 4, 5, 6}` is distinct from `{1, 2, 3, 4, 5, 9}`
//!
//! But because we are allowing `6` and `9` to be reversed, the two distinct
//! sets in the last example both represent the extended set
//! `{1, 2, 3, 4, 5, 6, 9}` for the purpose of forming `2`-digit numbers.
//!
//! How many distinct arrangements of the two cubes allow for all of the square
//! numbers to be displayed?

use std::sync::LazyLock;

use itertools::Itertools;

static SQUARES: LazyLock<Vec<(u8, u8)>> = LazyLock::new(|| {
    (1..10).map(|n| n * n).map(|n| (n / 10, n % 10)).collect()
});

pub fn solve() {
    let permutations = (0..10u8).combinations(6).map(add_69);

    let result = permutations
        .combinations(2)
        .map(convert_type)
        .filter(is_valid_dices)
        .count();

    println!("{result}");
}

fn add_69(mut v: Vec<u8>) -> Vec<u8> {
    if v.contains(&6) {
        v.push(9);
    } else if v.contains(&9) {
        v.push(6);
    }

    v
}

fn convert_type(vs: Vec<Vec<u8>>) -> [Vec<u8>; 2] {
    vs.try_into().unwrap()
}

fn is_valid_dices([v1, v2]: &[Vec<u8>; 2]) -> bool {
    SQUARES.iter().all(|(d1, d2)| {
        (v1.contains(d1) && v2.contains(d2))
            || (v1.contains(d2) && v2.contains(d1))
    })
}

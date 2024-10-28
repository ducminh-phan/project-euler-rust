//! A Pythagorean triplet is a set of three natural numbers, `a < b < c`,
//! for which, `a^2 + b^2 = c^2`.
//!
//! For example, `3^2 + 4^2 = 5^2`.
//!
//! There exists exactly one Pythagorean triplet for which `a + b + c = 1000`.
//! Find the product `abc`.

use num::pow;

struct PythagoreanTriples {
    limits: u32,
    a: u32,
    b: u32,
    c: u32,
    m: u32,
    n: u32,
}

impl PythagoreanTriples {
    pub fn new(limits: u32) -> Self {
        Self {
            limits,
            a: 0,
            b: 0,
            c: 0,
            m: 2,
            n: 1,
        }
    }
}

impl Iterator for PythagoreanTriples {
    type Item = (u32, u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.c >= self.limits {
            return None;
        }

        self.a = pow(self.m, 2) - pow(self.n, 2);
        self.b = 2 * self.m * self.n;
        self.c = pow(self.m, 2) + pow(self.n, 2);

        self.n += 1;

        if self.n == self.m {
            self.n = 1;
            self.m += 1;
        }

        Some((self.a, self.b, self.c))
    }
}

pub fn main() {
    let mut triples = PythagoreanTriples::new(1000);
    let (a, b, c) = triples.find(|(a, b, c)| a + b + c == 1000).unwrap();
    println!("{}", a * b * c)
}

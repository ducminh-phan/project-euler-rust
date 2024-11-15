use num::integer::{gcd, Roots};
use num::{BigUint, Integer, One, Zero};

pub fn compute_continued_fraction_sqrt(n: u64) -> (u64, Vec<u64>) {
    // Find the nearest square number less than n
    let m = n.sqrt();
    let s = (n as f64).sqrt();

    // Let s = sqrt(n), now we have s = m + 1/x
    // => x = 1/(s - m)
    // => x = (s + m) / (s² - m²)

    // We represent (a * s + b)/d as (a, b, d)
    // => x = (1, m, s² - m²) = (1, m, n - m²)

    let mut cycle = vec![];
    let mut x = (1, m, n - m * m);
    loop {
        let (a, b, d) = x;

        if a == 1 && d == 1 {
            // Now x(n) = s + b for some b
            // => x(n) = (b + m) + 1/x1 and we have a loop
            cycle.push(b + m);
            break;
        }

        let p = (((a as f64) * s + b as f64) / (d as f64)).floor() as u64;
        cycle.push(p);

        // Now x = (a, b, d) = p + (a, b - p*d, d) = p + (a, b', d), we need to
        // compute x_new = 1/(a, b', d) = d/(a * s + b')
        // = d * (a * s - b') / (a² * n - b'²)
        let b1 = p * d - b;
        let d_new = a * a * n - b1 * b1;
        let a_new = a * d;
        let b_new = d * b1;
        let g = gcd3(a_new, b_new, d_new);

        x = (a_new / g, b_new / g, d_new / g);
    }

    (m, cycle)
}

fn gcd3<T: Integer>(a: T, b: T, c: T) -> T {
    gcd(gcd(a, b), c)
}

pub struct Convergents<'a> {
    gen: Box<dyn Iterator<Item = BigUint> + 'a>,

    a0: BigUint,
    b0: BigUint,

    a1: BigUint,
    b1: BigUint,
}

impl<'a> Convergents<'a> {
    pub fn from_iter<N>(k: N, gen: Box<dyn Iterator<Item = N> + 'a>) -> Self
    where
        N: Copy + 'a,
        BigUint: From<N>,
    {
        Self {
            gen: Box::new(gen.into_iter().map(BigUint::from)),

            a0: BigUint::one(),
            b0: BigUint::zero(),

            a1: BigUint::from(k),
            b1: BigUint::one(),
        }
    }

    pub fn from_cycle<N>(k: N, cycle: &'a [N]) -> Self
    where
        N: Copy,
        BigUint: From<N>,
    {
        let iter = Box::new(cycle.iter().copied().cycle());

        Self::from_iter(k, iter)
    }
}

impl<'a> Iterator for Convergents<'a> {
    type Item = (BigUint, BigUint);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(k) = self.gen.next() {
            let item = (self.a1.clone(), self.b1.clone());

            let a_new = k.clone() * &self.a1 + &self.a0;
            let b_new = k.clone() * &self.b1 + &self.b0;

            self.a0 = self.a1.clone();
            self.a1 = a_new;
            self.b0 = self.b1.clone();
            self.b1 = b_new;

            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use num::ToPrimitive;

    use super::*;

    #[test]
    fn test_compute_continued_fraction_sqrt() {
        assert_eq!(compute_continued_fraction_sqrt(7), (2, vec![1, 1, 1, 4]));
        assert_eq!(
            compute_continued_fraction_sqrt(19),
            (4, vec![2, 1, 3, 1, 2, 8])
        );
        assert_eq!(
            compute_continued_fraction_sqrt(31),
            (5, vec![1, 1, 3, 5, 3, 1, 1, 10])
        );
    }

    #[test]
    fn test_convergents_from_cycle() {
        let (m, cycle) = compute_continued_fraction_sqrt(2);
        let convergents = Convergents::from_cycle(m, &cycle);
        let r = convergents
            .take(10)
            .map(|(n, d)| (n.to_u64().unwrap(), d.to_u64().unwrap()))
            .collect_vec();

        assert_eq!(
            r,
            vec![
                (1, 1),
                (3, 2),
                (7, 5),
                (17, 12),
                (41, 29),
                (99, 70),
                (239, 169),
                (577, 408),
                (1393, 985),
                (3363, 2378),
            ]
        );
    }

    #[test]
    fn test_convergents_from_iter() {
        let iter = Box::new((1u8..).flat_map(|k| [1, 2 * k, 1]));
        let convergents = Convergents::from_iter(2u8, iter);
        let r = convergents
            .take(10)
            .map(|(n, d)| (n.to_u64().unwrap(), d.to_u64().unwrap()))
            .collect_vec();

        assert_eq!(
            r,
            vec![
                (2, 1),
                (3, 1),
                (8, 3),
                (11, 4),
                (19, 7),
                (87, 32),
                (106, 39),
                (193, 71),
                (1264, 465),
                (1457, 536),
            ]
        )
    }
}

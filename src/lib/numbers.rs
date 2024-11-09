use num::integer::Roots;
use num::pow::Pow;

use crate::primes::{PrimeSet, Primes};

type Factors = Vec<(u64, usize)>;

pub fn factor(n: u64, primes: &mut Primes) -> Factors {
    let mut n = n;

    let mut factors = vec![];

    if n < 2 {
        return factors;
    }

    for prime in primes.iter() {
        let mut pow = 0;

        while n % prime == 0 {
            pow += 1;
            n /= prime;
        }

        if pow > 0 {
            factors.push((prime, pow));
        }

        if n == 1 {
            break;
        }
    }

    factors
}

pub fn divisors(n: u64) -> Vec<u64> {
    let mut divisors = vec![];

    for i in 1..=(n / 2) {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    divisors
}

pub fn digits<N>(n: N) -> Vec<u8>
where
    num::BigUint: From<N>,
{
    num::BigUint::from(n).to_radix_le(10)
}

pub fn n_digits<N>(n: N) -> u32
where
    num::BigUint: From<N>,
{
    digits(n).len() as u32
}

pub fn is_square<N: Roots + Pow<u8, Output = N>>(n: N) -> bool {
    n == n.sqrt().pow(2)
}

pub fn is_triangle_number(n: u64) -> bool {
    is_square(8 * n + 1)
}

pub fn is_pentagonal_number(n: u64) -> bool {
    let s = 24 * n + 1;
    is_square(s) && (1 + s.sqrt()) % 6 == 0
}

pub fn is_palindrome<N: ToString>(x: &N) -> bool {
    x.to_string().chars().rev().eq(x.to_string().chars())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primes::Primes;

    #[test]
    fn test_factor() {
        let mut primes = Primes::new();

        assert_eq!(factor(2, &mut primes), vec![(2, 1)]);
        assert_eq!(factor(4, &mut primes), vec![(2, 2)]);
        assert_eq!(factor(6, &mut primes), vec![(2, 1), (3, 1)]);
        assert_eq!(factor(12, &mut primes), vec![(2, 2), (3, 1)]);
        assert_eq!(factor(30, &mut primes), vec![(2, 1), (3, 1), (5, 1)]);
        assert_eq!(factor(36, &mut primes), vec![(2, 2), (3, 2)]);
        assert_eq!(factor(36, &mut primes), vec![(2, 2), (3, 2)]);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(2), vec![1]);
        assert_eq!(divisors(4), vec![1, 2]);
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6]);
        assert_eq!(divisors(27), vec![1, 3, 9]);
        assert_eq!(divisors(42), vec![1, 2, 3, 6, 7, 14, 21]);
    }

    #[test]
    fn test_digits() {
        assert_eq!(digits(4231u32), vec![1, 3, 2, 4]);
        assert_eq!(digits(31416u32), vec![6, 1, 4, 1, 3]);
    }

    #[test]
    fn test_is_square() {
        assert!(is_square(1));
        assert!(!is_square(2));
        assert!(!is_square(3));
        assert!(is_square(4));
        assert!(is_square(25));
        assert!(!is_square(37));
        assert!(!is_square(42));
    }
}

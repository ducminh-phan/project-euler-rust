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

pub fn factor_once(n: u64) -> Factors {
    factor(n, &mut Primes::new())
}

#[cfg(test)]
mod tests {
    use crate::primes::Primes;

    use super::*;

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
        assert_eq!(factor_once(36), vec![(2, 2), (3, 2)]);
    }
}

//! What is the largest prime factor of the number 600851475143?

/// Reduce a number n by a divisor d until n is not divisible by d
fn reduce_div(n: u64, d: u64) -> u64 {
    match n % d {
        0 => reduce_div(n / d, d),
        _ => n,
    }
}

/// Compute all prime factors of a given number
fn prime_factors(n: u64) -> Vec<u64> {
    fn _prime_factors(n: u64, d: u64) -> Vec<u64> {
        match n {
            1 => vec![],
            _ => match n % d {
                0 => {
                    let mut factors = _prime_factors(reduce_div(n, d), d);
                    factors.push(d);
                    factors
                }
                _ => _prime_factors(n, d + 1),
            },
        }
    }

    _prime_factors(n, 2)
}

pub fn solve() {
    let n = 600851475143;
    let factors = prime_factors(n);

    let result = factors[0];
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduce_div() {
        assert_eq!(reduce_div(4, 2), 1);
        assert_eq!(reduce_div(6, 2), 3);
        assert_eq!(reduce_div(9, 2), 9);
        assert_eq!(reduce_div(10, 2), 5);
    }

    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(4), vec![2]);
        assert_eq!(prime_factors(6), vec![3, 2]);
        assert_eq!(prime_factors(10), vec![5, 2]);
        assert_eq!(prime_factors(20), vec![5, 2]);
        assert_eq!(prime_factors(30), vec![5, 3, 2]);
    }
}

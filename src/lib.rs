#[link(name = "primecount", kind = "static")]
extern "C" {
    fn primecount_pi(x: i64) -> i64;
    fn primecount_phi(x: i64, a: i64) -> i64;
    fn primecount_nth_prime(n: i64) -> i64;
}

/// Count the number of primes <= x using Xavier Gourdon's
/// algorithm. Uses all CPU cores by default.
/// Returns -1 if an error occurs.
///
/// Run time: O(x^(2/3) / (log x)^2)
/// Memory usage: O(x^(1/3) * (log x)^3)
pub fn pi(x: i64) -> i64 {
    unsafe { primecount_pi(x) }
}

/// Partial sieve function (a.k.a. Legendre-sum).
/// phi(x, a) counts the numbers <= x that are not divisible
/// by any of the first a primes.
/// Returns -1 if an error occurs.
pub fn phi(x: i64, a: i64) -> i64 {
    unsafe { primecount_phi(x, a) }
}

/// Find the nth prime using a combination of the prime counting
/// function and the sieve of Eratosthenes.
/// @pre n <= 216289611853439384
/// Returns -1 if an error occurs.
///
/// Run time: O(x^(2/3) / (log x)^2)
/// Memory usage: O(x^(1/2))
pub fn nth_prime(n: i64) -> i64 {
    unsafe { primecount_nth_prime(n) }
}

// TODO(madiyar): Add i128 version of APIs.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primecount_pi() {
        assert_eq!(pi(20), 8);
    }

    #[test]
    fn test_primecount_phi() {
        assert_eq!(phi(10i64.pow(12), 78498i64), 37607833521)
    }

    #[test]
    fn test_primecount_nth_prime() {
        assert_eq!(nth_prime(455052511), 9999999967);
    }
}

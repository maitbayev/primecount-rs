#[link(name = "primecount", kind = "static")]
extern "C" {
    fn primecount_pi(x: i64) -> i64;
    fn primecount_phi(x: i64, a: i64) -> i64;
    fn primecount_nth_prime(n: i64) -> i64;
}

/// Counts the number of primes <= `x` using Xavier Gourdon's
/// algorithm. Uses all CPU cores by default.
///
/// Returns -1 if an error occurs.
///
/// # Examples
///
/// ```
/// // The primes are: 2, 3, 5 and 7.
/// assert_eq!(primecount::pi(10), 4);
///
/// // The primes are: 2, 3, 5, 7 and 11.
/// assert_eq!(primecount::pi(11), 5);
///
/// assert_eq!(primecount::pi(10i64.pow(12)), 37607912018);
/// ```
///
/// - Run time complexity: O(x^(2/3) / (log x)^2)
/// - Memory usage: O(x^(1/3) * (log x)^3)
pub fn pi(x: i64) -> i64 {
    unsafe { primecount_pi(x) }
}

/// Partial sieve function (a.k.a. Legendre-sum).
///
/// `phi(x, a)` counts the numbers <= `x` that are not divisible
/// by any of the first `a` primes.
///
/// Returns -1 if an error occurs.
///
/// # Examples
///
/// ```
/// // The numbers are: 1, 5 and 7.
/// assert_eq!(primecount::phi(10, 2), 3);
///
/// // The numbers are: 1, 7, 11 and 13.
/// assert_eq!(primecount::phi(15, 3), 4);
///
/// assert_eq!(primecount::phi(10i64.pow(12), 78498i64), 37607833521)
/// ```
pub fn phi(x: i64, a: i64) -> i64 {
    unsafe { primecount_phi(x, a) }
}

/// Find the `n`th prime using a combination of the prime counting
/// function and the sieve of Eratosthenes.
///
/// @pre n <= 216289611853439384
///
/// Returns -1 if an error occurs.
///
/// # Examples
///
/// ```
/// assert_eq!(primecount::nth_prime(1), 2);
/// assert_eq!(primecount::nth_prime(5), 11);
/// assert_eq!(primecount::nth_prime(455052511), 9999999967);
/// ```
///
/// - Run time: O(x^(2/3) / (log x)^2)
/// - Memory usage: O(x^(1/2))
pub fn nth_prime(n: i64) -> i64 {
    unsafe { primecount_nth_prime(n) }
}

// TODO(madiyar): Add i128 version of APIs.

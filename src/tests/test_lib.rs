//! Sub-module to test out the functions defined in the main library file.

use crate::get_hcn;
use crate::get_primes;
use crate::is_hcn;
use crate::is_prime;

use crate::libs::constants;

#[test]
pub fn test_is_prime() {
    //! Tests the check_if_prime function.
    let num: u64 = 7;
    let (check, factors) = is_prime(num);
    assert_eq!(check, true);
    assert_eq!(factors, vec![1, 7]);
}

#[test]
pub fn test_is_not_prime() {
    //! Tests the check_if_prime function with a non-prime number, 8.
    let num: u64 = 8;
    let (check, factors) = is_hcn(num);
    assert_eq!(check, false);
    assert_eq!(factors, vec![1, 2, 4, 8]);
}

#[test]
pub fn test_is_prime_2() {
    //! Tests the check_if_prime function with a prime number, 11.
    let num: u64 = 11;
    let (check, factors) = is_prime(num);
    assert_eq!(check, true);
    assert_eq!(factors, vec![1, 11]);
}

#[test]
pub fn test_composite_number() {
    //! Tests the check_if_prime function with a non-prime number, 10.

    let num: u64 = 10;
    let (check, factors) = is_prime(num);
    assert_eq!(check, false);
    assert_eq!(factors, vec![1, 2, 5, 10]);
}

#[test]
pub fn test_get_primes() {
    //! Tests the find_primes function with a non-prime number, 10.
    let num: u64 = 10;
    let primes = get_primes(num);
    assert_eq!(primes, vec![2, 3, 5, 7]);
}

#[test]
pub fn test_get_primes_2() {
    //! Tests the `get_primes` function.
    // As the `KNOWN_PRIMES` constant is defined in `src\libs\constants.rs` to contain all known prime numbers until 50, we can use it to test the `find_primes_till` function.
    let num: u64 = 50;
    let primes = get_primes(num);
    assert_eq!(primes, constants::KNOWN_PRIMES);
}

#[test]
pub fn test_is_hcn() {
    //! Tests the is_hcf function with a non-anti-prime number, 16.
    let num: u64 = 16;
    let (check, factors) = is_hcn(num);
    assert_eq!(check, false);
    assert_eq!(factors, vec![1, 2, 4, 8, 16]);
}

#[test]
pub fn test_is_hcn_2() {
    //! Tests the is_hcn function with an anti-prime number, 12.
    let num: u64 = 12;
    let (check, factors) = is_hcn(num);
    assert_eq!(check, true);
    assert_eq!(factors, vec![1, 2, 3, 4, 6, 12]);
}

#[test]
pub fn test_get_hcn() {
    //! Tests the get_hcn function.
    // As the `KNOWN_ANTIPRIMES` constant is defined in `src\libs\constants.rs` to contain all known anti-prime numbers until 10_080, we can use it to test the `get_hcn` function.
    let num: u64 = 10_080;
    let anti_primes = get_hcn(num);
    assert_eq!(anti_primes, constants::KNOWN_ANTIPRIMES);
}

#[test]
pub fn test_get_hcn_2() {
    //! Tests the get_hcn function with a non-anti-prime number, 16.
    let num: u64 = 16;
    let anti_primes = get_hcn(num);
    assert_eq!(anti_primes, vec![1, 2, 4, 6, 12]);
}

// prithoo: This test passes, but takes 64 seconds or so to run. So, it is ignored.
// if I let this run, pretty soon I will owe Github $1000 for exceeding the free tier.
/// Uncomment the `#[ignore]` attribute to run this test __IN LOCAL TESTING ONLY__ !
///
#[ignore]
#[test]
pub fn test_get_hcn_3() {
    //! Tests the get_hcn function with a non-anti-prime number, 16'540.
    let num: u64 = 16_540;
    let anti_primes = get_hcn(num);
    assert_eq!(
        anti_primes,
        vec![
            1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1_260, 1_680, 2_520,
            5_040, 7_560, 10_080, 15_120
        ]
    );
}

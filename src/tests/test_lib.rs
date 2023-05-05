//! Sub-module to test out the functions defined in the main library file.

use crate::check_if_prime;
use crate::check_if_anti_prime;
use crate::find_primes_till;

use crate::libs::constants;

#[test]
pub fn test_is_prime(){
    //! Tests the check_if_prime function.
    let num: u64 = 7;
    let (check, factors) = check_if_prime(num);
    assert_eq!(check, true);
    assert_eq!(factors, vec![1,7]);
}

#[test]
pub fn test_is_not_prime(){
    //! Tests the check_if_prime function with a non-prime number, 8.
    let num: u64 = 8;
    let (check, factors) = check_if_prime(num);
    assert_eq!(check, false);
    assert_eq!(factors, vec![1,2,4,8]);
}

#[test]
pub fn test_is_prime_2(){
    //! Tests the check_if_prime function with a prime number, 11.
    let num: u64 = 11;
    let (check, factors) = check_if_prime(num);
    assert_eq!(check, true);
    assert_eq!(factors, vec![1,11]);
}

#[test]
pub fn test_composite_number(){
    //! Tests the check_if_prime function with a non-prime number, 10.
    
    let num: u64 = 10;
    let (check, factors) = check_if_prime(num);
    assert_eq!(check, false);
    assert_eq!(factors, vec![1,2,5,10]);
}

#[test]
pub fn test_find_primes(){
    //! Tests the find_primes function with a non-prime number, 10.
    let num: u64 = 10;
    let primes = find_primes_till(num);
    assert_eq!(primes, vec![2,3,5,7]);
}

#[test]
pub fn test_find_primes_2(){
    //! Tests the find_primes function.
    // As the `KNOWN_PRIMES` constant is defined in `src\libs\constants.rs` to contain all known prime numbers until 50, we can use it to test the `find_primes_till` function.
    let num: u64 = 50;
    let primes = find_primes_till(num);
    assert_eq!(primes, constants::KNOWN_PRIMES);
}

#[test]
pub fn test_check_if_anti_prime(){
    //! Tests the check_if_anti_prime function with a non-anti-prime number, 16.
    let num: u64 = 16;
    let (check, factors) = check_if_anti_prime(num);
    assert_eq!(check, false);
    assert_eq!(factors, vec![1,2,4,8,16]);
}

#[test]
pub fn test_check_if_anti_prime_2(){
    //! Tests the check_if_anti_prime function with an anti-prime number, 12.
    let num: u64 = 12;
    let (check, factors) = check_if_anti_prime(num);
    assert_eq!(check, true);
    assert_eq!(factors, vec![1,2,3,4,6,12]);
}
//! Functions/methods to check prime numbers.
#![allow(warnings)]
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::libs::cache_map;
use crate::libs::constants;
use crate::libs::utils;

fn count_factors(num: u64) -> u64 {
    let mut factors: u64 = 2; // All natural numbers except 1 are divisible by 1 and themselves.
    let mut divisor: u64 = 2;
    if num == 0 {
        panic!("0 has infinite factors...");
    } else if num == 1 {
        // Static known case: 1 has the factor: [1]
        return 1;
    } else if num == 2 {
        // Static known case: 2 has factors: [1, 2]
        return 2;
    } else if num == 3 {
        // Static known case: 3 has factors: [1, 3]
        return 2;
    } else if num > 3 {
        while divisor < ((num / 2) + 1) as u64 {
            if num % divisor == 0 {
                factors = factors + 1;
            }
            divisor = divisor + 1;
        }
    } else {
        panic!(
            "`{num}` is not a valid unsigned, 64-bit integer.",
            num = num
        );
    }

    return factors;
}

pub fn check_if_prime(num: u64) -> (bool, Vec<u64>) {
    //! Checks to see if a given number is a prime number.
    let known_prime_numbers: [u64; constants::KNOWN_PRIMES.len()] = constants::KNOWN_PRIMES; //List of known prime number; reduces processing time.

    let upper_limit: u64 = (num / 2) as u64 + 1 as u64; //We do not need to check beyond the num/2 factor as it will be its highest possible factor.
    let mut divisor: u64 = 2; //Initialize the factor as 2.
    let mut flag: bool = true; //Default state of the flag: true-> isPrime; false-> notPrime.

    let mut factors: Vec<u64> = Vec::new(); //Initialize an empty list of factors.

    // If the given number is a known prime number as defined in crate::constants::KNOWN_PRIMES,
    // skip testing it and directly return (false, [1, num])
    if known_prime_numbers.contains(&num) {
        flag = true;
        factors.push(1);
        factors.push(num);
        return (flag, factors);
    }

    // Check all possible factors from 2 to num/2 and if a factor is found, add that value to the list of known factors.
    while divisor < upper_limit {
        if num % divisor == 0 {
            factors.push(divisor);
        }
        divisor = divisor + 1;
    }

    // If the list of known factors is not 0, then set flag as false.
    if factors.len() == 0 {
        factors.push(1);
        factors.push(num);

        factors = utils::unique_elements_vector(factors);
        factors.sort();
    } else if factors.len() > 0 {
        factors.push(1);
        factors.push(num);
        factors = utils::unique_elements_vector(factors);
        factors.sort();
        flag = false;
    }

    // Return flag and list of factors.
    return (flag, factors);
}

pub fn check_if_anti_prime(num: u64) -> (bool, Vec<u64>) {
    //! Checks to see if a given number is an anti-prime number.

    // This is the only function that currently requires a global variable or cache for its performance to be acceptable.

    let mut prime_check: bool = false; // Status check of the given number if it's a prime.
    let mut factors: Vec<u64> = Vec::new(); // Factors of the given number.

    if num == 0 || num == 1 || num == 2 {
        factors.push(num);
        if num > 1 {
            factors.push(1);
            factors.sort();
        }
        return (true, factors);
    }

    //// Check to see if this number is a prime number or not.
    (prime_check, factors) = check_if_prime(num);
    let n_factors: u64 = factors.len() as u64;
    if prime_check == true {
        return (false, factors);
    }

    let prev_start: u64 = 3;
    let mut n_previous_factors: u64 = 0 as u64; // Number of factors of a potential lower value natural number than `num`.
    let mut n_previous_highers: u64 = 0 as u64; // Number of lower numbers with a higher or equal number of factors than `num`

    // Loop to see if the number is just a composite number or an anti-prime number.
    // An anti-prime number is defined as a number which has more factors than any natural number lesser than itself.
    for item in prev_start..num {
        if cache_map::contains_key(&item) == false {
            n_previous_factors = count_factors(item);
            cache_map::insert(item, n_previous_factors);
            if n_previous_factors >= n_factors {
                n_previous_highers = n_previous_highers + 1;
            }
            continue;
        } else if cache_map::contains_key(&item) == true {
            n_previous_factors = cache_map::retrieve(&item);
            if n_previous_factors >= n_factors {
                n_previous_highers = n_previous_highers + 1;
            }
            continue;
        } else {
            continue;
        }
    }

    // In case this-> fn was called independently without the find_anti_primes_till() fn being available to clear the hashtable.
    cache_map::clean(); // Cleans the global hashtable if it gets too large.

    if n_previous_highers == 0 {
        return (true, factors);
    } else {
        return (false, factors);
    }
}

pub fn find_anti_primes_till(num: u64) -> Vec<u64> {
    //! Finds all the anti-prime numbers till a given number.
    let mut anti_primes: Vec<u64> = Vec::new();
    let knowns: [u64; constants::KNOWN_ANTIPRIMES.len()] = constants::KNOWN_ANTIPRIMES;
    let known_primes: Vec<u64> = find_primes_till(num);
    let mut result: bool; // True if an individual number is an anti-prime number, false if it is not.
    let mut _factors: Vec<u64> = Vec::new(); // List of factors of the number.

    // If the given number is less than or equal to the last known anti-prime number, then add all numbers less than or equal to it to the return vector.
    if num <= knowns[knowns.len() - 1] {
        for item in knowns {
            if item <= num {
                anti_primes.push(item);
            }
        }
    }
    // Else if the given number is greater than the last known anti-prime number, then add all known anti-prime numbers to the return vector.
    else if num > knowns[knowns.len() - 1] {
        for item in knowns {
            anti_primes.push(item);
        }
    } else {
        panic!("Something went wrong while checking for anti-primes.");
    }

    // Start checking for anti-prime numbers from the last known anti-prime number + 1.
    let start: u64 = knowns[knowns.len() - 1] + 1;

    //// Benchmarking code; comment out for prod.
    // let start_time = SystemTime::now();

    // IMPORTANT: DO NOT REMOVE THIS LINE OF CODE.
    // This line of code initializes the cache map before the anti-prime number check begins.
    // This is done to prevent the cache map from growing too large and consuming too much memory.
    cache_map::init(); // Initialize the cache map.
    for item in start..num + 1 {
        if known_primes.contains(&item) {
            continue;
        }

        //// Debug code; comment out for prod.
        // println!("Checking {}.........{}% done", item, ((item as f32/num as f32)*100.0) as u64);
        (result, _) = check_if_anti_prime(item);
        if result == true {
            anti_primes.push(item);
        } else {
            continue;
        }
    }

    // IMPORTANT: DO NOT REMOVE THIS LINE OF CODE.
    // This line of code clears the cache map after the anti-prime number check is complete.
    // This is done to prevent the cache map from growing too large and consuming too much memory.
    cache_map::clear(); // Clear the cache map.

    //// Benchmarking code; comment out for prod.
    // let end_time = SystemTime::now();
    // let time_taken = end_time.duration_since(start_time).unwrap();
    // println!("Time taken to check for anti-primes: {} seconds.", time_taken.as_secs());

    // Remove duplicate elements from the vector and sort it.
    anti_primes = utils::unique_elements_vector(anti_primes);
    anti_primes.sort();

    return anti_primes;
}

pub fn find_primes_till(num: u64) -> Vec<u64> {
    //! # __CURRENT VERSION__
    //! Find all prime numbers using a sieve.

    let mut j: u64; // Inner-loop counter for the seive.
    let knowns = constants::KNOWN_PRIMES; // What am I supposed to do with this?

    // Pre-declaration of Hashmap
    let mut checked_primes: HashMap<u64, bool> = HashMap::new();
    for i in 2..num + 1 {
        checked_primes.insert(i, true);
    }

    for i in 2..num + 1 {
        if (checked_primes[&i] == true) {
            j = i * 2;
            while (j <= num) {
                checked_primes.insert(j, false);
                j = j + i;
            }
        }
    }

    let mut results: Vec<u64> = Vec::new();
    for (key, value) in &checked_primes {
        if value == &true {
            results.push(*key);
        }
    }

    results = utils::unique_elements_vector(results);
    results.sort();
    return results;
}

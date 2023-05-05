//! Functions/methods to check prime numbers.
#![allow(warnings)]

use crate::libs::constants;
use crate::libs::utils;

pub fn check_if_prime(num: u64) -> (bool, Vec<u64>) {
    //! Checks to see if a given number is a prime number.
    let known_prime_numbers: [u64; constants::KNOWN_PRIMES.len()] = constants::KNOWN_PRIMES; //List of known prime number; reduces processing time.

    let upper_limit: u64 = (num / 2) + 1; //We do not need to check beyond the num/2 factor as it will be its highest possible factor.
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

        // To handle a unique case with 1.
        factors = utils::unique_elements_vector(factors);
        factors.sort();
        // println!("Factors of {num}: {factors:?}", num=num, factors=factors);
    } else if factors.len() > 0 {
        let mut new_factors: Vec<u64> = Vec::new();

        // Construct a new vector with 1 and num as the first and last elements respectively.
        new_factors.push(1);
        for item in factors {
            new_factors.push(item);
        }
        new_factors.push(num);

        // Update the list of factors to the new list of factors.
        factors = new_factors;

        //// Debugging code; comment out for prod.
        // let _debug_statement = format!("Factors of {num}: {factors:?}", num=num, factors=factors);
        // println!("{}", _debug_statement.italic().dimmed());
        flag = false;
    }

    // Return flag and list of factors.
    return (flag, factors);
}

pub fn check_if_anti_prime(num: u64)->(bool, Vec<u64>){
    //! Checks to see if a given number is an anti-prime number.

    let mut prime_check: bool=false;
    let mut factors: Vec<u64> = Vec::new();

    if num == 0 || num == 1 || num == 2{
        println!("While {} is TECHNICALLY an anti-prime number, it is also a prime number due to a special case.", num);
        return (false, factors);
    }

    // Check to see if this number is a prime number or not.
    (prime_check, factors) = check_if_prime(num);
    if prime_check==true{
        println!("{} is a COMPOSITE number.", num);
        return (false, factors);
    }

    let prev_start: u64 = 3;
    let mut previous_check: bool = false;
    let mut previous_factors: Vec<u64>;
    let mut previous_highers: Vec<u64> = Vec::new();

    // Loop to see if the number is just a composite number or an anti-prime number.
    // An anti-prime number is defined as a number which has more factors than any natural number lesser than itself.
    for item in prev_start..num {
        (previous_check, previous_factors) = check_if_prime(item);
        if previous_factors.len() >= factors.len() {
            previous_highers.push(item)
        }
    }
    //// Debugging code; comment out for prod.
    // println!("Lower numbers with a higher or equal number of factors:\t{:?}", previous_highers);
    if previous_highers.len() == 0{
        println!("{} is not just a composite number, it is an ANTI-PRIME number.", num);
        return (true, factors);
    }
    else {
        println!("{} is not an ANTI-PRIME number as {:?} have a higher or equal number of factors than it.", num, previous_highers);
        return (false, previous_highers);
    }

}
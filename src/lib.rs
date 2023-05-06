//! Rust library crate to hold simple functions to check the prime-ness of a given unsigned, 64-bit integer.

mod libs;

pub fn description(show:bool)->String{
    //! Prints a description of the crate to the console and returns the same.
    //! 
    //! __Arguments:__
    //! 
    //! 1. `show: bool` - Whether to print the description to the console or not.
    //! 
    //! __Returns__:
    //! 
    //! 1. `String` - A brief description of the library as defined in `libs::constants::DESCRIPTION`
    let description_str = String::from(libs::constants::DESCRIPTION);
    if show==true{
        println!("{}", description_str);
    }    
    return description_str;
}

pub fn check_if_prime(num:u64)->(bool, Vec<u64>){
    //! Checks to see if a given number is a prime number.
    //! 
    //! __Arguments:__
    //! 
    //! 1. `num: u64` - The number to check.
    //! 
    //! __Returns:__
    //! 
    //! 1. `bool` - Is true if the number is prime, and false if it is not.
    //! 2. `Vec<u64>` - The list of factors of the number.
    let check: bool;
    let factors: Vec<u64>;
    (check,factors) = libs::primes::check_if_prime(num);
    return (check,factors);
}

pub fn check_if_anti_prime(num:u64)->(bool, Vec<u64>){
    //! Checks to see if a given number is an anti-prime number.
    //! 
    //! __Arguments:__
    //!
    //! 1. `num: u64` - The number to check.
    //! 
    //! __Returns:__  
    //! 
    //! 1. `bool` - Is true if the number is anti-prime, and false if it is not.
    //! 2. `Vec<u64>` - The list of factors of the number.
    let check: bool;
    let factors: Vec<u64>;
    (check,factors) = libs::primes::check_if_anti_prime(num);
    return (check,factors);
}

pub fn find_anti_primes_till(num: u64)->Vec<u64>{
    //! Find all anti-prime numbers until a given value `num`.
    //! 
    //! __Arguments:__
    //! 
    //! 1. `num: u64` - The number to check till.
    //! 
    //! __Returns:__
    //! 
    //! 1. `Vec<u64>` - The list of all anti-prime numbers until that number.
    //! 
    //! __WARNING:__ Extremely Computationally Expensive [^1]
    //! 
    //! [^1]: _Takes ~415 seconds to check ~6000 values beyond the last known anti-prime as defined in `libs::constants::KNOWN_ANTIPRIMES`_
    let anti_primes: Vec<u64> = libs::primes::find_anti_primes_till(num);
    return anti_primes;
}

pub fn find_primes_till(num:u64)->Vec<u64>{
    //! Finds all the prime numbers till a given number.
    //! 
    //! __Arguments:__
    //! 
    //! 1. `num: u64` - The number to check till.
    //! 
    //! __Returns:__ 
    //! 
    //! 1. `Vec<u64>` - A vector of all the prime numbers till the given number.
    let prime_numbers: Vec<u64>;
    prime_numbers = libs::primes::find_primes_till(num);
    return prime_numbers;
}

#[cfg(test)]
mod tests;


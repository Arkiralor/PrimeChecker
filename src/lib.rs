//! Rust library crate to hold simple functions to check the prime-ness of a given unsigned, 64-bit integer.

mod libs;

pub fn description(show:bool)->String{
    //! Prints a description of the crate to the console and returns the same.
    let description_str = String::from(libs::constants::DESCRIPTION);
    if show==true{
        println!("{}", description_str);
        return description_str;
    }    
    return description_str;
}

pub fn check_if_prime(num:u64)->(bool, Vec<u64>){
    //! Checks to see if a given number is a prime number.
    //! * Arguments:    `num: u64` - The number to check.
    //! * Returns:  `(bool, Vec<u64>)` - Where bool is true if the number is prime, and false if it is not. The vector is a list of factors of the number.
    let check: bool;
    let factors: Vec<u64>;
    (check,factors) = libs::primes::check_if_prime(num);
    return (check,factors);
}

pub fn check_if_anti_prime(num:u64)->(bool, Vec<u64>){
    //! Checks to see if a given number is an anti-prime number.
    //! * Arguments:    `num: u64` - The number to check.
    //! * Returns:  `(bool, Vec<u64>)` - Where bool is true if the number is anti-prime, and false if it is not. The vector is a list of factors of the number.
    let check: bool;
    let factors: Vec<u64>;
    (check,factors) = libs::primes::check_if_anti_prime(num);
    return (check,factors);
}

pub fn find_primes_till(num:u64)->Vec<u64>{
    //! Finds all the prime numbers till a given number.
    //! * Arguments:    `num: u64` - The number to check till.
    //! * Returns:  `Vec<u64>` - A vector of all the prime numbers till the given number.
    let prime_numbers: Vec<u64>;
    prime_numbers = libs::primes::find_primes_till(num);
    return prime_numbers;
}

#[cfg(test)]
mod tests;


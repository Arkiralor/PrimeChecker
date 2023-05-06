//! Holds package-wide constants.
#![allow(warnings)]

// Declaring this as a string slice, otherwise the borrow-checker won't let it be declared.
// Stackoverflow link: https://stackoverflow.com/a/45176487/11745092
pub const DESCRIPTION: &str = "Rust library crate to hold simple functions to check the prime-ness of a given unsigned, 64-bit integer.";

// List of known prime number; reduces processing time.
pub const KNOWN_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]; 
// List of known anti-primes; reduces processing time.
pub const KNOWN_ANTIPRIMES: [u64; 21] = [1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360, 720, 840, 1_260, 1_680, 2_520, 5_040, 7_560, 10_080];
pub const MIN_I: u64 = u64::MIN; // Minimum value of u64
pub const MAX_I: u64 = u64::MAX; // Maximum value of u64

pub const MIN_RAND: u64 = MIN_I;
pub const MAX_RAND: u64 = 1_001;
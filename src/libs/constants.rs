//! Holds package-wide constants.
#![allow(warnings)]

pub const KNOWN_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]; //List of known prime number; reduces processing time.
pub const MIN_I: u64 = u64::MIN; // Minimum value of u64
pub const MAX_I: u64 = u64::MAX; // Maximum value of u64

pub const MIN_RAND: u64 = MIN_I;
pub const MAX_RAND: u64 = 1_001;
//! Holds package-wide constants.
#![allow(warnings)]

pub const KNOWN_PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]; //List of known prime number; reduces processing time.
pub const MIN_I: i128 = i128::MIN; // Minimum value of i128
pub const MAX_I: i128 = i128::MAX; // Maximum value of i128
pub const MAX_F: f64 = f64::MAX; // Maximum value of f64
pub const MIN_F: f64 = f64::MIN; // Minimum value of f64
pub const MAX_F32: f32 = f32::MAX; // Maximum value of f32
pub const MIN_F32: f32 = f32::MIN; // Minimum value of f32

pub const MIN_R: i128 = -1_000;
pub const MAX_R: i128 = 1_001;
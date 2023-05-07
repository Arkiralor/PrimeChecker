//! This module holds a global hashtable that is used to cache the values of the factors of a number.
//! 
//! It is used to reduce the processing time for finding anti-prime numbers until `n` by upto 86.26%.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// Define a type alias for our hashtable
type CheckedValuesFactors = HashMap<u64, u64>;

// Create a global RwLock object that holds our hashtable
lazy_static::lazy_static! {
    static ref GLOBAL_HASHTABLE: Arc<RwLock<CheckedValuesFactors>> = Arc::new(RwLock::new(CheckedValuesFactors::new()));
}

// Function to initiate the Hashtable
pub fn init() {
    // Get a write lock on the global hashtable
    let mut hashtable = GLOBAL_HASHTABLE.write().unwrap();

    // Insert the key-value pair into the hashtable
    hashtable.insert(1, 1);
    hashtable.insert(2, 2);
    hashtable.insert(3, 2);
}

// Function to get the keys from the GLOBAL_HASHTABLE
pub fn keys() -> Vec<u64> {
    // Get a read lock on the global hashtable
    let hashtable = GLOBAL_HASHTABLE.read().unwrap();

    // Get the keys from the hashtable
    let keys = hashtable.keys().cloned().collect();

    // Return the keys
    keys
}

// Function to insert a key-value pair into the hashtable
pub fn insert(key: u64, value: u64) {
    // Get a write lock on the global hashtable
    let mut hashtable = GLOBAL_HASHTABLE.write().unwrap();

    // Insert the key-value pair into the hashtable
    hashtable.insert(key, value);
}

// Function to retrieve a value from the hashtable given a key
pub fn retrieve(key: &u64) -> u64 {
    // Get a read lock on the global hashtable
    let hashtable = GLOBAL_HASHTABLE.read().unwrap();

    // Retrieve the value from the hashtable
    let value = hashtable.get(key).cloned();

    // Return the value
    value.unwrap()
}

// Function to check if a key exists in the hashtable
pub fn contains_key(key: &u64) -> bool {
    // Get a read lock on the global hashtable
    let hashtable = GLOBAL_HASHTABLE.read().unwrap();

    // Check if the key exists in the hashtable
    let contains_key = hashtable.contains_key(key);

    // Return the result
    contains_key
}

// Function to clear the hashtable
pub fn clear() {
    // Get a write lock on the global hashtable
    let mut hashtable = GLOBAL_HASHTABLE.write().unwrap();

    // Clear the hashtable
    hashtable.clear();
}


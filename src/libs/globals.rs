#![allow(warnings)]
//! Global mutable variables.

use std::collections::HashMap;

use std::sync::Mutex;

static mut CHECKED_VALUES: Vec<u64> = Vec::new(); // Holds checked values so that we con't compute the number of factors for the same number again and again.
// Hold the number of factors for the items in `checked_values` // (of course, there will be optimizations later.)
// static mut checked_values_map: HashMap<u64, u64> = HashMap::insert(&mut self, 1, 1); 
lazy_static! {
    static ref HASHMAP: Mutex<HashMap<u32, &'static str>> = {
        let mut checked_values_map = HashMap::new();
        Mutex::new(checked_values_map)
    };    
}

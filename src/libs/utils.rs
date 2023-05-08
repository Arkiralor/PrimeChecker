//! General purpose utility functions.

#![allow(warnings)]

use std::process::{Command, Stdio};

pub fn unique_elements_vector<T: std::fmt::Debug + std::cmp::PartialEq>(_list: Vec<T>) -> Vec<T> {
    //! Find all the UNIQUE elements in a given vector of Datatype T;
    //!
    //! _where T has the following attributes:_
    //!
    //! 1. Debug()
    //! 2. PartialEq()
    //!
    //!  __Arguments:__
    //!
    //!  1. _list: Vec<T>
    //!
    //! Returns:
    //!
    //! 1. Vec<T>

    //  __Returns:__
    //      Vec<T>
    let mut unique_list: Vec<T> = Vec::new();
    for item in _list {
        if !unique_list.contains(&item) {
            unique_list.push(item);
        }
    }
    return unique_list;
}

pub fn clear_console() {
    //! Clear the console before a new line is printed.
    println!("{}[2J", 27 as char);
}

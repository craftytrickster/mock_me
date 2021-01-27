// The Test Context Crate is private and should not be consumed by any external APIs.
// It exists solely as a way for the mock_me crate to store injected functions
// to replace the functions that the programmer has mocked.

// This needs to be in its own crate due to limitations in proc-macro crates.

#[macro_use]
extern crate lazy_static;

use std::ops::Drop;
use std::sync::Mutex;
use std::collections::HashMap;

lazy_static! {
    // Protected by Mutex since tests can be run in parallel
    // usize represents the pointer to the function that we are storing
    static ref GLOBAL_FUNCTION_LOOKUP: Mutex<HashMap<String, usize>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}

pub struct TextContext;

impl TextContext {
    pub fn set(&self, key: String, value: usize) {
        let mut lookup = GLOBAL_FUNCTION_LOOKUP.lock().unwrap();
        if lookup.contains_key(&*key) {
            panic!("You have already used the mocking key '{}', you cannot have duplicate keys!", key);
        }

        lookup.insert(key, value);
    }

    pub fn get(&self, key: &str) -> usize {
        let lookup = GLOBAL_FUNCTION_LOOKUP.lock().unwrap();
        match lookup.get(key) {
            Some(function_pointer) => *function_pointer,
            None => panic!("Unable to find a mocked function with the identifier: {}.", key)
        }
    }
}

impl Drop for TextContext {
    fn drop(&mut self) {
        let mut lookup = GLOBAL_FUNCTION_LOOKUP.lock().unwrap();
        lookup.clear();
    }
}


pub fn get_test_context() -> TextContext {
    TextContext
}
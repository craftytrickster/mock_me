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
        lookup.insert(key, value);
    }

    pub fn get(&self, key: &str) -> usize {
        let lookup   = GLOBAL_FUNCTION_LOOKUP.lock().unwrap();
        *lookup.get(key).unwrap()
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
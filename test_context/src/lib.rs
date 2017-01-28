#[macro_use]
extern crate lazy_static;

use std::ops::Drop;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::any::Any;

lazy_static! {
    // Protected by Mutex since tests can be run in parallel
    static ref GLOBAL_FUNCTION_LOOKUP: Mutex<HashMap<String, Arc<Any + Send + Sync>>> = {
        let m = HashMap::new();

        Mutex::new(m)
    };
}

pub struct TextContext;

impl TextContext {
    pub fn set(&self, key: String, value: fn(f64) -> String) {
        let mut lookup = GLOBAL_FUNCTION_LOOKUP.lock().unwrap();
        lookup.insert(key, Arc::new(value) as Arc<Any + Send + Sync>);
    }

    pub fn get(&self, key: &str) -> Arc<Any + Send + Sync> {
        let lookup   = GLOBAL_FUNCTION_LOOKUP.lock().unwrap();
        lookup.get(key).unwrap().clone()
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
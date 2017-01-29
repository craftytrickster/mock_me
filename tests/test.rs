#![feature(proc_macro)]
extern crate mock_me;
use mock_me::{mock, inject};

#[mock(id_1="external_db_call: fn(u32) -> String", id_2="other_call: fn() -> String")]
fn my_super_cool_function() -> String {
    let input = 42u32;
    // external_db_call will be replaced with fake_mocked_call during testing
    let db_result = external_db_call(input);

    // other_call will also be replaced
    let other_result = other_call();
    format!("I have two results! {} and {}", db_result, other_result)
}

#[test]
#[inject(id_1="db_fake", id_2="other_fake")]
fn actual_test2() {
    let result = my_super_cool_function();
    assert_eq!(result, "I have two results! Faker! and This is indeed a disturbing universe.");
}

fn db_fake(_: u32) -> String { "Faker!".to_string() }
fn other_fake() -> String { "This is indeed a disturbing universe.".to_string() }



#[mock(fun="silly_func: fn(f64, f64) -> f64")]
fn function_with_fun_id() -> f64 {
    silly_func(30f64, 20f64)
}

#[allow(dead_code)]
fn silly_func(num_1: f64, num_2: f64) -> f64 {
    num_1 + num_2
}

#[test]
#[inject(fun="replacement_1")]
fn test_with_silly_func_1() {
    let result = function_with_fun_id();
    assert_eq!(result, 10f64);
}

fn replacement_1(num_1: f64, num_2: f64) -> f64 {
    num_1 - num_2
}

#[test]
#[inject(fun="replacement_2")]
fn test_with_silly_func_2() {
    let result = function_with_fun_id();
    assert_eq!(result, 600f64);
}

fn replacement_2(num_1: f64, num_2: f64) -> f64 {
    num_1 * num_2
}

#![feature(proc_macro)]
extern crate mockme;
use mockme::{mock, inject};

//#[test]
fn function_call_without_mock() {
    let result = concrete_process_payment(50000f64, 100u32);
    assert_eq!(result, "Concrete payment of 50000 just processed");
}

//#[test]
//#[inject(mock_method_id_1, fake_process_payment)]
fn actual_test() {
    let result = function_call_with_mock(50000f64);
    assert_eq!(result, "No processing occurred, just testing 50000");
}


#[test]
#[inject(mock_method_id_1="duh")]
fn actual_test2() {
    let result = function_call_with_mock(50000f64);
    assert_eq!(result, "stupid100");
}

fn duh(_: f64, v: u32) -> String { format!("stupid{}99", v) }
fn fake_multiple(value: f64) -> f64 { 12340f64 }
/*
#[mock(
    fun_crazy_mock(concrete_process_payment="fn(f64) -> String"),
    boring_crazy_mock(concrete_process_payment="fn(f64) -> String")
)]
*/

#[mock(
    mock_method_id_1="concrete_process_payment: fn(f64, u32) -> String"
)]
fn function_call_with_mock(value: f64) -> String {
    let second_value = multiple(100f64);
    concrete_process_payment(value, second_value as u32)
}

fn multiple(value: f64) -> f64 {
    value * 52f64
}

fn concrete_process_payment(amount: f64, v: u32) -> String {
    format!("Concrete payment of {} just processed", amount + v as f64)
}

fn fake_process_payment(amount: f64) -> String {
    format!("No processing occurred, just testing {}", amount)
}
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
    assert_eq!(result, "stupid");
}

fn duh(_: f64, v: u32) -> String { format!("stupid{}", v) }

/*
#[mock(
    fun_crazy_mock(concrete_process_payment="fn(f64) -> String"),
    boring_crazy_mock(concrete_process_payment="fn(f64) -> String")
)]
*/

#[mock(mock_method_id_1="concrete_process_payment: fn(f64, u32) -> String")]
fn function_call_with_mock(value: f64) -> String {
    concrete_process_payment(value, 100u32)
}


fn concrete_process_payment(amount: f64, v: u32) -> String {
    format!("Concrete payment of {} just processed", amount + v as f64)
}

fn fake_process_payment(amount: f64) -> String {
    format!("No processing occurred, just testing {}", amount)
}
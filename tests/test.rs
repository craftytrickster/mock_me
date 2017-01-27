#![feature(proc_macro)]
extern crate mockme;
use mockme::{mock, inject};

#[test]
fn function_call_without_mock() {
    let result = concrete_process_payment(50000f64);
    assert_eq!(result, "Concrete payment of 50000 just processed");
}

#[test]
#[inject(mock_method_id_1, fake_process_payment)]
fn actual_test() {
    let result = function_call_with_mock(50000f64);
    assert_eq!(result, "No processing occurred, just testing 50000");
}


#[test]
#[inject(mock_method_id_1, duh)]
fn actual_test2() {
    let result = function_call_with_mock(50000f64);
    assert_eq!(result, "stupid");
}

fn duh(_: f64) -> String { String::from("stupid") }



#[mock(concrete_process_payment, mock_method_id_1)]
fn function_call_with_mock(value: f64) -> String {
    concrete_process_payment(value)
}


fn concrete_process_payment(amount: f64) -> String {
    format!("Concrete payment of {} just processed", amount)
}

fn fake_process_payment(amount: f64) -> String {
    format!("No processing occurred, just testing {}", amount)
}
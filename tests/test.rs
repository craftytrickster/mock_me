#![feature(proc_macro)]
extern crate mockme;
use mockme::mock;

#[test]
fn function_call_without_mock() {
    let result = concrete_process_payment(1000, 50000f64).unwrap();
    assert_eq!(result, "Concrete payment of 50000 just processed for account: 1000");
}

#[test]
#[mock(concrete_process_payment, fake_process_payment)]
fn function_call_with_mock() {
    let result = concrete_process_payment(1000, 50000f64).unwrap();
    assert_eq!(result, "No processing occurred, just testing 50000 for account: 1000");
}


fn concrete_process_payment(account_number: u64, amount: f64) -> Result<String, ()> {
    Ok(format!("Concrete payment of {} just processed for account: {}", amount, account_number))
}

fn fake_process_payment(account_number: u64, amount: f64) -> Result<String, ()> {
    Ok(format!("No processing occurred, just testing {} for account: {}", amount, account_number))
}
//! MockMe is a tool used to mock dependencies / function calls when running unit (lib) tests in Rust.
//!
//! ## How to Use
//!
//! Simply use the macro as seen in the example below.
//! When this code is run normally, MockMe will have no effect.
//! However, when the code is run as part of a unit test #[cfg(test)],
//! the mocked token will be used instead.
//!
//!
//! ```rust,ignore
//!
//! #![feature(proc_macro)]
//! extern crate mockme;
//! use mockme::mock;
//!
//! #[mock(external_db_call, fake_mocked_call)]
//! fn my_super_cool_function() {
//!     let input = 42;
//!     // external_db_call will be replaced with fake_mocked_call during testing
//!     let result = external_db_call(input);
//!     println!("{}", result);
//! }
//!
//! ```

#![feature(proc_macro)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn mock(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (concrete, test) = get_concrete_and_test_names(attr);

    let source = item.to_string();
    let modified_source = source.replace(&*concrete, &*test);

    let branched_source = format!(
        r#"
        #[cfg(not(test))]
        {}

        #[cfg(test)]
        {}
        "#, source, modified_source
    );

    branched_source.parse().unwrap()
}

// FIXME: This should be done in a more structured way
// Validation on value types might be a good idea
fn get_concrete_and_test_names(attr: TokenStream) -> (String, String) {
    let attr_str = attr.to_string();
    let pair = &attr_str[1..attr_str.len() - 1].replace(" ", "");

    let concrete_test_vec: Vec<_> = pair.split(",").collect();
    let items_specified = concrete_test_vec.len();
    assert_eq!(items_specified, 2,
        "There should be two items specified for the mock, you have specified: {}", items_specified
    );

    (concrete_test_vec[0].to_string(), concrete_test_vec[1].to_string())
}
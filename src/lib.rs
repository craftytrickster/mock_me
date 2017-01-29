//! MockMe is a tool used to mock dependencies / function calls when running unit (lib) tests in Rust.
//!
//! ## How to Use
//!
//! Simply use the macro as seen in the example below.
//! When this code is run normally, MockMe will have no effect.
//! However, when the code is run as part of a unit test #[cfg(test)],
//! the mocked token will be used instead.
//!
//! In order to use, we first must mark the functions that we would like to mock.
//! When running tests, we then identify the replacement function that we would like to inject.
//!
//! ```rust,ignore
//!
//! #![feature(proc_macro)]
//! extern crate mock_me;
//! use mock_me::{mock, inject};
//!
//! // Below we will create two mocking identifiers called id_1 and id_2.
//! // We will then provide the name of the two functions we are mocking, as well as
//! // their type signature. In future iterations, hopefully the signature won't be needed.
//! #[mock(id_1="external_db_call: fn(u32) -> String", id_2="other_call: fn() -> String")]
//! fn my_super_cool_function() -> String {
//!     let input = 42u32;
//!     // external_db_call will be replaced with fake_mocked_call during testing
//!     let db_result = external_db_call(input);
//!
//!     // other_call will also be replaced
//!     let other_result = other_call();
//!     format!("I have two results! {} and {}", db_result, other_result)
//! }
//!
//! // Finally, when we run our tests, we simply need to provide the identifier we previously used,
//! // as well as the name of the replacement function
//! #[test]
//! #[inject(id_1="db_fake", id_2="other_fake")]
//! fn actual_test2() {
//!     let result = my_super_cool_function();
//!     assert_eq!(result, "I have two results! Faker! and This is indeed a disturbing universe.");
//! }
//!
//! fn db_fake(_: u32) -> String { "Faker!".to_string() }
//! fn other_fake() -> String { "This is indeed a disturbing universe.".to_string() }
//!
//! ```

#![feature(proc_macro)]
#![feature(insert_str)]
extern crate proc_macro;
use proc_macro::TokenStream;

extern crate mock_me_test_context;

use std::fmt::Write;

mod macro_helper;
use macro_helper::*;

/// The mock macro is used mock a concrete function that is not desired during unit tests.
/// Its signature contains the identifier that is being mocked, with the function that will replace
/// the mocked function within quotes, as well as the mocked function signature.
#[proc_macro_attribute]
pub fn mock(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mock_matches = get_mock_matches(attr);

    let mut source = item.to_string();

    // I should find a more structured way of injecting test context into top of method
    let insertion_point = source.find("{").unwrap() + 1;
    source.insert_str(insertion_point, HEADER);

    let mut modified_source = source.clone();
    for m_match in mock_matches {
        let ctx_getter = format!(r#"
            (unsafe {{
                let _mock_me_test_usize_func = _mock_me_test_context_instance.get("{}");
                let _mock_me_test_transmuted_func: {} = std::mem::transmute(_mock_me_test_usize_func);
                _mock_me_test_transmuted_func
            }})
        "#, m_match.identifier, m_match.function_signature);

        // string replacement should be more controlled ideally than a blind replace
        modified_source = modified_source.replace(&*m_match.function_to_mock, &*ctx_getter);
    }


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

/// The inject macro is used to replace a mocked function with an alternative implementation.
/// Its signature contains the identifier that is being mocked, with the function that will replace
/// the mocked function within quotes.
#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    let inject_matches = get_inject_matches(attr);

    let mut source = item.to_string();

    let mut context_setter_string = HEADER.to_string();

    for i_match in inject_matches {
        write!(
            context_setter_string,
            "_mock_me_test_context_instance.set(\"{}\".to_string(), {} as usize);\n",
            i_match.identifier, i_match.function_to_mock
        ).unwrap();
    }

    // I should find a more structured way of injecting test context into top of method
    let insertion_point = source.find("{").unwrap() + 1;
    source.insert_str(insertion_point, &*context_setter_string);

    source.parse().unwrap()
}
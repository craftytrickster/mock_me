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
#![feature(insert_str)]

extern crate proc_macro;
use proc_macro::TokenStream;

extern crate mock_me_test_context;
use std::fmt::Write;

const HEADER: &'static str = r#"
    extern crate mock_me_test_context;
    let _mock_me_test_context_instance = mock_me_test_context::get_test_context();
"#;

#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    let inject_matches = get_inject_matches(attr);

    let mut source = item.to_string();

    let mut context_setter_string = HEADER.to_string();

    for i_match in inject_matches {
        write!(
            context_setter_string,
            "_mock_me_test_context_instance.set(\"{}\".to_string(), {});\n",
            i_match.identifier, i_match.function_to_mock
        ).unwrap();
    }

    // I should find a more structured way of injecting test context into top of method
    let insertion_point = source.find("{").unwrap() + 1;
    source.insert_str(insertion_point, &*context_setter_string);

    source.parse().unwrap()
}


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
            let _mock_me_test_casted_func = _mock_me_test_context_instance.get("{}").clone();
            let _mock_me_test_casted_ref = _mock_me_test_casted_func.as_ref() as &std::any::Any;

            _mock_me_test_casted_ref.downcast_ref::<{}>().unwrap()
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

#[derive(Debug)]
struct InjectMatch {
    identifier: String,
    function_to_mock: String,
}

#[derive(Debug)]
struct MockMatch {
    identifier: String,
    function_to_mock: String,
    function_signature: String
}

fn get_inject_matches(attr: TokenStream) -> Vec<InjectMatch> {
    let attr_str = attr.to_string();
    let without_parens = &attr_str[1..attr_str.len() - 1];

    let mut result = vec![];

    for part in without_parens.split("\" ,") {
        let sub_parts: Vec<_> = part.split("=").collect();
        let identifier = sub_parts[0].trim().to_string();
        let function_to_mock = sub_parts[1].trim().replace("\"", "");

        result.push(InjectMatch { identifier: identifier, function_to_mock: function_to_mock });
    }

    result
}

fn get_mock_matches(attr: TokenStream) -> Vec<MockMatch> {
    let attr_str = attr.to_string();
    let without_parens = &attr_str[1..attr_str.len() - 1];

    let mut result = vec![];

    for part in without_parens.split("\" ,") {
        let sub_parts: Vec<_> = part.split("=").collect();
        let identifier = sub_parts[0].trim().to_string();

        let function_portions: Vec<_> = sub_parts[1].split(":").collect();
        let function_to_mock = function_portions[0].trim().to_string().replace("\"", "");
        let function_signature = function_portions[1].trim().replace("\"", "");

        result.push(MockMatch {
            identifier: identifier,
            function_to_mock: function_to_mock,
            function_signature: function_signature
        });
    }

    result
}
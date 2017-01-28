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

extern crate syn;
extern crate test_context;

#[proc_macro_attribute]
pub fn inject(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (mock_id, method_to_inject) = get_attr_tuple(attr);
    let mut source = item.to_string();

    let context_setter_string = format!(r#"
        extern crate test_context;
        let ctx = test_context::get_test_context();
        ctx.set("{}".to_string(), {});
    "#, mock_id, method_to_inject);

    // I should find a more structured way of injecting test context into top of method
    let insertion_point = source.find("{").unwrap() + 1;
    source.insert_str(insertion_point, &*context_setter_string);

    source.parse().unwrap()
}


#[proc_macro_attribute]
pub fn mock(attr: TokenStream, item: TokenStream) -> TokenStream {
    let (concrete, mock_id) = get_attr_tuple(attr);

    let source = item.to_string();
    let ctx_getter = format!(r#"
        extern crate test_context;
        use std::any::Any;

        let ctx = test_context::get_test_context();
        let casted_func = ctx.get("{}").clone();
        let casted_ref = casted_func.as_ref() as &Any;

        casted_ref.downcast_ref::<fn(f64) -> String>().unwrap()
    "#, mock_id);

    // string replacement should be more controlled ideally than a blind replace
    let modified_source = source.replace(&*concrete, &*ctx_getter);

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
fn get_attr_tuple(attr: TokenStream) -> (String, String) {
    let attr_str = attr.to_string();
    let pair = &attr_str[1..attr_str.len() - 1].replace(" ", "");

    let concrete_test_vec: Vec<_> = pair.split(",").collect();
    let items_specified = concrete_test_vec.len();
    assert_eq!(items_specified, 2,
        "There should be two items specified for the mock, you have specified: {}", items_specified
    );

    (concrete_test_vec[0].to_string(), concrete_test_vec[1].to_string())
}
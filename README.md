MockMe
======

MockMe is a tool used to mock dependencies / function calls when running unit (lib) tests in Rust.

## How to Use

Simply use the macro as seen in the example below.
When this code is run normally, MockMe will have no effect.
However, when the code is run as part of a unit test `#[cfg(test)]`,
the mocked token will be used instead.

```rust

 #![feature(proc_macro)]
 extern crate mockme;
 use mockme::mock;

 // Below we will create two mocking identifiers called id_1 and id_2.
 // We will then provide the name of the two functions we are mocking, as well as
 // their type signature. In future iterations, hopefully the signature won't be needed.
#[mock(id_1="external_db_call: fn(u32) -> String", id_2="other_call: fn() -> String")]
fn my_super_cool_function() -> String {
    let input = 42u32;
    // external_db_call will be replaced with fake_mocked_call during testing
    let db_result = external_db_call(input);

    // other_call will also be replaced
    let other_result = other_call();
    format!("I have two results! {} and {}", db_result, other_result)
}

 // Finally, when we run our tests, we simply need to provide the identifier we previously used,
 // as well as the name of the replacement function
#[test]
#[inject(id_1="db_fake", id_2="other_fake")]
fn actual_test2() {
    let result = my_super_cool_function();
    assert_eq!(result, "I have two results! Faker! and This is indeed a disturbing universe.");
}

fn db_fake(_: u32) -> String { "Faker!".to_string() }
fn other_fake() -> String { "This is indeed a disturbing universe.".to_string() }
```

## Contributions

All contributions are welcome! This library is still in its infancy, so everything helps.
Code contributions, feature requests and bug reports are all appreciated.

## Limitations

Currently, the library is unable to infer the signature of the function that is being mocked. As a result,
the programmer needs to provide it, which hurts the ergonomics of the library.
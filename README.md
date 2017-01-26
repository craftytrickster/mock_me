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

#[mock(external_db_call, fake_mocked_call)]
fn my_super_cool_function() {
    let input = 42;
    // external_db_call will be replaced with fake_mocked_call during testing
    let result = external_db_call(input);
    println!("{}", result);
}
```

This library is still in its infancy and is considered experimental.
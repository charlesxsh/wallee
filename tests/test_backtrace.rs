#![allow(clippy::let_underscore_untyped)]

#[rustversion::not(nightly)]
#[ignore]
#[test]
fn test_backtrace() {}

#[rustversion::nightly]
#[test]
fn test_backtrace() {
    use wallee::wallee;

    let error = wallee!("oh no!");
    let _ = error.backtrace();
}

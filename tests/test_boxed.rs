#![allow(
    // Clippy bug: https://github.com/rust-lang/rust-clippy/issues/7422
    clippy::nonstandard_macro_braces,
)]

use std::error::Error as StdError;
use std::io;
use thiserror::Error;
use wallee::wallee;

#[derive(Error, Debug)]
#[error("outer")]
struct MyError {
    source: io::Error,
}

#[test]
fn test_boxed_str() {
    let error = Box::<dyn StdError + Send + Sync>::from("oh no!");
    let error = wallee!(error);
    assert_eq!("oh no!", error.to_string());
    assert_eq!(
        "oh no!",
        error
            .downcast_ref::<Box<dyn StdError + Send + Sync>>()
            .unwrap()
            .to_string()
    );
}

#[test]
fn test_boxed_thiserror() {
    let error = MyError {
        source: io::Error::new(io::ErrorKind::Other, "oh no!"),
    };
    let error = wallee!(error);
    assert_eq!("oh no!", error.source().unwrap().to_string());
}

#[test]
fn test_boxed_wallee() {
    let error = wallee!("oh no!").context("it failed");
    let error = wallee!(error);
    assert_eq!("oh no!", error.source().unwrap().to_string());
}

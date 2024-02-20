// #![allow(
//     clippy::assertions_on_result_states,
//     clippy::eq_op,
//     clippy::incompatible_msrv, // https://github.com/rust-lang/rust-clippy/issues/12257
//     clippy::items_after_statements,
//     clippy::needless_pass_by_value,
//     clippy::shadow_unrelated,
//     clippy::wildcard_imports
// )]

mod common;

use self::common::*;
use std::cell::Cell;
use std::future;
use wallee::{ensure, wallee};

#[test]
fn test_messages() {
    assert_eq!("oh no!", bail_literal().unwrap_err().to_string());
    assert_eq!("oh no!", bail_fmt().unwrap_err().to_string());
    assert_eq!("oh no!", bail_error().unwrap_err().to_string());
}

#[test]
fn test_ensure() {
    let f = || {
        ensure!(1 + 1 == 2, "This is correct");
        Ok(())
    };
    assert!(f().is_ok());

    let v = 1;
    let f = || {
        ensure!(v + v == 2, "This is correct, v: {}", v);
        Ok(())
    };
    assert!(f().is_ok());

    let f = || {
        ensure!(v + v == 1, "This is not correct, v: {}", v);
        Ok(())
    };
    assert!(f().is_err());

    let f = || {
        ensure!(v + v == 1);
        Ok(())
    };
    assert_eq!(
        f().unwrap_err().to_string(),
        "Condition failed: `v + v == 1` (2 vs 1)",
    );
}

#[test]
fn test_temporaries() {
    fn require_send_sync(_: impl Send + Sync) {}

    require_send_sync(async {
        // If wallee hasn't dropped any temporary format_args it creates by the
        // time it's done evaluating, those will stick around until the
        // semicolon, which is on the other side of the await point, making the
        // enclosing future non-Send.
        future::ready(wallee!("...")).await;
    });

    fn message(cell: Cell<&str>) -> &str {
        cell.get()
    }

    require_send_sync(async {
        future::ready(wallee!(message(Cell::new("...")))).await;
    });
}

#[test]
fn test_brace_escape() {
    let err = wallee!("unterminated ${{..}} expression");
    assert_eq!("unterminated ${..} expression", err.to_string());
}

#[test]
fn test_wallee_macro() {
    use std::io;

    let err = wallee!("oh no!");
    let s = format!("{err:#}");
    assert_eq!(s, "oh no!");

    let x = 33;
    let err = wallee!("oh no! {x}");
    let s = format!("{err:#}");
    assert_eq!(s, "oh no! 33");

    let err = wallee!("oh no!: {}", 33);
    let s = format!("{err:#}");
    assert_eq!(s, "oh no!: 33");

    let err = wallee!(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!"));
    let s = format!("{err:#}");
    assert_eq!(s, "oh no!");

    let err = wallee!(wallee!("oh no!")).context("io failed");
    let s = format!("{err:#}");
    assert_eq!(s, "io failed: oh no!");

    let err =
        wallee!(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!")).context("io failed");
    let s = format!("{err:#}");
    assert_eq!(s, "io failed: oh no!");

    let x = 33;
    let err = wallee!(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!"))
        .context(format!("io failed {x}"));
    let s = format!("{err:#}");
    assert_eq!(s, "io failed 33: oh no!");

    let err = wallee!(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!"))
        .context(format!("io failed: {}", 33));
    let s = format!("{err:#}");
    assert_eq!(s, "io failed: 33: oh no!");

    let err = wallee!(wallee!("oh no!")).context(format!("io failed: {}", 33));
    let s = format!("{err:#}");
    assert_eq!(s, "io failed: 33: oh no!");
}

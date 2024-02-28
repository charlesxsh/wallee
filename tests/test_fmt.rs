use std::io;
use wallee::{bail, Context, Result};

fn f() -> Result<()> {
    bail!(io::Error::new(io::ErrorKind::PermissionDenied, "oh no!"));
}

fn g() -> Result<()> {
    f().context("f failed")
}

fn h() -> Result<()> {
    g().context("g failed")
}

const EXPECTED_ALTDISPLAY_F: &str = "oh no!";

const EXPECTED_ALTDISPLAY_G: &str = "f failed: oh no!";

const EXPECTED_ALTDISPLAY_H: &str = "g failed: f failed: oh no!";

const EXPECTED_DEBUG_F: &str = "tests/test_fmt.rs(5:5): oh no!";

const EXPECTED_DEBUG_G: &str = "\
tests/test_fmt.rs(9:9): f failed

Caused by:
    oh no!";

const EXPECTED_DEBUG_H: &str = "\
tests/test_fmt.rs(13:9): g failed

Caused by:
    0: f failed
    1: oh no!";

const EXPECTED_ALTDEBUG_F: &str = "\
Wallee {
    file: \"tests/test_fmt.rs\",
    line: 5,
    column: 5,
    error: Custom {
        kind: PermissionDenied,
        error: \"oh no!\",
    },
}";

const EXPECTED_ALTDEBUG_G: &str = "\
Wallee {
    file: \"tests/test_fmt.rs\",
    line: 9,
    column: 9,
    error: Error {
        context: \"f failed\",
        source: Wallee {
            file: \"tests/test_fmt.rs\",
            line: 5,
            column: 5,
            error: Custom {
                kind: PermissionDenied,
                error: \"oh no!\",
            },
        },
    },
}";

const EXPECTED_ALTDEBUG_H: &str = "\
Wallee {
    file: \"tests/test_fmt.rs\",
    line: 13,
    column: 9,
    error: Error {
        context: \"g failed\",
        source: Wallee {
            file: \"tests/test_fmt.rs\",
            line: 9,
            column: 9,
            error: Error {
                context: \"f failed\",
                source: Wallee {
                    file: \"tests/test_fmt.rs\",
                    line: 5,
                    column: 5,
                    error: Custom {
                        kind: PermissionDenied,
                        error: \"oh no!\",
                    },
                },
            },
        },
    },
}";

#[test]
fn test_display() {
    assert_eq!("g failed", h().unwrap_err().to_string());
}

#[test]
fn test_altdisplay() {
    assert_eq!(EXPECTED_ALTDISPLAY_F, format!("{:#}", f().unwrap_err()));
    assert_eq!(EXPECTED_ALTDISPLAY_G, format!("{:#}", g().unwrap_err()));
    assert_eq!(EXPECTED_ALTDISPLAY_H, format!("{:#}", h().unwrap_err()));
}

#[test]
fn test_debug() {
    assert_eq!(
        EXPECTED_DEBUG_F,
        &format!("{:?}", f().unwrap_err())[..EXPECTED_DEBUG_F.len()]
    );
    assert_eq!(
        EXPECTED_DEBUG_G,
        &format!("{:?}", g().unwrap_err())[..EXPECTED_DEBUG_G.len()]
    );
    assert_eq!(
        EXPECTED_DEBUG_H,
        &format!("{:?}", h().unwrap_err())[..EXPECTED_DEBUG_H.len()]
    );
}

#[test]
fn test_altdebug() {
    assert_eq!(EXPECTED_ALTDEBUG_F, format!("{:#?}", f().unwrap_err()));
    assert_eq!(EXPECTED_ALTDEBUG_G, format!("{:#?}", g().unwrap_err()));
    assert_eq!(EXPECTED_ALTDEBUG_H, format!("{:#?}", h().unwrap_err()));
}

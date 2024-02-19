/// Return early with an error.
///
/// This macro is equivalent to `return Err(`[`wallee!($args...)`][wallee!]`)`.
///
/// The surrounding function's or closure's return value is required to be
/// `Result<_,`[`wallee::Error`][crate::Error]`>`.
///
/// [wallee!]: crate::wallee
///
/// # Example
///
/// ```
/// # use wallee::{bail, Result};
/// #
/// # fn has_permission(user: usize, resource: usize) -> bool {
/// #     true
/// # }
/// #
/// # fn main() -> Result<()> {
/// #     let user = 0;
/// #     let resource = 0;
/// #
/// if !has_permission(user, resource) {
///     bail!("permission denied for accessing {}", resource);
/// }
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use wallee::{bail, Result};
/// # use thiserror::Error;
/// #
/// # const MAX_DEPTH: usize = 1;
/// #
/// #[derive(Error, Debug)]
/// enum ScienceError {
///     #[error("recursion limit exceeded")]
///     RecursionLimitExceeded,
///     # #[error("...")]
///     # More = (stringify! {
///     ...
///     # }, 1).1,
/// }
///
/// # fn main() -> Result<()> {
/// #     let depth = 0;
/// #
/// if depth > MAX_DEPTH {
///     bail!(ScienceError::RecursionLimitExceeded);
/// }
/// #     Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => {
        return $crate::__private::Err($crate::__wallee!($msg))
    };
    ($err:expr $(,)?) => {
        return $crate::__private::Err($crate::__wallee!($err))
    };
    ($fmt:expr, $($arg:tt)*) => {
        return $crate::__private::Err($crate::__wallee!($fmt, $($arg)*))
    };
}

/// Return early with an error if a condition is not satisfied.
///
/// This macro is equivalent to `if !$cond { return
/// Err(`[`wallee!($args...)`][wallee!]`); }`.
///
/// The surrounding function's or closure's return value is required to be
/// `Result<_,`[`wallee::Error`][crate::Error]`>`.
///
/// Analogously to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`
/// rather than panicking.
///
/// [wallee!]: crate::wallee
///
/// # Example
///
/// ```
/// # use wallee::{ensure, Result};
/// #
/// # fn main() -> Result<()> {
/// #     let user = 0;
/// #
/// ensure!(user == 0, "only user 0 is allowed");
/// #     Ok(())
/// # }
/// ```
///
/// ```
/// # use wallee::{ensure, Result};
/// # use thiserror::Error;
/// #
/// # const MAX_DEPTH: usize = 1;
/// #
/// #[derive(Error, Debug)]
/// enum ScienceError {
///     #[error("recursion limit exceeded")]
///     RecursionLimitExceeded,
///     # #[error("...")]
///     # More = (stringify! {
///     ...
///     # }, 1).1,
/// }
///
/// # fn main() -> Result<()> {
/// #     let depth = 0;
/// #
/// ensure!(depth <= MAX_DEPTH, ScienceError::RecursionLimitExceeded);
/// #     Ok(())
/// # }
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! ensure {
    ($cond:expr $(,)?) => {
        if !$cond {
            return $crate::__private::Err($crate::Error::msg(
                $crate::__private::concat!("Condition failed: `", $crate::__private::stringify!($cond), "`")
            ));
        }
    };
    ($cond:expr, $msg:literal $(,)?) => {
        if !$cond {
            return $crate::__private::Err($crate::__wallee!($msg));
        }
    };
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return $crate::__private::Err($crate::__wallee!($err));
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            return $crate::__private::Err($crate::__wallee!($fmt, $($arg)*));
        }
    };
}

#[cfg(not(doc))]
#[macro_export]
macro_rules! ensure {
    ($($tt:tt)*) => {
        $crate::__parse_ensure!(
            /* state */ 0
            /* stack */ ()
            /* bail */ ($($tt)*)
            /* fuel */ (~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~ ~~~~~~~~~~)
            /* parse */ {()}
            /* dup */ ($($tt)*)
            /* rest */ $($tt)*
        )
    };
}

/// Construct an ad-hoc error from a string or existing non-`wallee` error
/// value.
///
/// This evaluates to an [`Error`][crate::Error]. It can take either just a
/// string, or a format string with arguments. It also can take any custom type
/// which implements `Debug` and `Display`.
///
/// If called with a single argument whose type implements `std::error::Error`
/// (in addition to `Debug` and `Display`, which are always required), then that
/// Error impl's `source` is preserved as the `source` of the resulting
/// `wallee::Error`.
///
/// # Example
///
/// ```
/// # type V = ();
/// #
/// use wallee::{wallee, Result};
///
/// fn lookup(key: &str) -> Result<V> {
///     if key.len() != 16 {
///         return Err(wallee!("key length must be 16 characters, got {:?}", key));
///     }
///
///     // ...
///     # Ok(())
/// }
/// ```
#[macro_export]
macro_rules! wallee {
    ($msg:literal $(,)?) => {
        $crate::__private::must_use({
            let error = $crate::__private::format_err($crate::__private::format_args!($msg));
            error
        })
    };
    ($err:expr $(,)?) => {
        $crate::__private::must_use({
            use $crate::__private::kind::*;
            let error = match $err {
                error => (&error).wallee_kind().make(error),
            };
            error
        })
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::msg($crate::__private::format!($fmt, $($arg)*))
    };
}

// Not public API. This is used in the implementation of some of the other
// macros, in which the must_use call is not needed because the value is known
// to be used.
#[doc(hidden)]
#[macro_export]
macro_rules! __wallee {
    ($msg:literal $(,)?) => ({
        let error = $crate::__private::format_err($crate::__private::format_args!($msg));
        error
    });
    ($err:expr $(,)?) => ({
        use $crate::__private::kind::*;
        let error = match $err {
            error => (&error).wallee_kind().make(error),
        };
        error
    });
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::msg($crate::__private::format!($fmt, $($arg)*))
    };
}

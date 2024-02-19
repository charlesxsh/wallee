#[cfg(feature = "std")]
pub type Location = std::panic::Location<'static>;

#[cfg(not(feature = "std"))]
pub struct Location<'a> {
    pub file: &'a str,
    pub line: u32,
    pub column: u32,
}

#[cfg(feature = "std")]
macro_rules! caller {
    () => {
        *$crate::location::Location::caller()
    };
}

#[cfg(not(feature = "std"))]
macro_rules! caller {
    () => {
        Location {
            file: "",
            line: 0,
            column: 0,
        }
    };
}

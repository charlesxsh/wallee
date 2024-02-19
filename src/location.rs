pub type Location = std::panic::Location<'static>;

macro_rules! caller {
    () => {
        *$crate::location::Location::caller()
    };
}

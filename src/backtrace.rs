pub(crate) use std::backtrace::{Backtrace, BacktraceStatus};

macro_rules! impl_backtrace {
    () => {
        std::backtrace::Backtrace
    };
}

macro_rules! backtrace {
    () => {
        Some(crate::backtrace::Backtrace::capture())
    };
}

macro_rules! backtrace_if_absent {
    ($err:expr) => {
        backtrace!()
    };
}

fn _assert_send_sync() {
    fn _assert<T: Send + Sync>() {}
    _assert::<Backtrace>();
}

//! `log`
//!
//! log feature facade

#[cfg(feature = "log")]
#[allow(unused_imports)]
pub(crate) use log::{debug, error, info, log, log_enabled, trace, warn, Level};

#[cfg(not(feature = "log"))]
macro_rules! _null_log {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

#[cfg(not(feature = "log"))]
#[allow(unused_macros)]
macro_rules! log_enabled {
    (target: $target:expr, $lvl:expr) => {
        false
    };
    ($lvl:expr) => {
        false
    };
}

#[cfg(not(feature = "log"))]
#[allow(dead_code)]
#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub(crate) enum Level {
    /// The "error" level.
    ///
    /// Designates very serious errors.
    // This way these line up with the discriminants for LevelFilter below
    // This works because Rust treats field-less enums the same way as C does:
    // https://doc.rust-lang.org/reference/items/enumerations.html#custom-discriminant-values-for-field-less-enumerations
    Error = 1,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug,
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace,
}

#[cfg(not(feature = "log"))]
#[allow(unused_imports)]
pub(crate) use {
    _null_log as debug, _null_log as error, _null_log as info, _null_log as log,
    _null_log as trace, _null_log as warn, log_enabled,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_test() {
        debug!("debug testing");
    }

    #[test]
    fn error_test() {
        error!("error testing");
    }

    #[test]
    fn log_test() {
        log!(Level::Error, "log testing");
    }

    #[test]
    fn log_enabled_test() {
        let l = log_enabled!(Level::Error);
        assert!(!l);
    }

    #[test]
    fn info_test() {
        info!("info testing");
    }

    #[test]
    fn trace_test() {
        trace!("trace testing");
    }

    #[test]
    fn warn_test() {
        warn!("warn testing");
    }
}

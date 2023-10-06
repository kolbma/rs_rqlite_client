//! `tracing`
//!
//! tracing feature facade

#[cfg(feature = "tracing")]
#[allow(unused_imports)]
pub(crate) use tracing::{
    debug, error, if_log_enabled, info, level_enabled, level_filters::LevelFilter, trace, warn,
    Level,
};

#[cfg(not(feature = "tracing"))]
macro_rules! _null_tracing {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

#[cfg(not(feature = "tracing"))]
#[allow(dead_code)]
#[allow(unused_imports)]
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! level_enabled {
    ($lvl:expr) => {
        false
    };
}

#[cfg(not(feature = "tracing"))]
#[allow(unused_imports)]
pub(crate) use {
    _null_tracing as debug, _null_tracing as error, _null_tracing as if_log_enabled,
    _null_tracing as info, _null_tracing as LevelFilter, _null_tracing as trace,
    _null_tracing as warn, _null_tracing as Level, level_enabled,
};

#[cfg(not(feature = "tracing"))]
#[allow(dead_code)]
#[repr(usize)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub(crate) enum Level {
    /// The "trace" level.
    ///
    /// Designates very low priority, often extremely verbose, information.
    Trace = 0,
    /// The "debug" level.
    ///
    /// Designates lower priority information.
    Debug = 1,
    /// The "info" level.
    ///
    /// Designates useful information.
    Info = 2,
    /// The "warn" level.
    ///
    /// Designates hazardous situations.
    Warn = 3,
    /// The "error" level.
    ///
    /// Designates very serious errors.
    Error = 4,
    /// Disabled facade
    Disabled = 100,
}

#[cfg(not(feature = "tracing"))]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct LevelFilter(Option<Level>);

#[cfg(not(feature = "tracing"))]
#[allow(dead_code)]
impl LevelFilter {
    pub(crate) fn current() -> Level {
        Level::Disabled
    }
}

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
    fn if_log_enabled_test() {
        if_log_enabled!(Level::Error, {});
    }

    #[test]
    fn info_test() {
        info!("info testing");
    }

    #[test]
    fn level_enabled_test() {
        let l = level_enabled!(Level::TRACE);
        assert!(!l);
    }

    #[test]
    fn level_filter_test() {
        let _ = LevelFilter::current();
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

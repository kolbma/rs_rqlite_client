//! `buildtime`
//!

include!(concat!(env!("OUT_DIR"), "/buildtime.rs"));

/// Build time stamp of the crate
pub const BUILD_TIME: &str = CRATE_BUILD_TIME;

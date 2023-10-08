//! `Monitor` states for `Query<State>`
#![cfg(feature = "monitor")]

use crate::state::State;

pub use endpoint::Endpoint;
pub use nodes::Nodes;
pub use readyz::Readyz;

mod endpoint;
mod nodes;
mod readyz;
pub mod response;

/// rqlite serves diagnostic and statistical information, as well as detailed information about
/// the underlying Raft system
///
/// See <https://rqlite.io/docs/guides/monitoring-rqlite/#status-api>
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monitor;
impl State for Monitor {}

#[cfg(test)]
#[cfg(any(feature = "percent_encoding", feature = "url"))]
mod tests {
    use std::time::Duration;

    use lazy_static::lazy_static;

    use crate::{Connection, Query};

    const TEST_CONNECTION_URL: &str = "http://localhost:4001/";

    #[cfg(feature = "url")]
    lazy_static! {
        static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL).unwrap();
    }
    #[cfg(not(feature = "url"))]
    lazy_static! {
        static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL);
    }

    #[test]
    fn monitor_status_test() {
        let mut q = Query::new(&TEST_CONNECTION).monitor();

        assert_eq!(&q.create_path_with_query(), "/status");

        q = q.set_pretty();

        assert_eq!(&q.create_path_with_query(), "/status?pretty");

        q = q.set_timeout(Duration::from_secs(3).into());
        assert_eq!(&q.create_path_with_query(), "/status?pretty&timeout=3s");
    }
}

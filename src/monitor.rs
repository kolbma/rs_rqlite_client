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
    use std::{sync::OnceLock, time::Duration};

    use crate::{Connection, Query};

    const TEST_CONNECTION_URL: &str = "http://localhost:4001/";

    static TEST_CONNECTION: OnceLock<Connection> = OnceLock::new();

    fn test_connection() -> &'static Connection {
        TEST_CONNECTION.get_or_init(|| {
            #[cfg(feature = "url")]
            let c = Connection::new(TEST_CONNECTION_URL).unwrap();
            #[cfg(not(feature = "url"))]
            let c = Connection::new(TEST_CONNECTION_URL);

            c
        })
    }

    #[test]
    fn monitor_status_test() {
        let mut q = Query::new(test_connection()).monitor();

        assert_eq!(&q.create_path_with_query(), "/status");

        q = q.set_pretty();

        assert_eq!(&q.create_path_with_query(), "/status?pretty");

        q = q.set_timeout(Duration::from_secs(3).into());
        assert_eq!(&q.create_path_with_query(), "/status?pretty&timeout=3s");
    }
}

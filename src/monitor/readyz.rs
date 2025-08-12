use crate::{state::State, Query};

/// rqlite nodes serve a _ready_ status [`monitor::Endpoint::Readyz`](crate::monitor::Endpoint::Readyz)
/// if the node is ready to respond to database requests and cluster management operations.
///
/// If you wish to check if the node is running, and responding to HTTP requests, regardless of
/// Leader status, `enable_noleader`.
///
/// See <https://rqlite.io/docs/guides/monitoring-rqlite/#readiness-checks>
///
#[derive(Debug, Eq, PartialEq)]
pub struct Readyz;
impl State for Readyz {}

impl Query<'_, Readyz> {
    /// Enable noleader query param to check all nodes, regardless of Leader status
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#readiness-checks>
    ///
    #[must_use]
    pub fn enable_noleader(self) -> Self {
        self.enable_noleader_helper()
    }

    /// Enable sync query param to block until the node has received the log entry
    /// equal to Leader’s Commit Index as it was set by the latest Heartbeat
    /// received from the Leader.\
    /// This allows you to check that a node is “caught up” with the Leader.
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#sync-flag>
    ///
    #[must_use]
    pub fn enable_sync(self) -> Self {
        self.enable_sync_helper()
    }

    /// Check for readiness `sync` query flag status
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#sync-flag>
    ///
    #[must_use]
    #[inline]
    pub fn is_sync(&self) -> bool {
        self.is_sync
    }
}

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
    fn monitor_readyz_test() {
        let mut q = Query::new(&TEST_CONNECTION).monitor().readyz();

        assert_eq!(&q.create_path_with_query(), "/readyz");

        q = q.set_pretty();

        assert_eq!(&q.create_path_with_query(), "/readyz?pretty");

        q = q.enable_noleader();

        assert_eq!(&q.create_path_with_query(), "/readyz?noleader&pretty");

        q = q.set_timeout(Duration::from_secs(3).into());
        assert_eq!(
            &q.create_path_with_query(),
            "/readyz?noleader&pretty&timeout=3s"
        );
    }

    #[test]
    fn monitor_readyz_sync_test() {
        let mut q = Query::new(&TEST_CONNECTION).monitor().readyz();

        assert_eq!(&q.create_path_with_query(), "/readyz");

        q = q.enable_sync();

        assert_eq!(&q.create_path_with_query(), "/readyz?sync");

        q = q.set_timeout(Duration::from_secs(3).into());
        assert_eq!(&q.create_path_with_query(), "/readyz?sync&timeout=3s");
    }
}

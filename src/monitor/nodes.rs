use crate::{state::State, Query};

/// _Nodes_ return basic information for nodes in the cluster, as seen by the node
/// receiving the nodes request. The receiving node will also check whether it can actually
/// connect to all other nodes in the cluster.\
/// This is an effective way to determine the cluster leader, and the leader’s HTTP API address.
/// It can also be used to check if the cluster is basically running.
/// If the other nodes are reachable, it probably is.
///
/// By default, the node only checks if voting nodes are contactable.
///
/// See <https://rqlite.io/docs/guides/monitoring-rqlite/#nodes-api>
///
#[derive(Debug, Eq, PartialEq)]
pub struct Nodes;
impl State for Nodes {}

impl Query<'_, Nodes> {
    /// Enable nonvoters query param to check also read-only nodes
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#nodes-api>
    #[must_use]
    pub fn enable_nonvoters(self) -> Self {
        self.enable_nonvoters_helper()
    }

    /// Request easier parseable version 2 JSON format
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#nodes-api>
    #[must_use]
    pub fn enable_version2(self) -> Self {
        self.enable_version_helper(2)
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
    fn monitor_nodes_test() {
        let mut q = Query::new(&TEST_CONNECTION).monitor().nodes();

        assert_eq!(&q.create_path_with_query(), "/nodes");

        q = q.set_pretty();

        assert_eq!(&q.create_path_with_query(), "/nodes?pretty");

        q = q.enable_nonvoters();

        assert_eq!(&q.create_path_with_query(), "/nodes?nonvoters&pretty");

        q = q.set_timeout(Duration::from_secs(3).into());
        assert_eq!(
            &q.create_path_with_query(),
            "/nodes?nonvoters&pretty&timeout=3s"
        );
    }

    #[test]
    fn monitor_nodes_version2_test() {
        let mut q = Query::new(&TEST_CONNECTION).monitor().nodes();

        assert_eq!(&q.create_path_with_query(), "/nodes");

        q = q.set_pretty();

        assert_eq!(&q.create_path_with_query(), "/nodes?pretty");

        q = q.enable_version2();

        assert_eq!(&q.create_path_with_query(), "/nodes?pretty&ver=2");
    }
}

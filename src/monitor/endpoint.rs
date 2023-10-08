//! The supported monitor [`Endpoint`]s nodes, readyz, status

/// The supported monitor `Endpoint`s nodes, readyz, status
///
#[derive(Debug, Default, Eq, PartialEq)]
pub enum Endpoint {
    /// `Nodes` endpoint for cluster node information
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#nodes-api>
    Nodes,
    /// `Readyz` endpoint for node check
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#readiness-checks>
    Readyz,
    /// Diagnostic and statistical information `Status` endpoint
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#status-api>
    #[default]
    Status,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::Nodes => f.write_str("/nodes"),
            Endpoint::Readyz => f.write_str("/readyz"),
            Endpoint::Status => f.write_str("/status"),
        }
    }
}

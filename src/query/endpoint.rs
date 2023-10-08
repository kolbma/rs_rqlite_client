//! The supported [`Endpoint`]s execute, nodes, query, readyz, request, status

/// The supported `Endpoint`s execute, nodes, query, readyz, request, status
///
/// [`monitor::Monitor`](crate::monitor::Monitor) endpoints require feature `monitor`.
///
#[derive(Debug, Default, Eq, PartialEq)]
pub enum Endpoint {
    /// `Execute` modifications
    ///
    /// See <https://rqlite.io/docs/api/api/#writing-data>
    Execute,

    /// [`Monitor`](crate::monitor::Monitor) endpoints
    ///
    /// Requires feature `monitor`.
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/>
    #[cfg(feature = "monitor")]
    Monitor(crate::monitor::Endpoint),

    /// `Query` select statements
    ///
    /// See <https://rqlite.io/docs/api/api/#querying-data>
    #[default]
    Query,

    /// `Request` unified endpoint
    ///
    /// See <https://rqlite.io/docs/api/api/#unified-endpoint>
    Request,
}

impl std::fmt::Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Endpoint::Execute => f.write_str("/db/execute"),
            #[cfg(feature = "monitor")]
            Endpoint::Monitor(monitor) => monitor.fmt(f),
            Endpoint::Query => f.write_str("/db/query"),
            Endpoint::Request => f.write_str("/db/request"),
        }
    }
}

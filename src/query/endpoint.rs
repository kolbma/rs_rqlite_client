//! The supported [`Endpoint`]s execute, query, request

/// The supported `Endpoint`s execute, query, request
///
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Endpoint {
    /// `Execute` modifications
    ///
    /// See <https://rqlite.io/docs/api/api/#writing-data>
    Execute,
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
            Endpoint::Query => f.write_str("/db/query"),
            Endpoint::Request => f.write_str("/db/request"),
        }
    }
}

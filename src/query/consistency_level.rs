//! rqlite support various read-[`ConsistencyLevel`]

/// rqlite support various read-[`ConsistencyLevel`]
///
/// This is why rqlite offers selectable read consistency levels of weak (the default), strong, and none.
/// Each is explained below, and examples of each are shown at the end of this page.
///
/// See <https://rqlite.io/docs/api/read-consistency/>
///
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ConsistencyLevel {
    /// No level set
    #[default]
    Nolevel,
    /// With none, the node receiving your read request simply queries its local SQLite database, and does not perform
    /// any Leadership check – in fact, the node could be completely disconnected from the rest of the cluster, but
    /// the query will still be successful. This offers the fastest query response, but suffers from the potential
    /// issues outlined above, whereby there is a chance of Stale Reads if the Leader changes during the query, or if
    /// the node has become disconneted from the cluster.
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#none>
    None,
    /// To avoid even the issues associated with weak consistency, rqlite also offers strong. In this mode, the node
    /// receiving the request sends the query through the Raft consensus system, ensuring that the cluster Leader
    /// remains the Leader at all times during the processing of the query. When using strong you can be sure that
    /// the database reflects every change sent to it prior to the query. However, this will involve the Leader
    /// contacting at least a quorum of nodes, and will therefore increase query response times.
    ///
    /// If a query request is sent to a Follower, and strong consistency is specified, the Follower will
    /// transparently forward the request to the Leader. The Follower waits for the response from the Leader, and
    /// then returns that response to the client.
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#strong>
    Strong,
    /// Weak instructs the node receiving the read request to check that it is the Leader, before querying the local
    /// SQLite file. Checking Leader state only involves checking state local to the node, so is very fast. There is,
    /// however, a very small window of time (milliseconds by default) during which the node may return stale data if
    /// a Leader-election is in progress. This is because after the local Leader check, but before the local SQLite
    /// database is read, another node could be elected Leader and make changes to the cluster. As result the node
    /// may not be quite up-to-date with the rest of cluster.
    ///
    /// If the node determines it is not the Leader, the node will transparently forward the request to the Leader,
    /// which will in turn perform a Weak read of its database. The node then waits for the response from the Leader,
    /// and then returns that response to the client
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#weak>
    Weak,
}

impl std::fmt::Display for ConsistencyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConsistencyLevel::Nolevel => f.write_str(""),
            ConsistencyLevel::None => f.write_str("none"),
            ConsistencyLevel::Strong => f.write_str("strong"),
            ConsistencyLevel::Weak => f.write_str("weak"),
        }
    }
}

//! rqlite support various Read-[`ConsistencyLevel`]

/// rqlite support various Read-[`ConsistencyLevel`]
///
/// You do not need to know the information on this page to use rqlite well, it's mostly for advanced users.\
/// rqlite has also been run through Jepsen-style testing. You can read about that [here](https://github.com/wildarch/jepsen.rqlite/blob/main/doc/blog.md).
///
/// Even though serving queries does not require Raft consensus (because the database is not changed), queries should
/// generally be served by the cluster Leader. Why is this? Because, without this check, queries on a node could
/// return results that are out-of-date i.e. stale. This could happen for one, or both, of the following two reasons:
///
/// * The node that received your request, while still part of the cluster, has fallen behind the Leader in terms of
///   updates to its underlying database.
/// * The node is no longer part of the cluster, and has stopped receiving Raft log updates.
///
/// This is why rqlite offers selectable read consistency levels of weak (the default), linearizable, strong, and
/// none. Each is explained below, and examples of each are shown at the end of this page.
///
/// See <https://rqlite.io/docs/api/read-consistency/>
///
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ConsistencyLevel {
    /// Auto is not an actual Read Consistency level. Instead if a client selects this level during a read request,
    /// the receiving node will automatically select the level which is (usually) most sensible for the node's type.
    /// In the case of a read-only node None is chosen as the level. In all other cases Weak is the chosen as the
    /// level.
    ///
    /// Using auto can simplify clients as clients do not need know ahead of time whether they will be talking to a
    /// read-only node or voting node. A client can just select auto.
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#auto>
    Auto,
    /// Linearizable reads implement the process described in section 6.4 of the Raft dissertation titled
    /// _Processing read-only queries more efficiently_.
    ///
    /// To avoid the issues associated with weak consistency, rqlite also offers linearizable.
    ///
    /// This type of read is, as the name suggests, linearizable because these types of reads reflect a state of the
    /// system sometime after the read was initiated; each read will at least return the results of the latest
    /// committed write. Linearizable reads are reasonably fast, though measurably slower than weak.
    ///
    /// How does the node guarantee linearizable reads? It does this as follows: when the node receives the read
    /// request it records the Raft Commit Index, and as well as checking local state to see if it is the Leader.
    /// Next the node heartbeats with the Followers, and waits until it receives a quorum of responses.
    /// Finally – and this is critical – the Leader waits until at least the write request contained in the previously
    /// recorded commit index is applied to the SQLite database. Once this happens it then performs the read.
    ///
    /// Linearizable reads means the Leader contacts at least a quorum of nodes, and will therefore increase query
    /// response times. But since the Raft log is not actually involved, read performance is only dependant on the
    /// network performance between the nodes.
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#linearizable>
    Linearizable,
    /// No level set
    #[default]
    Nolevel,
    /// With none, the node receiving your read request simply queries its local SQLite database, and does not perform
    /// any Leadership check – in fact, the node could be completely disconnected from the rest of the cluster, but
    /// the query will still be successful. This offers the absolute fastest query response, but suffers from the
    /// potential issues outlined above, whereby there is a chance of Stale Reads if the Leader changes during the
    /// query, or if the node has become disconnected from the cluster.
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#none>
    ///
    /// See also [`query::freshness::Freshness`](crate::query::freshness::Freshness).
    None,
    /// Strong consistency has little use in production systems, as the reads are costly, consume disk space, and do
    /// not offer any benefit over Linearizable reads. __Don't use Strong in production__. Strong reads can be useful
    /// in certain testing scenarios however.
    ///
    /// rqlite also offers a consistency level known as strong. In this mode, the node receiving the request ensures
    /// that all committed entries in the Raft log have also been applied to the SQLite database at the time the query
    /// is executed. Strong reads accomplish this by sending the query through the actual Raft log. This will, of
    /// course, involve the Leader contacting at least a quorum of nodes, some disk IO, and will therefore increase
    /// query response times. Strong reads are linearizable.
    ///
    /// If a query request is sent to a Follower, and strong consistency is specified, the Follower will transparently
    /// forward the request to the Leader. The Follower waits for the response from the Leader, and then returns that
    /// response to the client.
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#strong>
    Strong,
    /// Weak consistency is used if you don't specify any level, or if an unrecognized level is specified – and it's
    /// almost certainly the right choice for your application.
    ///
    /// Weak instructs the node receiving the read request to check that it is the Leader, and if it is the Leader,
    /// the node simply reads its local SQLite database. If the node determines it is not the Leader, the node will
    /// transparently forward the request to the Leader, which will in turn perform a Weak read of its database. In
    /// that case the node waits for the response from the Leader, and then returns that response to the client.
    ///
    /// Weak reads are usually very fast, but have some potential shortcomings, which are described below.
    ///
    /// A node checks if it's the Leader by checking state local to the node, so this check is very fast. However
    /// there is a small window of time (less than a second by default) during which a node may think it's the Leader,
    /// but has actually been deposed, a new Leader elected, and other writes have taken place on the cluster. If this
    /// happens the node may not be quite up-to-date with the rest of cluster, and stale data may be returned.
    /// Technically this means that weak reads are not Linearizable.
    ///
    /// In practise this type of inconsistency is unlikely to happen, which is why Weak is the right choice for most
    /// applications.
    ///
    /// See <https://rqlite.io/docs/api/read-consistency/#weak>
    Weak,
}

impl std::fmt::Display for ConsistencyLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => f.write_str("auto"),
            Self::Linearizable => f.write_str("linearizable"),
            Self::Nolevel => f.write_str(""),
            Self::None => f.write_str("none"),
            Self::Strong => f.write_str("strong"),
            Self::Weak => f.write_str("weak"),
        }
    }
}

use std::collections::HashMap;

use crate::response::mapping::Timed;

use super::NodesV2;

/// Data container for response of [`monitor::Endpoint::Nodes`](crate::monitor::Endpoint::Nodes)
///
/// See also [`monitor::Nodes`](crate::monitor::Nodes)
///
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Nodes(pub HashMap<String, NodeState>);

impl std::ops::Deref for Nodes {
    type Target = HashMap<String, NodeState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "ureq")]
impl TryFrom<ureq::Response> for Nodes {
    type Error = crate::Error;

    fn try_from(response: ureq::Response) -> Result<Self, Self::Error> {
        let status = response.status();

        if !(200..300).contains(&status) {
            return Err(crate::Error::HttpError(
                status,
                response.status_text().to_string(),
            ));
        }

        response.into_json::<Self>().map_err(Self::Error::from)
    }
}

impl From<Nodes> for NodesV2 {
    fn from(nodes: Nodes) -> Self {
        Self {
            nodes: nodes.values().cloned().collect(),
        }
    }
}

impl From<NodesV2> for Nodes {
    fn from(nodes_v2: NodesV2) -> Self {
        let mut nodes = HashMap::new();
        for node in nodes_v2.nodes {
            let _ = nodes.insert(node.id.clone(), node);
        }
        Self(nodes)
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NodeState {
    pub addr: String,
    pub api_addr: String,
    pub id: String,
    pub leader: bool,
    pub reachable: bool,
    pub time: f64,
}

impl Timed for NodeState {
    fn time(&self) -> Option<f64> {
        Some(self.time)
    }
}

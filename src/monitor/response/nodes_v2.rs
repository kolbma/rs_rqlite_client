use super::nodes::NodeState;

/// Data container for response of [`monitor::Endpoint::Nodes`](crate::monitor::Endpoint::Nodes) Version 2
///
/// See also [`monitor::Nodes`](crate::monitor::Nodes)
///
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct NodesV2 {
    pub(crate) nodes: Vec<NodeState>,
}

impl std::ops::Deref for NodesV2 {
    type Target = Vec<NodeState>;

    fn deref(&self) -> &Self::Target {
        &self.nodes
    }
}

#[cfg(feature = "ureq")]
impl TryFrom<ureq::Response> for NodesV2 {
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

//! Everything related to __rqlite__ database server `response` handling

use crate::Error;
pub use query::Query;

pub mod mapping;
pub mod query;

/// Result type with [`Response`] or [`Error`]
pub type Result = std::result::Result<Response, Error>;

/// [`Response`] `enum` for handling different __rqlited__ database server responses
#[derive(Debug, PartialEq)]
pub enum Response {
    /// Response of [`monitor::Endpoint::Nodes`](crate::monitor::response::Nodes) (feature `monitor`)
    #[cfg(feature = "monitor")]
    Node(crate::monitor::response::Nodes),
    /// Response of [`monitor::Endpoint::Readyz`](crate::monitor::response::Readyz) (feature `monitor`)
    #[cfg(feature = "monitor")]
    Readyz(crate::monitor::response::Readyz),
    /// Response of [`query::Endpoint::Execute`](crate::query::Endpoint::Execute),
    /// [`query::Endpoint::Query`](crate::query::Endpoint::Query),
    /// [`query::Endpoint::Request`](crate::query::Endpoint::Request)
    Query(Query),
    /// Response of [`monitor::Endpoint::Status`](crate::monitor::response::Status) (feature `monitor`)
    #[cfg(feature = "monitor")]
    Status(crate::monitor::response::Status),
}

#[cfg(feature = "ureq")]
impl TryFrom<ureq::Response> for Response {
    type Error = Error;

    fn try_from(response: ureq::Response) -> std::result::Result<Self, Self::Error> {
        let status = response.status();

        if !(200..300).contains(&status) {
            return Err(Error::HttpError(status, response.status_text().to_string()));
        }

        let mut value = None;
        let mut content = None;

        // checks
        if let Some(content_type) = response.header("Content-Type") {
            let content_length = response
                .header("Content-Length")
                .and_then(|s| s.parse::<usize>().ok());

            if content_type.starts_with("application/json") {
                value = Some(response.into_json::<crate::Value>().map_err(Error::from)?);
            } else if let Some(content_length) = content_length {
                if content_length <= 200 && content_type.starts_with("text/plain") {
                    content = response.into_string().ok();
                } else {
                    return Err("content-length too big".into());
                }
            } else {
                return Err("unsupported response".into());
            }
        } else {
            return Err("unsupported response".into());
        }

        // response type parsing - feature monitor
        #[cfg(feature = "monitor")]
        if let Some(value) = value {
            if value.get("results").is_some() {
                serde_json::from_value::<Query>(value)
                    .map(Response::Query)
                    .map_err(Error::from)
            } else if value.get("build").is_some() {
                serde_json::from_value::<crate::monitor::response::Status>(value)
                    .map(Response::Status)
                    .map_err(Error::from)
            } else if value.get("nodes").is_some() {
                serde_json::from_value::<crate::monitor::response::NodesV2>(value)
                    .map(|nodes_v2| Response::Node(crate::monitor::response::Nodes::from(nodes_v2)))
                    .map_err(Error::from)
            } else if value.is_object() {
                serde_json::from_value::<crate::monitor::response::Nodes>(value)
                    .map(Response::Node)
                    .map_err(Error::from)
            } else {
                Err(Error::ResponseError(value))
            }
        } else if let Some(content) = content {
            use std::str::FromStr;
            Ok(Response::Readyz(
                crate::monitor::response::Readyz::from_str(&content)?,
            ))
        } else {
            Err("unsupported response".into())
        }

        // response type parsing - NOT feature monitor
        #[cfg(not(feature = "monitor"))]
        if let Some(value) = value {
            if value.get("results").is_some() {
                serde_json::from_value::<Query>(value)
                    .map(Response::Query)
                    .map_err(Error::from)
            } else {
                Err(Error::ResponseError(value))
            }
        } else {
            let _ = content;
            Err("unsupported response".into())
        }
    }
}

#[allow(unreachable_patterns)]
impl From<Response> for Query {
    fn from(response: Response) -> Self {
        match response {
            Response::Query(r) => r,
            _ => panic!("not matching"),
        }
    }
}

#[cfg(feature = "monitor")]
impl From<Response> for crate::monitor::response::Nodes {
    fn from(response: Response) -> Self {
        match response {
            Response::Node(r) => r,
            _ => panic!("not matching"),
        }
    }
}

#[cfg(feature = "monitor")]
impl From<Response> for crate::monitor::response::NodesV2 {
    fn from(response: Response) -> Self {
        match response {
            Response::Node(r) => r.into(),
            _ => panic!("not matching"),
        }
    }
}

#[cfg(feature = "monitor")]
impl From<Response> for crate::monitor::response::Readyz {
    fn from(response: Response) -> Self {
        match response {
            Response::Readyz(r) => r,
            _ => panic!("not matching"),
        }
    }
}

#[cfg(feature = "monitor")]
impl From<Response> for crate::monitor::response::Status {
    fn from(response: Response) -> Self {
        match response {
            Response::Status(r) => r,
            _ => panic!("not matching"),
        }
    }
}

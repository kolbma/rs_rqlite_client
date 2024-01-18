//! Implemented [`Request`] handling utilizing _crate_ [`ureq`](https://crates.io/crates/ureq)
#![cfg(feature = "ureq")]

use std::marker::PhantomData;

use lazy_static::lazy_static;

#[allow(clippy::module_name_repetitions)]
pub use self::request_type::RequestType;
use self::request_type::{Get, Post};
use crate::response::Result;
use crate::{log, tracing, Connection, Response};
use crate::{
    query::{Query, State},
    Error, RequestBuilder,
};

#[allow(clippy::module_name_repetitions)]
pub mod request_type;
mod tls;

/// Implemented [`Request`] handling utilizing _crate_ [`ureq`](https://crates.io/crates/ureq)
///
/// Requires enabled feature `ureq`.
///
/// Requests can be initialized manually, but there should be little requirements for.  
/// The preferred usage is via [`Query`] and [`Query::request_run()`].
///
#[derive(Clone, Debug)]
pub struct Request<T>
where
    T: RequestType,
{
    agent: Option<ureq::Agent>,
    t: PhantomData<T>,
}

impl<T> Request<T>
where
    T: RequestType,
{
    /// Create new `Request`
    #[must_use]
    pub fn new() -> Self {
        Self {
            agent: None,
            t: PhantomData,
        }
    }

    /// Create new `Request` for [`Connection`]
    ///
    /// Build and use a new [`ureq::Agent`].
    ///
    #[must_use]
    #[inline]
    pub fn from_connection(connection: &Connection) -> Self {
        let mut r = Request::<T>::new();
        r.agent = Some(user_agent(Some(connection)));
        r
    }
}

#[inline]
pub(crate) fn user_agent(connection: Option<&Connection>) -> ureq::Agent {
    let agent = ureq::AgentBuilder::new().user_agent(&DEFAULT_USER_AGENT);

    let proxy = connection
        .and_then(|c| {
            c.proxy()
                .map(String::from)
                .or_else(Connection::detect_proxy)
        })
        .or_else(Connection::detect_proxy);

    let agent = if let Some(proxy) = &proxy {
        log::debug!("try proxy {proxy}");
        tracing::debug!("try proxy {proxy}");
        match ureq::Proxy::new(proxy) {
            Ok(ureq_proxy) => {
                log::info!("use proxy {proxy}");
                tracing::info!("use roxy {proxy}");
                agent.proxy(ureq_proxy)
            }
            Err(err) => {
                let _ = err;
                log::warn!("fail proxy {proxy}: {err}");
                tracing::warn!("fail proxy {proxy}: {err}");
                agent
            }
        }
    } else {
        agent
    };

    agent.build()
}

lazy_static! {
    /// default HTTP User-Agent header
    static ref DEFAULT_USER_AGENT: String = {
        format!("rqlite_client/{}", crate::BUILD_TIME)
    };

    /// request agent singleton
    static ref UREQ_AGENT: ureq::Agent = user_agent(None);
}

impl Request<Get> {
    fn request<T: State>(agent: Option<&ureq::Agent>, query: &Query<T>) -> Result {
        log::debug!("[GET] {}: {:?}", query.to_string(), query.sql());
        tracing::debug!("[GET] {}: {:?}", query.to_string(), query.sql());

        let agent = if let Some(agent) = agent {
            agent
        } else {
            &UREQ_AGENT
        };

        let r = agent
            .get(&query.to_string())
            .set("Content-Type", "application/json");

        let r = if let Some(timeout) = query.timeout_request() {
            r.timeout(*timeout)
        } else {
            r
        };

        let r = r.call().map_err(Error::from)?;

        Response::try_from(r)
    }
}

impl Request<Post> {
    fn request<T: State>(agent: Option<&ureq::Agent>, query: &Query<T>) -> Result {
        log::debug!("[POST] {}: {:?}", query.to_string(), query.sql());
        tracing::debug!("[POST] {}: {:?}", query.to_string(), query.sql());

        let agent = if let Some(agent) = agent {
            agent
        } else {
            &UREQ_AGENT
        };

        let r = agent
            .post(&query.to_string())
            .set("Content-Type", "application/json");

        let r = if let Some(timeout) = query.timeout_request() {
            r.timeout(*timeout)
        } else {
            r
        };

        let r = r.send_json(query.sql()).map_err(Error::from)?;

        Response::try_from(r)
    }
}

impl<T> Default for Request<T>
where
    T: RequestType,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> From<&Connection> for Request<T>
where
    T: RequestType,
{
    fn from(connection: &Connection) -> Self {
        Self::from_connection(connection)
    }
}

impl<T> From<Connection> for Request<T>
where
    T: RequestType,
{
    fn from(connection: Connection) -> Self {
        Self::from_connection(&connection)
    }
}

impl From<Get> for Request<Get> {
    fn from(_value: Get) -> Self {
        Request::<Get>::new()
    }
}

impl From<Post> for Request<Post> {
    fn from(_value: Post) -> Self {
        Request::<Post>::new()
    }
}

impl<S> RequestBuilder<S> for Request<Get>
where
    S: State,
{
    #[inline]
    fn run(&self, query: &Query<S>) -> Result {
        Self::request(self.agent.as_ref(), query)
    }
}

impl<S> RequestBuilder<S> for Request<Post>
where
    S: State,
{
    #[inline]
    fn run(&self, query: &Query<S>) -> Result {
        Self::request(self.agent.as_ref(), query)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        request_type::{Get, Post},
        Connection, Request,
    };

    #[test]
    fn request_get_test() {
        let request = Request::<Get>::new();
        assert!(request.agent.is_none());
        assert!(Request::from(Get).agent.is_none());
    }

    #[test]
    fn request_post_test() {
        let request = Request::<Post>::new();
        assert!(request.agent.is_none());
        assert!(Request::from(Post).agent.is_none());
    }

    #[test]
    fn request_connection_test() {
        let c = Connection::new("http://example.com");
        #[cfg(feature = "url")]
        let c = c.unwrap();

        assert!(Request::<Get>::from(&c).agent.is_some());
        assert!(Request::<Post>::from(c).agent.is_some());
    }
}

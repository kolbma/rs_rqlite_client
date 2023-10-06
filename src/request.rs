//! Implemented [`Request`] handling utilizing _crate_ [`ureq`](https://crates.io/crates/ureq)
#![cfg(feature = "ureq")]

use std::marker::PhantomData;

use lazy_static::lazy_static;

#[allow(clippy::module_name_repetitions)]
pub use self::request_type::RequestType;
use self::request_type::{Get, Post};
use crate::{log, tracing, Connection};
use crate::{
    query::{Query, State},
    Error, RequestBuilder, Response, ResponseResult,
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

    let agent = if let Some(proxy) = proxy {
        log::debug!("try proxy {proxy}");
        tracing::debug!("try proxy {proxy}");
        match ureq::Proxy::new(&proxy) {
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
    fn request<T: State>(agent: Option<&ureq::Agent>, query: &Query<T>) -> ResponseResult {
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

        let r = if let Some(timeout) = query.timeout() {
            r.timeout(*timeout)
        } else {
            r
        };

        let r = r.call().map_err(Error::from)?;

        Response::try_from(r)
    }
}

impl Request<Post> {
    fn request<T: State>(agent: Option<&ureq::Agent>, query: &Query<T>) -> ResponseResult {
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

        let r = if let Some(timeout) = query.timeout() {
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
    fn run(&self, query: &Query<S>) -> ResponseResult {
        Self::request(self.agent.as_ref(), query)
    }
}

impl<S> RequestBuilder<S> for Request<Post>
where
    S: State,
{
    #[inline]
    fn run(&self, query: &Query<S>) -> ResponseResult {
        Self::request(self.agent.as_ref(), query)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use lazy_static::lazy_static;

    use crate::{
        query::Query,
        result::{self, Result},
        test_rqlited::TEST_RQLITED_DB,
        Connection, DataType, Request, RequestBuilder,
    };

    use super::request_type::{Get, Post};

    const TEST_CONNECTION_URL: &str = "http://localhost:4001/";
    const TEST_PROXY_URL: &str = "http://proxy.example.com:12345";
    const TEST_SOCKS_PROXY_URL: &str = "socks5://user:password@127.0.0.1:12345";

    #[cfg(feature = "url")]
    lazy_static! {
        static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL).unwrap();
        static ref TEST_PROXY_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL)
            .unwrap()
            .set_proxy(TEST_PROXY_URL);
        static ref TEST_SOCKS_PROXY_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL)
            .unwrap()
            .set_proxy(TEST_SOCKS_PROXY_URL);
    }
    #[cfg(not(feature = "url"))]
    lazy_static! {
        static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL);
        static ref TEST_PROXY_CONNECTION: Connection =
            Connection::new(TEST_CONNECTION_URL).set_proxy(TEST_PROXY_URL);
        static ref TEST_SOCKS_PROXY_CONNECTION: Connection =
            Connection::new(TEST_CONNECTION_URL).set_proxy(TEST_SOCKS_PROXY_URL);
    }

    #[test]
    fn nolevel_test() {
        TEST_RQLITED_DB.run_test(|| {
            let r = Request::from(Get).run(&Query::new(&TEST_CONNECTION).set_sql_str("SELECT 1"));

            assert!(r.is_ok(), "response error: {}", r.err().unwrap());

            let r = r.unwrap();
            let result = r.results().next().unwrap();

            match result {
                Result::Standard(result) => {
                    assert_eq!(
                        result,
                        &result::Standard {
                            columns: vec!["1".to_string()],
                            time: None,
                            types: vec![DataType::Integer],
                            values: vec![vec![1.into()]]
                        }
                    );
                }
                _ => unreachable!(),
            }
        });
    }

    #[cfg(feature = "ureq")]
    #[test]
    fn nolevel_request_run_test() {
        TEST_RQLITED_DB.run_test(|| {
            let r = TEST_CONNECTION
                .query()
                .set_sql_str("SELECT 1")
                .request_run();

            assert!(r.is_ok(), "response error: {}", r.err().unwrap());

            let r = r.unwrap();
            let result = r.results().next().unwrap();

            match result {
                Result::Standard(result) => {
                    assert_eq!(
                        result,
                        &result::Standard {
                            columns: vec!["1".to_string()],
                            time: None,
                            types: vec![DataType::Integer],
                            values: vec![vec![1.into()]]
                        }
                    );
                }
                _ => unreachable!(),
            }
        });
    }

    #[test]
    fn proxy_test() {
        TEST_RQLITED_DB.run_test(|| {
            let r = Request::<Get>::from(&*TEST_PROXY_CONNECTION).run(
                &Query::new(&TEST_PROXY_CONNECTION)
                    .set_timeout(Duration::from_millis(10))
                    .set_sql_str("SELECT 1"),
            );

            assert!(r.is_err());
            let err_msg = r.unwrap_err().to_string();
            assert!(
                err_msg.contains("Dns Failed") || err_msg.contains("Connection Failed"),
                "{}",
                err_msg
            );
        });
    }

    #[test]
    fn request_timeout_test() {
        TEST_RQLITED_DB.run_test(|| {
            let r = Request::<Get>::new().run(
                &Query::new(&TEST_CONNECTION)
                    .set_timeout(Duration::from_nanos(10))
                    .set_sql_str("SELECT 1"),
            );

            assert!(r.is_err());
            let err_msg = r.unwrap_err().to_string();
            assert!(err_msg.contains("Network Error: timed out"), "{}", err_msg);
        });
    }

    #[test]
    fn socks_proxy_test() {
        TEST_RQLITED_DB.run_test(|| {
            let r = Request::<Get>::from(&*TEST_SOCKS_PROXY_CONNECTION).run(
                &Query::new(&TEST_SOCKS_PROXY_CONNECTION)
                    .set_timeout(Duration::from_millis(10))
                    .set_sql_str("SELECT 1"),
            );

            assert!(r.is_err());
            let err_msg = r.unwrap_err().to_string();
            #[cfg(feature = "ureq_socks_proxy")]
            assert!(
                err_msg.contains("Dns Failed") || err_msg.contains("Connection Failed"),
                "{}",
                err_msg
            );
            #[cfg(not(feature = "ureq_socks_proxy"))]
            assert!(err_msg.contains("SOCKS feature disabled"), "{}", err_msg);
        });
    }

    #[test]
    fn weak_multi_test() {
        TEST_RQLITED_DB.run_test(|| {
            let r = Request::<Post>::new().run(
                &Query::new(&TEST_CONNECTION)
                    .set_weak()
                    .push_sql_str("SELECT 1")
                    .push_sql_str("SELECT date()"),
            );

            assert!(r.is_ok(), "response error: {}", r.err().unwrap());

            let r = r.unwrap();
            let mut results = r.results();
            let result = results.next().unwrap();

            match result {
                Result::Standard(result) => {
                    assert_eq!(
                        result,
                        &result::Standard {
                            columns: vec!["1".to_string()],
                            time: None,
                            types: vec![DataType::Integer],
                            values: vec![vec![1.into()]]
                        }
                    );
                }
                _ => unreachable!(),
            }

            let result = results.next().unwrap();

            match result {
                Result::Standard(result) => {
                    assert_eq!(
                        result,
                        &result::Standard {
                            columns: vec!["date()".to_string()],
                            time: None,
                            types: vec![DataType::Text],
                            values: vec![vec![time::OffsetDateTime::now_utc()
                                .format(
                                    &time::format_description::parse("[year]-[month]-[day]")
                                        .unwrap()
                                )
                                .unwrap()
                                .into()]]
                        }
                    );
                }
                _ => unreachable!(),
            }
        });
    }
}

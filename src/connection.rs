//! Create and configure your [`Connection`] and get a [`Query`]

#[cfg(feature = "url")]
use std::str::FromStr;

use crate::query::{state, Endpoint, Query};
use crate::{log, tracing};

pub(crate) use scheme::Scheme;

mod scheme;

/// Create and configure your [`Connection`] and get a [`Query`]
///
/// This is a connection builder.
///
/// # Usage
///
/// Create a new `Connection`
///
/// ```no_run
/// let mut con = rqlite_client::Connection::new("http://localhost:4001");
/// #[cfg(feature = "url")]
/// let mut con = con.unwrap();
///
/// ```
///
/// Set optionally a proxy address to use for the request
///
/// ```no_run
/// # let mut con = rqlite_client::Connection::new("http://localhost:4001");
/// # #[cfg(feature = "url")]
/// # let mut con = con.unwrap();
/// con = con.set_proxy("http://proxy.example.com:8080");
/// ```
///
/// And retrieve a [`Query`] builder instance to start working with the database
///
/// ```no_run
/// # let mut con = rqlite_client::Connection::new("http://localhost:4001");
/// # #[cfg(feature = "url")]
/// # let mut con = con.unwrap();
/// # con = con.set_proxy("http://proxy.example.com:8080");
/// let query = con.execute()
///     .enable_transaction()
///     .push_sql_str("CREATE TEMP TABLE tmp")
///     .push_sql_str("CREATE TEMP TABLE tmp2")
///     .set_timing()
///     .set_timeout(std::time::Duration::from_secs(2));
///
/// // Running `Query` needs a working `RequestBuilder` implementation
/// #[cfg(feature = "ureq")]
/// {
///     let response_result = query.request_run();
///     // or for starting with `Request` implementing `RequestBuilder` (same result)
///     use crate::rqlite_client::RequestBuilder;
///     let response_result =
///         rqlite_client::Request::<rqlite_client::request_type::Post>::new().run(&query);
///     // tables should've been created
///
///     // try again - query can be modified and run multiple times
///     let response_result = query.request_run();
///     // should be error, because tables already exist
/// }
/// ```
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Connection {
    proxy: Option<String>,
    scheme: Scheme,
    #[cfg(feature = "url")]
    url: url::Url,
    #[cfg(not(feature = "url"))]
    url: String,
}

impl Connection {
    /// Create new [`Connection`]
    ///
    /// # Errors
    ///
    /// `url::ParseError` if `url` is no `url::Url`
    #[cfg(feature = "url")]
    #[inline]
    pub fn new(url: &str) -> Result<Self, url::ParseError> {
        let url = url::Url::parse(url)?;
        let scheme = url.scheme();

        let scheme = if scheme == "https" {
            Scheme::Https
        } else if scheme == "http" {
            Scheme::Http
        } else {
            Scheme::default()
        };

        Ok(Self {
            proxy: Self::detect_proxy(),
            scheme,
            url,
        })
    }
    /// Create new [`Connection`]
    #[cfg(not(feature = "url"))]
    #[must_use]
    #[inline]
    pub fn new(url: &str) -> Self {
        let scheme = if url.starts_with("https:") {
            Scheme::Https
        } else if url.starts_with("http:") {
            Scheme::Http
        } else {
            Scheme::default()
        };

        Self {
            proxy: Self::detect_proxy(),
            scheme,
            url: url.to_string(),
        }
    }

    /// Retrieve `Query` instance for queries with write capability (_CREATE/INSERT_ statements)
    ///
    /// See <https://rqlite.io/docs/api/api/#writing-data>
    ///
    #[must_use]
    #[inline]
    pub fn execute(&self) -> Query<state::NoLevelMulti> {
        log::debug!("query[execute]: {:?}", self);
        tracing::debug!("query[execute]: {:?}", self);

        Query::new(self)
            .set_endpoint(Endpoint::Execute)
            .switch_multi()
    }

    /// Get proxy
    #[must_use]
    #[inline]
    pub fn proxy(&self) -> Option<&str> {
        self.proxy.as_deref()
    }

    /// Retrieve `Query` instance for queries with read capability (_SELECT_ statements)
    #[must_use]
    #[inline]
    pub fn query(&self) -> Query<state::NoLevel> {
        log::debug!("query: {:?}", self);
        tracing::debug!("query: {:?}", self);

        Query::new(self)
    }

    /// Retrieve `Query` instance for queries with read and write capability (every combination of statement)
    ///
    /// See <https://rqlite.io/docs/api/api/#unified-endpoint>
    ///
    #[must_use]
    #[inline]
    pub fn request(&self) -> Query<state::NoLevelMulti> {
        log::debug!("query[request]: {:?}", self);
        tracing::debug!("query[request]: {:?}", self);

        Query::new(self)
            .set_endpoint(Endpoint::Request)
            .switch_multi()
    }

    /// Set proxy
    #[must_use]
    #[inline]
    pub fn set_proxy(mut self, proxy: &str) -> Self {
        self.proxy = Some(proxy.to_string());
        self
    }

    /// Get scheme of url
    #[must_use]
    #[inline]
    pub fn scheme(&self) -> Scheme {
        self.scheme
    }

    #[cfg(feature = "url")]
    #[inline]
    pub(crate) fn url(&self) -> &'_ url::Url {
        &self.url
    }

    #[cfg(not(feature = "url"))]
    #[inline]
    pub(crate) fn url(&self) -> &'_ str {
        &self.url
    }

    #[allow(clippy::similar_names)]
    pub(crate) fn detect_proxy() -> Option<String> {
        let env_http_proxy = std::env::var("HTTP_PROXY").ok();
        let env_https_proxy = std::env::var("HTTPS_PROXY").ok();
        let env_all_proxy = std::env::var("ALL_PROXY").ok();
        if env_https_proxy.is_some() {
            env_https_proxy
        } else if env_http_proxy.is_some() {
            env_http_proxy
        } else if env_all_proxy.is_some() {
            env_all_proxy
        } else {
            None
        }
    }
}

const DEFAULT_CONNECTION_URL: &str = "http://localhost:4001/";

impl Default for Connection {
    fn default() -> Self {
        Self {
            proxy: Self::detect_proxy(),
            scheme: Scheme::default(),
            #[cfg(feature = "url")]
            url: url::Url::parse(DEFAULT_CONNECTION_URL).unwrap(),
            #[cfg(not(feature = "url"))]
            url: DEFAULT_CONNECTION_URL.to_string(),
        }
    }
}

#[cfg(feature = "url")]
impl TryFrom<&str> for Connection {
    type Error = url::ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Connection::new(value)
    }
}

#[cfg(feature = "url")]
impl FromStr for Connection {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Connection::new(s)
    }
}

#[cfg(not(feature = "url"))]
impl From<&str> for Connection {
    fn from(value: &str) -> Self {
        Connection::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Connection;
    use super::Scheme;

    #[cfg(feature = "url")]
    #[test]
    fn scheme_test() {
        let c = Connection::new("http://example.com").unwrap();
        assert_eq!(c.scheme(), Scheme::Http);

        let c = Connection::new("https://example.com").unwrap();
        assert_eq!(c.scheme(), Scheme::Https);
    }

    #[cfg(not(feature = "url"))]
    #[test]
    fn scheme_test() {
        let c = Connection::new("http://example.com");
        assert_eq!(c.scheme(), Scheme::Http);

        let c = Connection::new("https://example.com");
        assert_eq!(c.scheme(), Scheme::Https);
    }
}

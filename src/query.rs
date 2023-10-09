//! Builder for the SQL statement [`Query`]

use std::{cell::RefCell, marker::PhantomData, time::Duration};

use crate::{log, tracing};
use crate::{Connection, Value};
pub(crate) use consistency_level::ConsistencyLevel;
pub(crate) use endpoint::Endpoint;
pub(crate) use freshness::Freshness;
pub(crate) use state::State;
pub(crate) use timeout::Timeout;

pub mod consistency_level;
mod duration_string;
pub mod endpoint;
pub mod freshness;
pub mod state;
pub mod timeout;
mod varparam_macro;

/**
Builder for the SQL statement [`Query`]

The preferred way to create a `Query` is via _methods_ of the [`Connection`], depending of the intention:

* [`Connection::execute()`](./struct.Connection.html#method.execute)
* [`Connection::query()`](./struct.Connection.html#method.query)
* [`Connection::request()`](./struct.Connection.html#method.request)

A `Query` will be transitioned between different consistency state levels:

* `NoLevel`
* `LevelNone`
* `LevelStrong`
* `LevelWeak`

`NoLevel` and `LevelWeak` have the same effect for the `Query` and its `Response`.

If there are pushed multiple SQL statements, there are state levels ending with _Multi_.
These queries need to use an _HTTP POST_ request to the database backend.

For more information about the state levels, see [`ConsistencyLevel`].

To set __SQL statements__ use the most appropriate _method_ of the `set_sql_...()` or `push_sql_...()` group.

For being safe against [SQL injection](https://owasp.org/www-community/attacks/SQL_Injection) preferable use
parameterized statements.

For convenience with different types of parameters, there is the [`crate::varparam!`]-macro.

*/
#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Query<'a, T>
where
    T: State,
{
    connection: &'a Connection,
    consistency_level: Option<ConsistencyLevel>,
    endpoint: Endpoint,
    freshness: Option<Freshness>,
    is_associative: bool,
    is_noleader: bool,
    is_nonvoters: bool,
    is_pretty: bool,
    is_queue: bool,
    is_redirect: bool,
    is_timing: bool,
    is_transaction: bool,
    is_url_modified: bool,
    is_wait: bool,
    sql: Vec<Value>,
    state: PhantomData<T>,
    timeout: Option<Timeout>,
    timeout_request: Option<Duration>,
    #[cfg(feature = "url")]
    url_cache: RefCell<Option<url::Url>>,
    #[cfg(not(feature = "url"))]
    url_cache: RefCell<Option<String>>,
}

impl<'a, T> Query<'a, T>
where
    T: State,
{
    /// [`Connection`] of `Query`
    #[must_use]
    #[inline]
    pub fn connection(&self) -> &'a Connection {
        self.connection
    }

    /// [`ConsistencyLevel`] of `Query` or `None`
    #[must_use]
    #[inline]
    pub fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level
    }

    /// Disable automatic redirect forwarding
    ///
    /// See <https://rqlite.io/docs/api/api/#disabling-request-forwarding>
    ///
    #[must_use]
    #[inline]
    pub fn disable_redirect(mut self) -> Self {
        if self.is_redirect {
            self.is_redirect = false;
            log::trace!("is_redirect: {}", self.is_redirect);
            tracing::trace!("is_redirect: {}", self.is_redirect);
            self.url_modified()
        } else {
            self
        }
    }

    /// Check for associative `Query`
    ///
    /// See <https://rqlite.io/docs/api/api/#associative-response-form>
    ///
    #[must_use]
    #[inline]
    pub fn is_associative(&self) -> bool {
        self.is_associative
    }

    /// Check for readiness `noleader` query flag status
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#readiness-checks>
    ///
    #[must_use]
    #[inline]
    pub fn is_noleader(&self) -> bool {
        self.is_noleader
    }

    /// Check for nodes `nonvoter` query flag status
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#nodes-api>
    ///
    #[must_use]
    #[inline]
    pub fn is_nonvoters(&self) -> bool {
        self.is_nonvoters
    }

    /// Check for pretty `Query`
    ///
    /// The use of the URL param pretty is optional, and results in pretty-printed JSON responses.  
    /// Makes only sense during debug, because it is more verbose in memory and speed.
    ///
    #[must_use]
    #[inline]
    pub fn is_pretty(&self) -> bool {
        self.is_pretty
    }

    /// Check for queued `Query`
    ///
    /// __rqlite__ exposes a special API flag, which will instruct rqlite to queue up write-requests
    /// and execute them asynchronously.  
    /// rqlite will merge the requests, once a batch-size of them has been queued on the node or a
    /// configurable timeout expires, and execute them as though they had been both contained in a
    /// single request.
    ///
    /// Each response includes a monotonically-increasing `sequence_number`, which allows to track
    /// the persisting of the data with the [`Endpoint::Status`](super::monitor::Endpoint::Status).
    ///
    /// With [`Query::is_wait()`] true, the data has been persisted when the
    /// [`response::Result`](crate::response::Result) is available and successful.
    ///
    /// See <https://rqlite.io/docs/api/queued-writes/>
    ///
    #[must_use]
    #[inline]
    pub fn is_queue(&self) -> bool {
        self.is_queue
    }

    /// Check for automatic redirect forwarding [[default: true]]
    ///
    /// See <https://rqlite.io/docs/api/api/#disabling-request-forwarding>
    ///
    /// The url path param indicates a disabled redirect, but we use it
    /// here in the sense of the word.  
    /// So when path param should be set, this needs to return `false`.
    ///
    #[must_use]
    #[inline]
    pub fn is_redirect(&self) -> bool {
        self.is_redirect
    }

    /// Check for timing `Query`
    ///
    /// Time is measured in seconds. If you do not want timings, do not pass timings as a URL parameter.
    ///
    #[must_use]
    #[inline]
    pub fn is_timing(&self) -> bool {
        self.is_timing
    }

    /// Check for enabled transaction
    ///
    /// A form of transactions are supported. To execute statements within a transaction, add transaction to the URL.
    ///
    /// When a transaction takes place either all statements of a `Query` will succeed, or neither.
    /// Performance is much, much better if multiple SQL INSERTs or UPDATEs are executed via a transaction.
    /// Note that processing of the request ceases the moment any single query results in an error.
    ///
    /// The behaviour of rqlite if you explicitly issue BEGIN, COMMIT, ROLLBACK, SAVEPOINT, and RELEASE to control
    /// your own transactions is __not defined__.  
    /// This is because the behavior of a cluster if it fails while such a manually-controlled transaction is not
    /// yet defined.
    ///
    /// It is __important__ to control transactions __only__ through this query parameter.
    ///
    /// Source: <https://rqlite.io/docs/api/api/#transactions>
    ///
    #[must_use]
    #[inline]
    pub fn is_transaction(&self) -> bool {
        self.is_transaction
    }

    /// Check for awaited queued `Query`
    ///
    /// When [`Query::is_queue()`] true, rqlite will merge the requests, once a batch-size of them has been queued
    /// on the node or a configurable timeout expires, and execute them as though they had been both contained in
    /// a single request.
    ///
    /// The `wait` can explicitly tell the request to wait until the queue has persisted all pending requests.
    ///
    /// See <https://rqlite.io/docs/api/queued-writes/#waiting-for-a-queue-to-flush>
    ///
    #[must_use]
    #[inline]
    pub fn is_wait(&self) -> bool {
        self.is_wait
    }

    /// Run `Request` for `Query`
    ///
    /// # Errors
    ///
    /// [Error](crate::Error) on failing [Request](crate::request::Request) run
    ///
    #[cfg(feature = "ureq")]
    pub fn request_run(&self) -> crate::response::Result {
        use crate::request_builder::RequestBuilder;
        match self.endpoint {
            #[cfg(feature = "monitor")]
            Endpoint::Query | Endpoint::Monitor(_) => {
                crate::request::Request::<crate::request::request_type::Get>::new().run(self)
            }
            #[cfg(not(feature = "monitor"))]
            Endpoint::Query => {
                crate::request::Request::<crate::request::request_type::Get>::new().run(self)
            }
            Endpoint::Execute | Endpoint::Request => {
                crate::request::Request::<crate::request::request_type::Post>::new().run(self)
            }
        }
    }

    /// Check for associative `Query`
    ///
    /// See <https://rqlite.io/docs/api/api/#associative-response-form>
    ///
    #[must_use]
    #[inline]
    pub fn set_associative(mut self) -> Self {
        if self.is_associative {
            self
        } else {
            self.is_associative = true;
            log::trace!("is_associative: {}", self.is_associative);
            tracing::trace!("is_associative: {}", self.is_associative);
            self.url_modified()
        }
    }

    /// Change [`Endpoint`]
    #[must_use]
    #[inline]
    pub(crate) fn set_endpoint(mut self, endpoint: Endpoint) -> Self {
        if self.endpoint == endpoint {
            self
        } else {
            self.endpoint = endpoint;
            log::trace!("endpoint: {:?}", self.endpoint);
            tracing::trace!("endpoint: {:?}", self.endpoint);
            self.url_modified()
        }
    }

    /// Set pretty `Query`
    ///
    /// The use of the URL param pretty is optional, and results in pretty-printed JSON responses.
    /// Makes only sense during debug, because it is more verbose in memory and speed.
    ///
    #[must_use]
    #[inline]
    pub fn set_pretty(mut self) -> Self {
        if self.is_pretty {
            self
        } else {
            self.is_pretty = true;
            log::trace!("is_pretty: {}", self.is_pretty);
            tracing::trace!("is_pretty: {}", self.is_pretty);
            self.url_modified()
        }
    }

    /// Set `timeout` for `Query` response
    ///
    /// See <https://rqlite.io/docs/api/api/#request-forwarding-timeouts>
    /// and
    /// see <https://rqlite.io/docs/api/queued-writes/#waiting-for-a-queue-to-flush>
    ///
    #[must_use]
    #[inline]
    pub fn set_timeout(mut self, timeout: Timeout) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set `timeout_request` for HTTP request of `Query`
    #[must_use]
    #[inline]
    pub fn set_timeout_request(mut self, timeout_request: Duration) -> Self {
        self.timeout_request = Some(timeout_request);
        self
    }

    /// Set timing in `Query`
    ///
    /// Time is measured in seconds. If you do not want timings, do not pass timings as a URL parameter.
    ///
    #[must_use]
    #[inline]
    pub fn set_timing(mut self) -> Self {
        if self.is_timing {
            self
        } else {
            self.is_timing = true;
            log::trace!("is_timing: {}", self.is_timing);
            tracing::trace!("is_timing: {}", self.is_timing);
            self.url_modified()
        }
    }

    /// Get SQL statements
    #[inline]
    pub fn sql(&self) -> &'_ Vec<Value> {
        &self.sql
    }

    /// Get optional `timeout` for `Query` response
    #[inline]
    pub fn timeout(&self) -> Option<&Timeout> {
        self.timeout.as_ref()
    }

    /// Get optional `timeout` for HTTP request of `Query`
    #[inline]
    pub fn timeout_request(&self) -> Option<&Duration> {
        self.timeout_request.as_ref()
    }

    #[inline]
    fn create_url_query(&self) -> String {
        let mut query_args = Vec::new();

        if self.is_associative {
            query_args.push("associative".to_string());
        }

        if self.is_noleader {
            query_args.push("noleader".to_string());
        }

        if self.is_nonvoters {
            query_args.push("nonvoters".to_string());
        }

        if self.is_pretty {
            query_args.push("pretty".to_string());
        }

        if self.is_queue {
            query_args.push("queue".to_string());
        }

        if !self.is_redirect {
            query_args.push("redirect".to_string());
        }

        if self.is_timing {
            query_args.push("timing".to_string());
        }

        if self.is_transaction {
            query_args.push("transaction".to_string());
        }

        if self.is_wait {
            query_args.push("wait".to_string());
        }

        if let Some(timeout) = self.timeout {
            query_args.push(format!("timeout={timeout}"));
        }

        if let Some(consistency_level) = self.consistency_level {
            if consistency_level != ConsistencyLevel::Nolevel {
                query_args.push(format!("level={consistency_level}"));
                if consistency_level == ConsistencyLevel::None {
                    if let Some(freshness) = self.freshness {
                        query_args.push(format!("freshness={freshness}"));
                    }
                }
            }
        }

        let mut query = String::new();

        for (i, arg) in query_args.iter().enumerate() {
            if i != 0 {
                query.push('&');
            }
            query.push_str(arg);
        }

        if self.endpoint == Endpoint::Query && self.sql.len() == 1 {
            if let Some(sql) = self.sql.get(0) {
                if let Some(json_query) = sql.as_str() {
                    if query_args.is_empty() {
                        query.push_str("q=");
                    } else {
                        query.push_str("&q=");
                    }

                    // Need percent_encoding if it is not included in
                    // using `Url#set_query`/`Url#query`
                    #[cfg(feature = "percent_encoding")]
                    #[cfg(not(feature = "url"))]
                    let json_query = &percent_encoding::utf8_percent_encode(
                        json_query,
                        percent_encoding::NON_ALPHANUMERIC,
                    )
                    .to_string();

                    log::debug!("q: {json_query}");
                    tracing::debug!("q: {json_query}");

                    query.push_str(json_query);
                } else {
                    log::debug!("q: <None>");
                    tracing::debug!("q: <None>");
                }
            }
        }

        query
    }

    #[cfg(feature = "url")]
    fn create_url(&self) -> url::Url {
        if self.is_url_modified || self.url_cache.borrow().is_none() {
            let mut url = self.connection.url().clone();

            url.set_path(&self.endpoint.to_string());
            let query = self.create_url_query();
            url.set_query(if query.is_empty() { None } else { Some(&query) });

            *self.url_cache.borrow_mut() = Some(url);
        }

        // checked in `if` above and set to Not-None
        self.url_cache.borrow().clone().unwrap()
    }

    #[cfg(not(feature = "url"))]
    fn create_url(&self) -> String {
        if self.is_url_modified || self.url_cache.borrow().is_none() {
            let mut url = self.connection.url().to_string();

            let endpoint = self.endpoint.to_string();

            if endpoint.starts_with('/') && url.ends_with('/') {
                let _ = url.pop();
            }

            url.push_str(&endpoint);
            let query = self.create_url_query();
            if !query.is_empty() {
                url.push('?');
                url.push_str(&query);
            }

            *self.url_cache.borrow_mut() = Some(url);
        }

        // checked in `if` above and set to Not-None
        self.url_cache.borrow().clone().unwrap()
    }

    /// Enable noleader query param
    #[cfg(feature = "monitor")]
    #[must_use]
    #[inline]
    pub(crate) fn enable_noleader_helper(mut self) -> Self {
        if self.is_noleader {
            self
        } else {
            self.is_noleader = true;
            log::trace!("is_noleader: {}", self.is_noleader);
            tracing::trace!("is_noleader: {}", self.is_noleader);
            self.url_modified()
        }
    }

    /// Enable nonvoters query param
    #[cfg(feature = "monitor")]
    #[must_use]
    #[inline]
    pub(crate) fn enable_nonvoters_helper(mut self) -> Self {
        if self.is_nonvoters {
            self
        } else {
            self.is_nonvoters = true;
            log::trace!("is_nonvoters: {}", self.is_nonvoters);
            tracing::trace!("is_nonvoters: {}", self.is_nonvoters);
            self.url_modified()
        }
    }

    #[inline]
    fn enable_transaction_helper(mut self) -> Self {
        if self.endpoint == Endpoint::Query {
            self
        } else {
            self.is_transaction = true;
            log::trace!("is_transaction: {}", self.is_transaction);
            tracing::trace!("is_transaction: {}", self.is_transaction);
            self.url_modified()
        }
    }

    #[inline]
    fn set_freshness_helper(mut self, freshness: impl Into<Freshness>) -> Self {
        let freshness = Some(freshness.into());
        if self.freshness == freshness {
            self
        } else {
            self.freshness = freshness;
            log::trace!("freshness: {:?}", self.freshness);
            tracing::trace!("freshness: {:?}", self.freshness);
            self.url_modified()
        }
    }

    #[inline]
    fn set_sql_helper(mut self, sql: Value) -> Self {
        let self_sql: &mut Vec<Value> = self.sql.as_mut();
        if !self_sql.is_empty() {
            self_sql.clear();
        }

        self = if self.endpoint == Endpoint::Query {
            log::trace!("sql: {:?}", sql);
            tracing::trace!("sql: {:?}", sql);

            self_sql.push(sql);
            self.url_modified()
        } else {
            panic!("set_sql only Query");
        };
        self
    }

    #[inline]
    fn set_sql_str_helper(self, sql: &str) -> Self {
        self.set_sql_helper(sql.into())
    }

    #[inline]
    fn set_sql_str_slice_helper(self, sql: &[&str]) -> Self {
        self.set_sql_helper(sql.into())
    }

    #[inline]
    fn set_sql_value_slice_helper<V>(self, sql: &[V]) -> Self
    where
        V: Into<Value> + Clone,
    {
        let v: Value = sql
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<Value>>()
            .into();
        self.set_sql_helper(v)
    }

    #[inline]
    fn push_sql_helper(self, sql: Value) -> Self {
        let mut self_mod = if self.endpoint == Endpoint::Query && self.sql.len() <= 1 {
            self.url_modified()
        } else {
            self
        };

        let sql = if sql.is_string() {
            Value::Array(vec![sql])
        } else {
            sql
        };

        log::trace!("sql: {:?}", sql);
        tracing::trace!("sql: {:?}", sql);
        self_mod.sql.push(sql);
        self_mod
    }

    #[inline]
    fn push_sql_str_helper(self, sql: &str) -> Self {
        self.push_sql_helper(sql.into())
    }

    #[inline]
    fn push_sql_str_slice_helper(self, sql: &[&str]) -> Self {
        self.push_sql_helper(sql.into())
    }

    #[inline]
    fn push_sql_value_slice_helper<V>(self, sql: &[V]) -> Self
    where
        V: Into<Value> + Clone,
    {
        let v: Value = sql
            .iter()
            .cloned()
            .map(Into::into)
            .collect::<Vec<Value>>()
            .into();
        self.push_sql_helper(v)
    }

    /// used to mark any modification which changes the url
    #[inline]
    fn url_modified(mut self) -> Self {
        self.is_url_modified = true;
        log::trace!("is_url_modified: {}", self.is_url_modified);
        tracing::trace!("is_url_modified: {}", self.is_url_modified);
        self
    }

    /// For cfg(test) ONLY
    #[cfg(test)]
    #[cfg(feature = "url")]
    #[inline]
    pub(crate) fn create_path_with_query(&self) -> String {
        let mut s = self.create_url().path().to_string();
        if let Some(query) = self.create_url().query() {
            s.push('?');
            s.push_str(query);
        }
        s
    }

    /// For cfg(test) ONLY
    #[cfg(test)]
    #[cfg(feature = "percent_encoding")]
    #[cfg(not(feature = "url"))]
    #[inline]
    pub(crate) fn create_path_with_query(&self) -> String {
        let mut s = self.endpoint.to_string();
        let query = self.create_url_query();
        if !query.is_empty() {
            s.push('?');
            s.push_str(&query);
        }
        s
    }
}

impl<T> std::fmt::Display for Query<'_, T>
where
    T: State,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.create_url().as_str())
    }
}

#[inline]
fn transition<S, T>(
    src: Query<'_, S>,
    consistency_level: ConsistencyLevel,
    freshness: Option<Freshness>,
) -> Query<'_, T>
where
    S: State,
    T: State,
{
    Query {
        connection: src.connection,
        consistency_level: Some(consistency_level),
        endpoint: src.endpoint,
        freshness,
        is_associative: src.is_associative,
        is_noleader: src.is_noleader,
        is_nonvoters: src.is_nonvoters,
        is_pretty: src.is_pretty,
        is_queue: src.is_queue,
        is_redirect: src.is_redirect,
        is_timing: src.is_timing,
        is_transaction: src.is_transaction,
        is_url_modified: src.is_url_modified,
        is_wait: src.is_wait,
        sql: src.sql,
        state: PhantomData,
        timeout: src.timeout,
        timeout_request: src.timeout_request,
        url_cache: src.url_cache,
    }
}

/// Generate methods for `Query` impls
///
macro_rules! gen_query {
    ($level_in:path, $level_out:path) => {
        #[doc = concat!("`Query<", stringify!($level_in), ">`\n\nSee [`Query`]\n\n")]
        impl<'a> Query<'a, $level_in> {
            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_in), ">`\n\n# Panics\n\nIf `consistency_level` is not set before (report internal bug)")]
            #[must_use]
            pub fn push_sql(self, sql: Value) -> Query<'a, $level_out> {
                let self_mod = self.push_sql_helper(sql);
                let consistency_level = self_mod.consistency_level.unwrap();
                let freshness = self_mod.freshness;
                transition(self_mod, consistency_level, freshness)
            }

            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_in), ">`\n\n# Panics\n\nIf `consistency_level` is not set before (report internal bug)")]
            #[must_use]
            pub fn push_sql_str(self, sql: &str) -> Query<'a, $level_out> {
                self.push_sql(sql.into())
            }

            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_in), ">`\n\n# Panics\n\nIf `consistency_level` is not set before (report internal bug)")]
            #[must_use]
            pub fn push_sql_str_slice(self, sql: &[&str]) -> Query<'a, $level_out> {
                self.push_sql(sql.into())
            }

            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_in), ">`\n\n# Panics\n\nIf `consistency_level` is not set before (report internal bug)")]
            #[must_use]
            pub fn push_sql_values<V>(self, sql: &[V])  -> Query<'a, $level_out> where V: Into<Value> + Clone {
                let self_mod = self.push_sql_value_slice_helper(sql);
                let consistency_level = self_mod.consistency_level.unwrap();
                let freshness = self_mod.freshness;
                transition(self_mod, consistency_level, freshness)
            }

            #[doc = concat!("Set a given `sql` to the `Query<", stringify!($level_in), ">`")]
            #[must_use]
            pub fn set_sql(self, sql: Value) -> Self {
                self.set_sql_helper(sql)
            }

            #[doc = concat!("Set a given `sql` to the `Query<", stringify!($level_in), ">`")]
            #[must_use]
            pub fn set_sql_str(self, sql: &str) -> Self {
                self.set_sql_str_helper(sql)
            }

            #[doc = concat!("Set a given `sql` to the `Query<", stringify!($level_in), ">`")]
            #[must_use]
            pub fn set_sql_str_slice(self, sql: &[&str]) -> Self {
                self.set_sql_str_slice_helper(sql)
            }

            #[doc = concat!("Set a given `sql` to the `Query<", stringify!($level_in), ">`")]
            #[must_use]
            pub fn set_sql_values<V>(self, sql: &[V]) -> Self where V: Into<Value> + Clone {
                self.set_sql_value_slice_helper(sql)
            }
        }

        #[doc = concat!("`Query<", stringify!($level_out), ">`\n\nSee [`Query`]\n\n")]
        impl<'a> Query<'a, $level_out> {
            #[doc = "Enable transaction\n\n\
                    A form of transactions are supported. To execute statements within a transaction, add transaction to the URL.\n\n\
                    When a transaction takes place either all statements of a `Query` will succeed, or neither.  \n\
                    Performance is much, much better if multiple SQL INSERTs or UPDATEs are executed via a transaction.  \n\
                    Note that processing of the request ceases the moment any single query results in an error.\n\n\
                    The behaviour of rqlite if you explicitly issue BEGIN, COMMIT, ROLLBACK, SAVEPOINT, and RELEASE to control\
                    your own transactions is __not defined__.  \n\
                    This is because the behavior of a cluster if it fails while such a manually-controlled transaction is not\
                    yet defined.\n\n\
                    It is __important__ to control transactions __only__ through this query parameter.\n\n\
                    Source: <https://rqlite.io/docs/api/api/#transactions>\n\n"]
            #[must_use]
            pub fn enable_transaction(self) -> Self {
                self.enable_transaction_helper()
            }

            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_out), ">`")]
            #[must_use]
            pub fn push_sql(self, sql: Value) -> Self {
                self.push_sql_helper(sql)
            }

            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_out), ">`")]
            #[must_use]
            pub fn push_sql_str(self, sql: &str) -> Self {
                self.push_sql_str_helper(sql)
            }

            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_out), ">`")]
            #[must_use]
            pub fn push_sql_str_slice(self, sql: &[&str]) -> Self {
                self.push_sql_str_slice_helper(sql)
            }

            #[doc = concat!("Append a given `sql` to the `Query<", stringify!($level_out), ">`")]
            #[must_use]
            pub fn push_sql_values<V>(self, sql: &[V]) -> Self where V: Into<Value> + Clone {
                self.push_sql_value_slice_helper(sql)
            }
        }
    };
}

/// Generate freshness methods for `Query` impl
macro_rules! gen_query_freshness {
    ( $($level:path),+ ) => {
        $(
            #[doc = concat!("`Query<", stringify!($level), ">`\n\nSee [`Query`]\n\n")]
            impl<'a> Query<'a, $level> {
                #[doc = concat!("`Freshness` of `Query<", stringify!($level), ">` or `None`\n\n")]
                #[doc = "The amount of time the database _node_ is allowed for not checking the _leader_ for\ndata accuracy.  \n"]
                #[doc = "If data is _stale_, there will be an error response.\n\nSee [`Freshness`] for more details."]
                #[must_use]
                #[inline]
                pub fn freshness(&self) -> Option<Freshness> {
                    self.freshness
                }

                #[doc = concat!("Set `Freshness` of `Query<", stringify!($level), ">`\n\n")]
                #[doc = "Sets the amount of time the database _node_ is allowed for not checking the _leader_ for\ndata accuracy.  \n"]
                #[doc = "If data is _stale_, there will be an error response.\n\nSee [`Freshness`] for more details."]
                #[must_use]
                pub fn set_freshness(self, freshness: impl Into<Freshness>) -> Self {
                    self.set_freshness_helper(freshness)
                }
            }
        )+
    };
}

gen_query!(state::LevelNone, state::LevelNoneMulti);
gen_query_freshness!(state::LevelNone, state::LevelNoneMulti);

/// `Query<LevelNone>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::LevelNone> {
    /// Set [`ConsistencyLevel::Strong`] for `Query`
    #[must_use]
    pub fn set_strong(self) -> Query<'a, state::LevelStrong> {
        transition(self, ConsistencyLevel::Strong, None)
    }

    /// Set [`ConsistencyLevel::Weak`] for `Query`
    #[must_use]
    pub fn set_weak(self) -> Query<'a, state::LevelWeak> {
        transition(self, ConsistencyLevel::Weak, None)
    }
}

/// `Query<LevelNoneMulti>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::LevelNoneMulti> {
    /// Set [`ConsistencyLevel::Strong`] for `Query`
    #[must_use]
    pub fn set_strong(self) -> Query<'a, state::LevelStrongMulti> {
        transition(self, ConsistencyLevel::Strong, None)
    }

    /// Set [`ConsistencyLevel::Weak`] for `Query`
    #[must_use]
    pub fn set_weak(self) -> Query<'a, state::LevelWeakMulti> {
        transition(self, ConsistencyLevel::Weak, None)
    }
}

gen_query!(state::LevelStrong, state::LevelStrongMulti);

/// `Query<LevelStrong>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::LevelStrong> {
    /// Set [`ConsistencyLevel::None`] for `Query`
    #[must_use]
    pub fn set_none(self) -> Query<'a, state::LevelNone> {
        transition(self, ConsistencyLevel::None, None)
    }

    /// Set [`ConsistencyLevel::Weak`] for `Query`
    #[must_use]
    pub fn set_weak(self) -> Query<'a, state::LevelWeak> {
        transition(self, ConsistencyLevel::Weak, None)
    }
}

/// `Query<LevelStrongMulti>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::LevelStrongMulti> {
    /// Set [`ConsistencyLevel::None`] for `Query`
    #[must_use]
    pub fn set_none(self) -> Query<'a, state::LevelNoneMulti> {
        transition(self, ConsistencyLevel::None, None)
    }

    /// Set [`ConsistencyLevel::Weak`] for `Query`
    #[must_use]
    pub fn set_weak(self) -> Query<'a, state::LevelWeakMulti> {
        transition(self, ConsistencyLevel::Weak, None)
    }
}

gen_query!(state::LevelWeak, state::LevelWeakMulti);

/// `Query<LevelWeak>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::LevelWeak> {
    /// Set [`ConsistencyLevel::None`] for `Query`
    #[must_use]
    pub fn set_none(self) -> Query<'a, state::LevelNone> {
        transition(self, ConsistencyLevel::None, None)
    }

    /// Set [`ConsistencyLevel::Strong`] for `Query`
    #[must_use]
    pub fn set_strong(self) -> Query<'a, state::LevelStrong> {
        transition(self, ConsistencyLevel::Strong, None)
    }
}

/// `Query<LevelWeakMulti>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::LevelWeakMulti> {
    /// Set [`ConsistencyLevel::None`] for `Query`
    #[must_use]
    pub fn set_none(self) -> Query<'a, state::LevelNoneMulti> {
        transition(self, ConsistencyLevel::None, None)
    }

    /// Set [`ConsistencyLevel::Strong`] for `Query`
    #[must_use]
    pub fn set_strong(self) -> Query<'a, state::LevelStrongMulti> {
        transition(self, ConsistencyLevel::Strong, None)
    }
}

gen_query!(state::NoLevel, state::NoLevelMulti);

/// `Query<NoLevel>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::NoLevel> {
    /// Get a `Query` to [`monitor::Monitor`](crate::monitor::Monitor) rqlited
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/>
    ///
    #[cfg(feature = "monitor")]
    pub fn monitor(self) -> Query<'a, crate::monitor::Monitor> {
        transition(
            self.set_endpoint(Endpoint::Monitor(crate::monitor::Endpoint::Status)),
            ConsistencyLevel::Nolevel,
            None,
        )
    }

    /// Create a new `Query`
    #[must_use]
    #[inline]
    pub(crate) fn new(connection: &'a Connection) -> Self {
        Self {
            connection,
            consistency_level: Some(ConsistencyLevel::default()),
            endpoint: Endpoint::default(),
            freshness: None,
            is_associative: false,
            is_noleader: false,
            is_nonvoters: false,
            is_pretty: false,
            is_queue: false,
            is_redirect: true,
            is_timing: false,
            is_transaction: false,
            is_url_modified: false,
            is_wait: false,
            sql: Vec::new(),
            state: PhantomData,
            timeout: None,
            timeout_request: None,
            url_cache: RefCell::new(None),
        }
    }

    /// Set [`ConsistencyLevel::None`] for `Query`
    #[must_use]
    pub fn set_none(self) -> Query<'a, state::LevelNone> {
        transition(self, ConsistencyLevel::None, None)
    }

    /// Set [`ConsistencyLevel::Strong`] for `Query`
    #[must_use]
    pub fn set_strong(self) -> Query<'a, state::LevelStrong> {
        transition(self, ConsistencyLevel::Strong, None)
    }

    /// Set [`ConsistencyLevel::Weak`] for `Query`
    #[must_use]
    pub fn set_weak(self) -> Query<'a, state::LevelWeak> {
        transition(self, ConsistencyLevel::Weak, None)
    }

    /// Switch to `state::NoLevelMulti`
    ///
    /// # Panics
    ///
    /// If `consistency_level` is not set before (report internal bug)
    #[must_use]
    pub fn switch_multi(self) -> Query<'a, state::NoLevelMulti> {
        let consistency_level = self.consistency_level.unwrap();
        let freshness = self.freshness;
        transition(self, consistency_level, freshness)
    }
}

/// `Query<NoLevelMulti>`
///
/// See [`Query`]
///
impl<'a> Query<'a, state::NoLevelMulti> {
    /// Set [`ConsistencyLevel::None`] for `Query`
    #[must_use]
    pub fn set_none(self) -> Query<'a, state::LevelNoneMulti> {
        transition(self, ConsistencyLevel::None, None)
    }

    /// Set __queued__ write
    #[must_use]
    #[inline]
    pub(crate) fn set_queue(mut self) -> Self {
        self.is_queue = true;
        self
    }

    /// Set [`ConsistencyLevel::Strong`] for `Query`
    #[must_use]
    pub fn set_strong(self) -> Query<'a, state::LevelStrongMulti> {
        transition(self, ConsistencyLevel::Strong, None)
    }

    /// Set `wait` for __queued__ write
    ///
    /// See [`Connection::execute_queue()`](./struct.Connection.html#method.execute_queue)
    ///
    #[must_use]
    #[inline]
    pub fn set_wait(mut self) -> Self {
        if self.is_queue {
            self.is_wait = true;
        } else {
            log::debug!("no queue write");
            tracing::debug!("no queue write");
        }
        self
    }

    /// Set [`ConsistencyLevel::Weak`] for `Query`
    #[must_use]
    pub fn set_weak(self) -> Query<'a, state::LevelWeakMulti> {
        transition(self, ConsistencyLevel::Weak, None)
    }
}

/// `Query<Monitor>`
///
/// Requires feature `monitor`.
///
/// See [`monitor::Monitor`](crate::monitor::Monitor)
///
#[cfg(feature = "monitor")]
impl<'a> Query<'a, crate::monitor::Monitor> {
    /// _Nodes_ return basic information for nodes in the cluster, as seen by the node
    /// receiving the nodes request. The receiving node will also check whether it can actually
    /// connect to all other nodes in the cluster.  
    /// This is an effective way to determine the cluster leader, and the leader’s HTTP API address.
    /// It can also be used to check if the cluster is basically running.
    /// If the other nodes are reachable, it probably is.
    ///
    /// By default, the node only checks if voting nodes are contactable.
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#nodes-api>
    ///
    pub fn nodes(self) -> Query<'a, crate::monitor::Nodes> {
        transition(
            self.set_endpoint(Endpoint::Monitor(crate::monitor::Endpoint::Nodes)),
            ConsistencyLevel::Nolevel,
            None,
        )
    }

    /// rqlite nodes serve a _ready_ status [`monitor::Endpoint::Readyz`](crate::monitor::Endpoint::Readyz)
    /// if the node is ready to respond to database requests and cluster management operations.
    ///
    /// If you wish to check if the node is running, and responding to HTTP requests, regardless of
    /// Leader status, `enable_noleader`.
    ///
    /// See <https://rqlite.io/docs/guides/monitoring-rqlite/#readiness-checks>
    ///
    pub fn readyz(self) -> Query<'a, crate::monitor::Readyz> {
        transition(
            self.set_endpoint(Endpoint::Monitor(crate::monitor::Endpoint::Readyz)),
            ConsistencyLevel::Nolevel,
            None,
        )
    }
}

#[cfg(test)]
#[cfg(any(feature = "percent_encoding", feature = "url"))]
mod tests {
    use std::time::Duration;

    use lazy_static::lazy_static;

    use crate::{query::Endpoint, varparam, Connection, Value};

    use super::Query;

    const TEST_CONNECTION_URL: &str = "http://localhost:4001/";

    #[cfg(feature = "url")]
    lazy_static! {
        static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL).unwrap();
    }
    #[cfg(not(feature = "url"))]
    lazy_static! {
        static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL);
    }

    #[test]
    fn set_sql_str_test() {
        let q = Query::new(&TEST_CONNECTION).set_sql_str("SELECT 1");
        #[cfg(feature = "url")]
        assert_eq!(&q.create_url_query(), "q=SELECT 1");
        #[cfg(not(feature = "url"))]
        assert_eq!(&q.create_url_query(), "q=SELECT%201");
        assert_eq!(&q.create_path_with_query(), "/db/query?q=SELECT%201");

        let q = q.set_sql_str("SELECT 2");
        #[cfg(feature = "url")]
        assert_eq!(&q.create_url_query(), "q=SELECT 2");
        #[cfg(not(feature = "url"))]
        assert_eq!(&q.create_url_query(), "q=SELECT%202");
        assert_eq!(&q.create_path_with_query(), "/db/query?q=SELECT%202");
    }

    #[test]
    fn set_sql_values_test() {
        let q = Query::new(&TEST_CONNECTION)
            .set_sql_values(&varparam!["SELECT COUNT(*) FROM test WHERE id = ?", 999]);
        #[cfg(feature = "url")]
        assert_eq!(&q.create_url_query(), "");
        #[cfg(not(feature = "url"))]
        assert_eq!(&q.create_url_query(), "");
        assert_eq!(&q.create_path_with_query(), "/db/query");

        let v: Value = q.sql().clone().into();
        assert_eq!(
            v.to_string(),
            "[[\"SELECT COUNT(*) FROM test WHERE id = ?\",999]]"
        );

        let q = Query::new(&TEST_CONNECTION).set_sql_values(&[
            "SELECT COUNT(*) FROM test WHERE id = ? or id = ?",
            "111",
            "222",
        ]);
        #[cfg(feature = "url")]
        assert_eq!(&q.create_url_query(), "");
        #[cfg(not(feature = "url"))]
        assert_eq!(&q.create_url_query(), "");
        assert_eq!(&q.create_path_with_query(), "/db/query");

        let v: Value = q.sql().clone().into();
        assert_eq!(
            v.to_string(),
            "[[\"SELECT COUNT(*) FROM test WHERE id = ? or id = ?\",\"111\",\"222\"]]"
        );
    }

    #[test]
    fn set_sql_values_escape_test() {
        let q = Query::new(&TEST_CONNECTION).set_sql_values(&[
            "SELECT COUNT(*) FROM test WHERE id = ? or id = ?",
            "1\"; SELECT * FROM secret; SELECT * FROM test WHERE id = 1",
            "222",
        ]);

        let v: Value = q.sql().clone().into();
        assert_eq!(
            v.to_string(),
            "[[\"SELECT COUNT(*) FROM test WHERE id = ? or id = ?\",\"1\\\"; SELECT * FROM secret; SELECT * FROM test WHERE id = 1\",\"222\"]]"
        );
    }

    #[test]
    fn basic_auth_test() {
        let c = Connection::new("http://user:password@example.com");
        #[cfg(feature = "url")]
        let c = c.unwrap();

        let url = c.query().create_url();
        #[cfg(feature = "url")]
        let url = url.to_string();

        assert_eq!(&url, "http://user:password@example.com/db/query");
    }

    #[test]
    fn nolevel_test() {
        let q = Query::new(&TEST_CONNECTION);
        assert_eq!(&q.create_path_with_query(), "/db/query");
        assert!(q.sql().is_empty());

        let q = Query::new(&TEST_CONNECTION).set_sql_str("SELECT 1");
        #[cfg(feature = "url")]
        assert_eq!(&q.create_url_query(), "q=SELECT 1");
        #[cfg(not(feature = "url"))]
        assert_eq!(&q.create_url_query(), "q=SELECT%201");
        assert_eq!(&q.create_path_with_query(), "/db/query?q=SELECT%201");
    }

    #[test]
    fn nolevelmulti_params_test() {
        let s = "SELECT 1";
        let q = Query::new(&TEST_CONNECTION).set_sql_str(s);
        assert_eq!(&q.create_path_with_query(), "/db/query?q=SELECT%201");
        assert_eq!(q.sql().len(), 1);
        assert_eq!(q.sql()[0].as_str().unwrap(), s);

        let q = Query::new(&TEST_CONNECTION)
            .push_sql(serde_json::from_str("[\"SELECT ?\", 1]").expect("array"));
        assert_eq!(&q.create_path_with_query(), "/db/query");
        assert_eq!(q.sql().len(), 1);
        assert_eq!(&format!("{}", q.sql()[0]), "[\"SELECT ?\",1]");

        let mut q =
            Query::new(&TEST_CONNECTION).push_sql(Value::Array(vec!["SELECT ?".into(), 1.into()]));
        assert_eq!(&q.create_path_with_query(), "/db/query");
        assert_eq!(q.sql().len(), 1);
        assert_eq!(&format!("{}", q.sql()[0]), "[\"SELECT ?\",1]");

        q = q
            .push_sql(Value::Array(vec!["SELECT ?".into(), 2.into()]))
            .push_sql(Value::String("SELECT 3".into()));
        assert_eq!(&q.create_path_with_query(), "/db/query");
        assert_eq!(q.sql().len(), 3);
        assert_eq!(&format!("{}", q.sql()[0]), "[\"SELECT ?\",1]");
        assert_eq!(&format!("{}", q.sql()[1]), "[\"SELECT ?\",2]");
        assert_eq!(&format!("{}", q.sql()[2]), "[\"SELECT 3\"]");

        let q = Query::new(&TEST_CONNECTION)
            .push_sql(vec![Value::String("SELECT ?".into()), 4.into()].into());
        assert_eq!(&q.create_path_with_query(), "/db/query");
        assert_eq!(q.sql().len(), 1);
        assert_eq!(&format!("{}", q.sql()[0]), "[\"SELECT ?\",4]");
    }

    #[test]
    fn queued_write_test() {
        let mut q = Query::new(&TEST_CONNECTION)
            .set_endpoint(Endpoint::Execute)
            .push_sql_str("CREATE TEMP TABLE test (val TEXT)")
            .push_sql_str("INSERT INTO temp.test (val) VALUES ('sample')")
            .set_queue();
        assert_eq!(q.sql().len(), 2);
        assert_eq!(&q.create_path_with_query(), "/db/execute?queue");

        q = q.set_wait();
        assert_eq!(&q.create_path_with_query(), "/db/execute?queue&wait");

        q = q.set_timeout(Duration::from_secs(3).into());
        assert_eq!(
            &q.create_path_with_query(),
            "/db/execute?queue&wait&timeout=3s"
        );
    }

    #[test]
    fn none_test() {
        let q = Query::new(&TEST_CONNECTION).set_none();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?level=none");
    }

    #[test]
    fn none_freshness_test() {
        let q = Query::new(&TEST_CONNECTION)
            .set_none()
            .set_freshness(Duration::from_secs(1));
        let path = q.create_path_with_query();
        assert_eq!(path, "/db/query?level=none&freshness=1s");
    }

    #[test]
    fn none_pretty_test() {
        let q = Query::new(&TEST_CONNECTION).set_none().set_pretty();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?pretty&level=none");

        let q = Query::new(&TEST_CONNECTION).set_pretty().set_none();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?pretty&level=none");
    }

    #[test]
    fn none_associative_freshness_pretty_timing_test() {
        let q = Query::new(&TEST_CONNECTION)
            .set_associative()
            .set_pretty()
            .set_none()
            .set_timing()
            .set_freshness(Duration::from_millis(5));
        let path = q.create_path_with_query();
        assert_eq!(
            &path,
            "/db/query?associative&pretty&timing&level=none&freshness=5ms"
        );
    }

    #[test]
    fn strong_test() {
        let q = Query::new(&TEST_CONNECTION).set_strong();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?level=strong");
    }

    #[test]
    fn strong_associative_test() {
        let q = Query::new(&TEST_CONNECTION).set_strong().set_associative();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?associative&level=strong");

        let q = Query::new(&TEST_CONNECTION).set_associative().set_strong();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?associative&level=strong");
    }

    #[test]
    fn redirect_test() {
        let q = Query::new(&TEST_CONNECTION).disable_redirect();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?redirect");
    }

    #[test]
    fn weak_test() {
        let q = Query::new(&TEST_CONNECTION).set_weak();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?level=weak");
    }

    #[test]
    fn weak_timing_test() {
        let q = Query::new(&TEST_CONNECTION).set_timing().set_weak();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?timing&level=weak");
    }

    #[test]
    fn weak_timing_transaction_test() {
        let q = Query::new(&TEST_CONNECTION)
            .set_timing()
            .set_weak()
            .push_sql_str("")
            .enable_transaction();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?timing&level=weak");

        let q = Query::new(&TEST_CONNECTION)
            .set_timing()
            .set_weak()
            .push_sql_str("")
            .set_endpoint(Endpoint::Execute)
            .enable_transaction();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/execute?timing&transaction&level=weak");
    }

    #[test]
    fn weak_associative_pretty_timing_test() {
        let q = Query::new(&TEST_CONNECTION)
            .set_associative()
            .set_pretty()
            .set_timing()
            .set_weak();
        let path = q.create_path_with_query();
        assert_eq!(&path, "/db/query?associative&pretty&timing&level=weak");
    }
}

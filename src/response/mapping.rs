//! [`Mapping`] `enum` for different JSON [`Response`](crate::Response)

pub use associative::Associative;
pub use empty::Empty;
pub use error::Error;
pub use execute::Execute;
pub use standard::Standard;
pub use timed::Timed;

mod associative;
mod empty;
mod error;
mod execute;
mod macros;
mod standard;
mod timed;

/// [`Mapping`] is used in [`response::Result`](super::Result) and
/// [`Query::results()`](super::query::Query::results())
///
/// # Examples
///
/// Match your [`response::Result`](super::Result) to a concrete [`Response`](super::Response)
///
/// ```
/// use rqlite_client::response::{self, Result};
/// use rqlite_client::{Mapping, Response};
///
/// // // for description of the data structure
/// // let response_result: Result =
/// //     Ok(Response::Query(response::query::Query {
/// //         results: vec![Mapping::Error(response::mapping::Error {
/// //             error: String::new(),
/// //         })],
/// //         sequence_number: None,
/// //         time: None,
/// //     }));
///
/// let response_result: Option<Result> = None;
///
/// if let Some(Ok(response_result)) = response_result {
///     if let Ok(query) = response::Query::try_from(response_result) {
///         for result in query.results() {
///             match result {
///                 Mapping::Error(err) => err.to_string(),
///                 _ => "no error".to_string(),
///             };
///         }
///     }
/// }
/// ```
///
/// Convert [`Result`] to an own data struct
///
/// ```no_run
/// use rqlite_client::response::{self, Query, Result};
/// use rqlite_client::response::mapping::Timed;
/// use rqlite_client::{Mapping, Response};
///
/// struct MyData {
///     timing: std::time::Duration,
/// }
///
/// impl TryFrom<&Mapping> for MyData {
///     type Error = rqlite_client::Error;
///
///     fn try_from(r: &Mapping) -> std::result::Result<Self, Self::Error> {
///         match r {
///             Mapping::Associative(r) => Ok(MyData {
///                 timing: r.duration().unwrap(),
///             }),
///             Mapping::Standard(r) => Ok(MyData {
///                 timing: r.duration().unwrap(),
///             }),
///             _ => Err(rqlite_client::Error::from("not expected MyData")),
///         }
///     }
/// }
///
/// let response_result: Option<Result> = None;
///
/// if let Some(Ok(response_result)) = response_result {
///     if let Ok(query) = response::Query::try_from(response_result) {
///         let my_data = query
///             .results()
///             .filter_map(|r| MyData::try_from(r).ok())
///             .collect::<Vec<MyData>>();
///     }
/// }
/// ```
///
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Mapping {
    /// See [`Associative`]
    Associative(Associative),
    /// See [`Error`]
    Error(Error),
    /// See [`Execute`]
    Execute(Execute),
    /// See [`Standard`]
    Standard(Standard),
    /// See [`Empty`]
    Empty(Empty),
}

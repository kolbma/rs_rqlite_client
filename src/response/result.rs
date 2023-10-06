//! Type-safe [`Result`] containers for [`ResponseResult`](super::ResponseResult)

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
mod standard;
mod timed;

/// [`Result`] is used in [`ResponseResult`](super::ResponseResult) and [`Response::results()`](super::Response::results())
///
/// # Examples
///
/// Match your [`ResponseResult`](super::ResponseResult) to a concrete [`Result`]
///
/// ```
/// // let response_result: rqlite_client::ResponseResult = Ok(rqlite_client::Response {
/// //    results: vec![rqlite_client::Result::Error(rqlite_client::result::Error {
/// //    error: String::new(),
/// //    })],
/// //    time: None,
/// // });
/// let response_result: Option<rqlite_client::ResponseResult> = None;
/// if let Some(Ok(response_result)) = response_result {
///     for result in response_result.results() {
///         match result {
///             rqlite_client::Result::Error(err) => err.to_string(),
///             _ => "no error".to_string(),
///         };
///     }
/// }
/// ```
///
/// Convert [`Result`] to an own data struct
///
/// ```no_run
/// use rqlite_client::{Result, result::Timed};
///
/// struct MyData {
///     timing: std::time::Duration,
/// }
///
/// impl TryFrom<&Result> for MyData {
///     type Error = rqlite_client::result::Error;
///
///     fn try_from(r: &Result) -> std::result::Result<Self, Self::Error> {
///         match r {
///             Result::Associative(r) => Ok(MyData {
///                 timing: r.duration().unwrap(),
///             }),
///             Result::Standard(r) => Ok(MyData {
///                 timing: r.duration().unwrap(),
///             }),
///             _ => Err(rqlite_client::result::Error::from("not expected MyData")),
///         }
///     }
/// }
///
/// let response_result: Option<rqlite_client::ResponseResult> = None;
/// if let Some(Ok(response_result)) = response_result {
///     let my_data = response_result
///         .results()
///         .filter_map(|r| MyData::try_from(r).ok())
///         .collect::<Vec<MyData>>();
/// }
/// ```
///
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum Result {
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

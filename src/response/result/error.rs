//! `error`

/// `Error` result
///
/// ```json
/// {
///     "results": [
///         {
///             "error": "near \"nonsense\": syntax error"
///         }
///     ],
///     "time": 2.478862
/// }
/// ```
///
/// See <https://rqlite.io/docs/api/api/#handling-errors>
///
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Error {
    /// error message
    pub error: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("result error: {}", self.error))
    }
}

impl From<&str> for Error {
    fn from(error: &str) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

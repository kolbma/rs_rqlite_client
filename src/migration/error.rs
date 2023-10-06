//! `error`

/// `MigrationError`
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Data malformat
    DataMalformat(String),
    /// Internal bug
    Internal(&'static str),
    /// No migration data available
    NoData,
    /// No request builder found
    NoRequestBuilder,
    /// Single `Query` failed
    QueryFail(String),
    /// Transaction for `Migration` failed
    TransactionFail(u16, String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DataMalformat(msg) => f.write_fmt(format_args!("data malformat: {msg}")),
            Error::Internal(msg) => f.write_fmt(format_args!("internal: {msg}")),
            Error::NoData => f.write_str("no migration data available"),
            Error::NoRequestBuilder => f.write_str("no request builder found"),
            Error::QueryFail(msg) => f.write_fmt(format_args!("query error: {msg}")),
            Error::TransactionFail(status, msg) => {
                f.write_fmt(format_args!("transaction status: {status}, message: {msg}"))
            }
        }
    }
}

impl TryFrom<crate::Error> for Error {
    type Error = Error;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::HttpError(status, msg) => Err(Error::TransactionFail(status, msg)),
            crate::Error::IoError(err) => Err(Error::DataMalformat(err.to_string())),
            #[cfg(feature = "migration")]
            crate::Error::MigrationError(err) => Err(err),
            crate::Error::ResultError(msg) => Err(Error::QueryFail(msg)),
            #[cfg(feature = "ureq")]
            crate::Error::UreqError(err, sql) => Err(Error::QueryFail(format!("{err} [{sql:?}]"))),
            #[cfg(feature = "url")]
            crate::Error::UrlParseError(_err) => Err(Error::Internal("url parse failed")),
        }
    }
}

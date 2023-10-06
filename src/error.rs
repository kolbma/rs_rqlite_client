//! All obtainable [`Error`] values

#[cfg(feature = "migration")]
use crate::migration::MigrationError;

/// All obtainable [`Error`] values
#[derive(Debug)]
pub enum Error {
    /// `HttpError` not status 2xx
    HttpError(u16, String),

    /// [`std::io::Error`]
    IoError(std::io::Error),

    /// `MigrationError` (required feature _migration_)
    #[cfg(feature = "migration")]
    MigrationError(MigrationError),

    /// `ResultError`
    ResultError(String),

    /// [`ureq::Error`] (required feature _ureq_)
    #[cfg(feature = "ureq")]
    UreqError(Box<ureq::Error>, Box<Option<crate::Value>>),

    /// [`url::ParseError`] (required feature _url_)
    #[cfg(feature = "url")]
    UrlParseError(url::ParseError),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HttpError(status, msg) => {
                f.write_fmt(format_args!("HTTP Status {status}: {msg}"))
            }

            Error::IoError(inner) => inner.fmt(f),

            #[cfg(feature = "migration")]
            Error::MigrationError(inner) => inner.fmt(f),

            Error::ResultError(msg) => f.write_str(msg),

            #[cfg(feature = "ureq")]
            Error::UreqError(inner, sql) => f.write_fmt(format_args!("{inner} [{sql:?}]")),

            #[cfg(feature = "url")]
            Error::UrlParseError(inner) => inner.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

#[cfg(feature = "migration")]
impl From<MigrationError> for Error {
    fn from(value: MigrationError) -> Self {
        Self::MigrationError(value)
    }
}

#[cfg(feature = "ureq")]
impl From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        Self::UreqError(Box::new(value), Box::new(None))
    }
}

#[cfg(feature = "url")]
impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Self::UrlParseError(value)
    }
}

//! Supported [`DataType`]s

/// Supported [`DataType`]s
///
/// See <https://github.com/rqlite/rqlite/blob/ea92d5d7bd8b5e730ba387bed300400470d23a75/db/db.go#L1372>
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    /// Blob Binary
    Blob,
    /// Boolean
    Boolean,
    /// Integer
    Integer,
    /// Numeric
    Numeric,
    /// Float, Double, Real
    Real,
    /// Text
    Text,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob => f.write_str("blob"),
            Self::Boolean => f.write_str("boolean"),
            Self::Integer => f.write_str("integer"),
            Self::Numeric => f.write_str("numeric"),
            Self::Real => f.write_str("real"),
            Self::Text => f.write_str("text"),
        }
    }
}

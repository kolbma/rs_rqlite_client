//! `empty`

use super::timed::Timed;

/// `Empty` result
///
/// ```json
/// {
///     "results": [
///         {
///             "time": 0.000277428
///         }
///     ],
///     "time": 0.01125789
/// }
/// ```
///
/// See <https://rqlite.io/docs/api/api/#unified-endpoint>
///
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Empty {
    /// optional timing info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

impl Timed for Empty {
    fn time(&self) -> Option<f64> {
        self.time
    }
}

//! `execute`

use std::collections::HashMap;

use crate::Value;

use super::timed::Timed;

/// `Execute` result
///
/// ```json
/// {
///     "results": [
///         {
///             "last_insert_id": 1,
///             "rows_affected": 1,
///             "time": 0.00886
///         }
///     ],
///     "time": 0.0152
/// }
/// ```
///
/// See <https://rqlite.io/docs/api/api/#writing-data> or <https://rqlite.io/docs/api/bulk-api/#updates>
///
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Execute {
    /// last inserted id
    pub last_insert_id: u64,
    /// optional rows of result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rows: Option<Vec<HashMap<String, Value>>>,
    /// number of affected rows
    pub rows_affected: usize,
    /// optional timing info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

impl Timed for Execute {
    fn time(&self) -> Option<f64> {
        self.time
    }
}

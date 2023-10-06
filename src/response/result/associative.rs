//! `associative`

use std::collections::HashMap;

use crate::{DataType, Value};

use super::timed::Timed;

/// `Associative` result
///
/// ```json
/// {
///     "results": [
///         {
///             "types": {"id": "integer", "age": "integer", "name": "text"},
///             "rows": [
///                 { "id": 1, "age": 20, "name": "fiona"},
///                 { "id": 2, "age": 25, "name": "declan"}
///             ],
///             "time": 0.000173061
///         }
///     ],
///     "time": 0.000185964
/// }
/// ```
///
/// See <https://rqlite.io/docs/api/api/#associative-response-form>
///
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Associative {
    /// rows in key:value hashmap
    pub rows: Vec<HashMap<String, Value>>,
    /// optional timing info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    /// description of column `DataType`
    pub types: HashMap<String, DataType>,
}

impl Timed for Associative {
    fn time(&self) -> Option<f64> {
        self.time
    }
}

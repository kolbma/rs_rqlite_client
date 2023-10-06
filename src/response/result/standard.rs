//! `standard`

use crate::{DataType, Value};

use super::timed::Timed;

/// `Standard` result
///
/// ```json
/// {
///     "results": [
///         {
///             "columns": [
///                 "id",
///                 "name",
///                 "age"
///             ],
///             "types": [
///                 "integer",
///                 "text",
///                 "integer"
///             ],
///             "values": [
///                 [
///                     1,
///                     "fiona",
///                     20
///                 ]
///             ],
///             "time": 0.0150043
///         }
///     ],
///     "time": 0.0220043
/// }
/// ```
///
/// See <https://rqlite.io/docs/api/api/#querying-data>
///
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Standard {
    /// query columns
    pub columns: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// time needed for query if requested
    pub time: Option<f64>,
    /// `DataType` of `columns`
    pub types: Vec<DataType>,
    /// data sets with values per `columns`
    pub values: Vec<Vec<Value>>,
}

impl Timed for Standard {
    fn time(&self) -> Option<f64> {
        self.time
    }
}

impl Standard {
    /// Create empty `Standard`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Append column
    #[must_use]
    pub fn push_column(mut self, col: &str) -> Self {
        self.columns.push(col.to_string());
        self
    }

    /// Append type
    #[must_use]
    pub fn push_type(mut self, t: DataType) -> Self {
        self.types.push(t);
        self
    }

    /// Append single value `v` in `values` vec at `value_index`
    #[must_use]
    pub fn push_value(mut self, v: Value, value_index: usize) -> Self {
        if let Some(values) = self.values.get_mut(value_index) {
            values.push(v);
        }
        self
    }

    /// Append vec `v` to `values`
    #[must_use]
    pub fn push_values(mut self, v: Vec<Value>) -> Self {
        self.values.push(v);
        self
    }
}

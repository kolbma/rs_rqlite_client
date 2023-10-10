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
    /// time needed for query if requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    /// `DataType` of `columns`
    pub types: Vec<DataType>,
    /// data sets with values per `columns`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<Vec<Value>>>,
}

impl Timed for Standard {
    fn time(&self) -> Option<f64> {
        self.time
    }
}

impl Standard {
    /// Get name of column at `index`
    #[must_use]
    pub fn column(&self, index: usize) -> Option<&String> {
        self.columns.get(index)
    }

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

    /// Append single value `v` in `values` vec at row `row_index`
    #[must_use]
    pub fn push_value(mut self, v: Value, row_index: usize) -> Self {
        let values = self.values.get_or_insert(Vec::new());
        if let Some(values) = values.get_mut(row_index) {
            values.push(v);
        }
        self
    }

    /// Append vec `v` to `values`
    #[must_use]
    pub fn push_values(mut self, v: Vec<Value>) -> Self {
        self.values.get_or_insert(Vec::new()).push(v);
        self
    }

    /// Get [`DataType`] for column `index`
    #[must_use]
    pub fn data_type(&self, index: usize) -> Option<DataType> {
        self.types.get(index).copied()
    }

    /// Get value of row `row_index` and column `column_index`
    #[must_use]
    pub fn value(&self, row_index: usize, column_index: usize) -> Option<&Value> {
        if let Some(values) = &self.values {
            if let Some(row) = values.get(row_index) {
                return row.get(column_index);
            }
        }
        None
    }

    /// Get values of row `row_index`
    #[must_use]
    pub fn values(&self, row_index: usize) -> Option<&Vec<Value>> {
        if let Some(values) = &self.values {
            return values.get(row_index);
        }
        None
    }
}

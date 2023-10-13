//! `associative`

use std::collections::HashMap;

use crate::{response::mapping, DataType, Value};

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

impl From<mapping::Standard> for Associative {
    fn from(standard: mapping::Standard) -> Self {
        let columns = standard.columns;
        let mut v = Vec::new();
        let mut hm_typ = HashMap::new();

        if let Some(rows) = standard.values {
            for row in rows {
                let mut hm = HashMap::new();

                for (idx, column) in columns.iter().enumerate() {
                    if let Some(value) = row.get(idx) {
                        let _ = hm.insert(column.to_string(), value.clone());
                    }
                }
                v.push(hm);
            }

            for (idx, column) in columns.iter().enumerate() {
                if let Some(t) = standard.types.get(idx) {
                    let _ = hm_typ.insert(column.to_string(), *t);
                }
            }
        }

        Associative {
            rows: v,
            time: standard.time,
            types: hm_typ,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{response::mapping, DataType};

    use super::Associative;

    #[test]
    fn standard_to_associativ_test() {
        let standard = mapping::Standard {
            columns: vec!["id".to_string(), "val".to_string()],
            time: None,
            types: vec![DataType::Text, DataType::Text],
            values: Some(vec![
                vec!["123456".into(), "value".into()],
                vec!["123456".into(), "value".into()],
                vec!["123456".into(), "value".into()],
            ]),
        };

        let associative = Associative::from(standard.clone());

        assert_eq!(associative.time, standard.time);
        assert_eq!(associative.rows.len(), standard.values.unwrap().len());
        let mut keys = associative.rows[0]
            .keys()
            .map(String::from)
            .collect::<Vec<String>>();
        keys.sort_unstable();
        let mut columns = standard.columns;
        columns.sort_unstable();
        assert_eq!(keys, columns);
    }
}

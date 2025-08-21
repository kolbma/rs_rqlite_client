//! Response for [`Query`] results

use std::time::Duration;

use super::mapping::Mapping;
use crate::Error;

/// Result type with [`response::query::Query`](crate::response::query::Query) or [`Error`]
pub type Result = std::result::Result<Query, Error>;

/// Response for [`Query`] results
///
/// See [`Mapping`] for available variants of `results` and
/// [examples](../mapping/enum.Mapping.html#examples) to handle responses.
///
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Query {
    /// query results
    results: Vec<Mapping>,
    /// `sequence_number` of queued writes
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence_number: Option<u64>,
    /// if requested timing information
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<f64>,
    #[serde(skip)]
    iter: Option<Vec<Mapping>>,
}

impl Query {
    /// Get the duration of the request if available
    #[must_use]
    pub fn duration(&self) -> Option<Duration> {
        self.time.map(Duration::from_secs_f64)
    }

    /// Iterator for available [`Result`]s
    pub fn iter(&self) -> std::slice::Iter<'_, Mapping> {
        self.results.iter()
    }

    /// Iterator for available [`Result`]s
    pub fn results(&self) -> std::slice::Iter<'_, Mapping> {
        self.results.iter()
    }

    /// For queued writes there will be a `sequence_number`
    #[must_use]
    #[inline]
    pub fn sequence_number(&self) -> Option<u64> {
        self.sequence_number
    }

    /// Get the duration of the request if available in `f64` seconds
    #[must_use]
    #[inline]
    pub fn time(&self) -> Option<f64> {
        self.time
    }
}

impl Iterator for Query {
    type Item = Mapping;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter.is_none() {
            self.iter = Some(self.results.clone());
        }
        let results = &mut self.results;
        if results.is_empty() {
            None
        } else {
            Some(results.remove(0))
        }
    }
}

impl<'a> IntoIterator for &'a Query {
    type IntoIter = std::slice::Iter<'a, Mapping>;
    type Item = &'a Mapping;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(feature = "ureq")]
impl TryFrom<ureq::Response> for Query {
    type Error = Error;

    fn try_from(response: ureq::Response) -> std::result::Result<Self, Self::Error> {
        let status = response.status();

        if !(200..300).contains(&status) {
            return Err(Error::HttpError(status, response.status_text().to_string()));
        }

        response.into_json::<Self>().map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, time::Duration};

    use super::Query;
    use crate::response::mapping::{Associative, Mapping, Standard};
    use crate::DataType;

    #[test]
    fn response_associative_json_test() {
        let r = Query {
            results: vec![Mapping::Associative(Associative {
                rows: Vec::new(),
                time: None,
                types: HashMap::new(),
            })],
            sequence_number: None,
            time: None,
            iter: None,
        };

        let res = serde_json::to_string_pretty(&r);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let json = res.unwrap();
        assert!(json.contains("\"results\": ["));
        assert_eq!(
            json,
            "{\n  \"results\": [\n    {\n      \"rows\": [],\n      \"types\": {}\n    }\n  ]\n}"
        );

        let res = serde_json::to_string(&r);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let json = res.unwrap();
        assert!(json.contains("\"results\":["));
        assert_eq!(json, "{\"results\":[{\"rows\":[],\"types\":{}}]}");
    }

    #[test]
    fn response_standard_json_test() {
        let r = Query {
            results: vec![Mapping::Standard(Standard {
                time: None,
                types: Vec::new(),
                columns: Vec::new(),
                values: None,
            })],
            sequence_number: None,
            time: None,
            iter: None,
        };

        let res = serde_json::to_string_pretty(&r);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let json = res.unwrap();
        assert!(json.contains("\"results\": ["));
        assert_eq!(json, "{\n  \"results\": [\n    {\n      \"columns\": [],\n      \"types\": []\n    }\n  ]\n}");

        let res = serde_json::to_string(&r);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let json = res.unwrap();
        assert!(json.contains("\"results\":["));
        assert_eq!(json, "{\"results\":[{\"columns\":[],\"types\":[]}]}");
    }

    #[test]
    fn response_standard_data_type_json_test() {
        let r = Query {
            results: vec![Mapping::Standard(Standard {
                time: None,
                types: vec![DataType::Integer, DataType::Text],
                columns: vec!["id".to_string(), "value".to_string()],
                values: Some(vec![vec![1.into(), "test".into()]]),
            })],
            sequence_number: None,
            time: None,
            iter: None,
        };

        let res = serde_json::to_string(&r);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let json = res.unwrap();
        assert!(json.contains("\"results\":["));
        assert_eq!(
            json,
            "{\"results\":[{\"columns\":[\"id\",\"value\"],\"types\":[\"integer\",\"text\"],\"values\":[[1,\"test\"]]}]}"
        );
    }

    #[test]
    fn deserialize_associative_test() {
        let json =
            "{\n  \"results\": [\n    {\n      \"rows\": [],\n      \"types\": {}\n    }\n  ]\n}";

        let res: Result<Query, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 1);
        match &r.results[0] {
            Mapping::Associative(_) => {}
            _ => unreachable!(),
        }

        // with numeric
        let json = r#"{"results":[{"types":{"account_id":"integer","accountname":"text","confirm_at":"numeric",
            "created_at":"numeric","email":"text","pwhash":"text","updated_at":"numeric"},
            "rows":[{"account_id":1,"accountname":"Max Mustermann","confirm_at":null,"created_at":1697087224,
            "email":"J7MtuhBnoVSzH2OWntrYyOtgVKhImly3ZKDjH7NFdjM=",
            "pwhash":"$argon2id$v=19$m=19456,t=2,p=1$3Mfjr2P+3SNYt93OutkSPA$O+V41bBVZdx8VN8mumVuHEADRIpP5bkBiKQxM5Eun4E",
            "updated_at":1697087224}]}]}"#;

        let res: Result<Query, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 1);
        match &r.results[0] {
            Mapping::Associative(_) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn deserialize_error_test() {
        let json = r#"{
                "results": [
                    {
                        "error": "near \"nonsense\": syntax error"
                    }
                ],
                "time": 2.478862
            }"#;

        let res: Result<Query, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 1);
        match &r.results[0] {
            Mapping::Error(err) => {
                assert!(!err.error.is_empty());
                assert_eq!(&err.error, "near \"nonsense\": syntax error");
                assert!(r.time.is_some());
                #[allow(clippy::float_cmp)]
                {
                    assert!(r.time().unwrap() == 2.478_862_f64);
                }
                assert_eq!(
                    r.duration().unwrap(),
                    Duration::from_secs_f64(2.478_862_f64)
                );
            }
            _ => unreachable!("{:#?}", r),
        }
    }

    #[test]
    fn deserialize_standard_test() {
        let json = "{\n  \"results\": [\n    {\n      \"columns\": [],\n      \"types\": [],\n      \"values\": []\n    }\n  ]\n}";

        let res: Result<Query, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 1);
        match &r.results[0] {
            Mapping::Standard(_) => {}
            _ => unreachable!(),
        }
    }

    #[test]
    fn deserialize_multi_test() {
        let json = r#"{
            "results": [
                {
                    "last_insert_id": 1,
                    "rows_affected": 1,
                    "time": 0.000074612,
                    "rows": null
                },
                {
                    "last_insert_id": 2,
                    "rows_affected": 1,
                    "time": 0.000044645,
                    "rows": null
                },
                {
                    "types": { "age": "integer", "id": "integer", "name": "text"},
                    "rows": [
                        {"age": 20, "id": 1, "name": "fiona"},
                        {"age": 30, "id": 2, "name": "declan"}
                    ],
                    "time": 0.000055248
                },
                {
                    "error": "no such table: bar"
                }
            ],
            "time": 0.010571084
        }"#;

        let res: Result<Query, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 4);
        let mut results = r.results();
        match results.next().unwrap() {
            Mapping::Execute(execute) => {
                assert_eq!(execute.last_insert_id, 1);
                assert!(execute.rows.is_none());
            }
            _ => unreachable!(),
        }
        match results.next().unwrap() {
            Mapping::Execute(execute) => {
                assert_eq!(execute.last_insert_id, 2);
                assert!(execute.rows.is_none());
            }
            _ => unreachable!(),
        }
        match results.next().unwrap() {
            Mapping::Associative(associative) => {
                assert_eq!(associative.types.get("age").unwrap(), &DataType::Integer);
                assert_eq!(associative.types.get("name").unwrap(), &DataType::Text);
                assert_eq!(associative.rows[0].get("age").unwrap(), 20);
                assert_eq!(associative.rows[1].get("name").unwrap(), "declan");
                #[allow(clippy::float_cmp)]
                {
                    assert_eq!(associative.time.unwrap(), 0.000_055_248);
                }
            }
            _ => unreachable!(),
        }
        match results.next().unwrap() {
            Mapping::Error(_) => {}
            _ => unreachable!(),
        }
    }
}

//! Database [`Response`] container for [`ResponseResult`]

use std::time::Duration;

use crate::Error;
pub(crate) use result::Result;

pub mod result;

/// Result type with [`Response`] or [`Error`]
#[allow(clippy::module_name_repetitions)]
pub type ResponseResult = std::result::Result<Response, Error>;

/// Database [`Response`] container with `results` of [`Query`](super::Query)
///
/// See [`Result`] for available variants of `results` and [examples](./result/enum.Result.html#examples) to handle responses.
///
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Response {
    /// query results
    results: Vec<Result>,
    /// `sequence_number` of queued writes
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence_number: Option<u64>,
    /// if requested timing information
    #[serde(skip_serializing_if = "Option::is_none")]
    time: Option<f64>,
}

impl Response {
    /// Get the duration of the request if available
    #[must_use]
    pub fn duration(&self) -> Option<Duration> {
        self.time.map(Duration::from_secs_f64)
    }

    /// Iterator for available [`Result`]s
    pub fn results(&self) -> std::slice::Iter<'_, Result> {
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

#[cfg(feature = "ureq")]
impl TryFrom<ureq::Response> for Response {
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

    use super::{
        result::{Associative, Result, Standard},
        Response,
    };
    use crate::DataType;

    #[test]
    fn response_associative_json_test() {
        let r = Response {
            results: vec![Result::Associative(Associative {
                rows: Vec::new(),
                time: None,
                types: HashMap::new(),
            })],
            sequence_number: None,
            time: None,
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
        let r = Response {
            results: vec![Result::Standard(Standard {
                time: None,
                types: Vec::new(),
                columns: Vec::new(),
                values: Vec::new(),
            })],
            sequence_number: None,
            time: None,
        };

        let res = serde_json::to_string_pretty(&r);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let json = res.unwrap();
        assert!(json.contains("\"results\": ["));
        assert_eq!(json, "{\n  \"results\": [\n    {\n      \"columns\": [],\n      \"types\": [],\n      \"values\": []\n    }\n  ]\n}");

        let res = serde_json::to_string(&r);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let json = res.unwrap();
        assert!(json.contains("\"results\":["));
        assert_eq!(
            json,
            "{\"results\":[{\"columns\":[],\"types\":[],\"values\":[]}]}"
        );
    }

    #[test]
    fn response_standard_data_type_json_test() {
        let r = Response {
            results: vec![Result::Standard(Standard {
                time: None,
                types: vec![DataType::Integer, DataType::Text],
                columns: vec!["id".to_string(), "value".to_string()],
                values: vec![vec![1.into(), "test".into()]],
            })],
            sequence_number: None,
            time: None,
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

        let res: std::result::Result<Response, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 1);
        match &r.results[0] {
            Result::Associative(_) => {}
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

        let res: std::result::Result<Response, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 1);
        match &r.results[0] {
            Result::Error(err) => {
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

        let res: std::result::Result<Response, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 1);
        match &r.results[0] {
            Result::Standard(_) => {}
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

        let res: std::result::Result<Response, serde_json::Error> = serde_json::from_str(json);

        assert!(res.is_ok(), "error: {}", res.err().unwrap());
        let r = res.unwrap();
        assert_eq!(r.results.len(), 4);
        let mut results = r.results();
        match results.next().unwrap() {
            Result::Execute(execute) => {
                assert_eq!(execute.last_insert_id, 1);
                assert!(execute.rows.is_none());
            }
            _ => unreachable!(),
        }
        match results.next().unwrap() {
            Result::Execute(_) => {}
            _ => unreachable!(),
        }
        match results.next().unwrap() {
            Result::Associative(associative) => {
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
            Result::Error(_) => {}
            _ => unreachable!(),
        }
    }
}

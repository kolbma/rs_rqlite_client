//! mapping macros

/// Generate implementations of `TryFrom<Mapping>` for a `structure`
///
/// # Examples
///
/// ```
/// use serde::Deserialize;
/// use rqlite_client::{response::mapping, map_deserialized, Mapping};
///
/// #[derive(Debug, serde::Deserialize)]
/// pub(crate) struct Test {}
///
/// map_deserialized!(pub(crate) Test => Tests);
///
/// let m = Mapping::Empty(mapping::Empty { time: None });
/// let _ = Tests::try_from(m);
/// ```
///
/// Another example with requirement for lifetime specifier:
///
/// ```
/// use serde::Deserialize;
/// use rqlite_client::{response::mapping, map_deserialized, Mapping};
///
/// #[derive(Debug, serde::Deserialize)]
/// pub(crate) struct Test<'a> {
///     s: &'a str,
/// }
///
/// map_deserialized!(pub(crate) Test, 'a => Tests);
///
/// let m = Mapping::Empty(mapping::Empty { time: None });
/// let _ = Tests::try_from(m);
/// ```
///
#[macro_export]
macro_rules! map_deserialized {
    ($v:vis $p:ident, $t:lifetime => $l:ident) => {
        #[doc = concat!("Container with _inner_ `", stringify!($p), "`")]
        #[doc = "\nImplements `Deref`"]
        #[derive(Debug)]
        $v struct $l<$t> {
            inner: Vec<$p<$t>>,
        }

        impl<$t> $l<$t> {
            $v fn new(v: Vec<$p<$t>>) -> Self {
                Self {
                    inner: v
                }
            }
        }

        impl<$t> std::ops::Deref for $l<$t> {
            type Target = Vec<$p<$t>>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }


        impl<$t> std::ops::DerefMut for $l<$t> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl<$t> TryFrom<$crate::response::mapping::Mapping> for $l<$t> {
            type Error = $crate::Error;

            fn try_from(mapping: $crate::response::mapping::Mapping) -> Result<Self, Self::Error> {
                match mapping {
                    $crate::response::mapping::Mapping::Associative(mut associative) => {
                        let mut v = Vec::new();

                        while let Some(row) = associative.rows.pop() {
                            v.insert(0, $p::deserialize(serde::de::value::MapDeserializer::new(row.into_iter()))
                                .map_err(|err| $crate::Error::ResultError(err.to_string()))?);
                        }

                        Ok($l::new(v))
                    }
                    $crate::response::mapping::Mapping::Error(err) => Err((err.error.as_str()).into()),
                    $crate::response::mapping::Mapping::Execute(_) => {
                        unimplemented!("execute mapping")
                    },
                    $crate::response::mapping::Mapping::Standard(standard) => {
                        let mut associative = $crate::response::mapping::Associative::from(standard);

                        let mut v = Vec::new();

                        while let Some(row) = associative.rows.pop() {
                            v.insert(0, $p::deserialize(serde::de::value::MapDeserializer::new(row.into_iter()))
                                .map_err(|err| $crate::Error::ResultError(err.to_string()))?);
                        }

                        Ok($l::new(v))
                    }
                    $crate::response::mapping::Mapping::Empty(_) => {
                        Err("empty result".into())
                    }
                }
            }
        }
    };
    ($v:vis $p:ident => $l:ident) => {
        #[doc = concat!("Container with _inner_ `", stringify!($p), "`")]
        #[doc = "\nImplements `Deref`"]
        #[derive(Debug)]
        $v struct $l {
            inner: Vec<$p>,
        }

        impl $l {
            $v fn new(v: Vec<$p>) -> Self {
                Self {
                    inner: v
                }
            }
        }

        impl std::ops::Deref for $l {
            type Target = Vec<$p>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl std::ops::DerefMut for $l {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl TryFrom<$crate::response::mapping::Mapping> for $l {
            type Error = $crate::Error;

            fn try_from(mapping: $crate::response::mapping::Mapping) -> Result<Self, Self::Error> {
                match mapping {
                    $crate::response::mapping::Mapping::Associative(mut associative) => {
                        let mut v = Vec::new();

                        while let Some(row) = associative.rows.pop() {
                            v.insert(0, $p::deserialize(serde::de::value::MapDeserializer::new(row.into_iter()))
                                .map_err(|err| $crate::Error::ResultError(err.to_string()))?);
                        }

                        Ok($l::new(v))
                    }
                    $crate::response::mapping::Mapping::Error(err) => Err((err.error.as_str()).into()),
                    $crate::response::mapping::Mapping::Execute(_) => {
                        unimplemented!("execute mapping")
                    },
                    $crate::response::mapping::Mapping::Standard(standard) => {
                        let mut associative = $crate::response::mapping::Associative::from(standard);

                        let mut v = Vec::new();

                        while let Some(row) = associative.rows.pop() {
                            v.insert(0, $p::deserialize(serde::de::value::MapDeserializer::new(row.into_iter()))
                                .map_err(|err| $crate::Error::ResultError(err.to_string()))?);
                        }

                        Ok($l::new(v))
                    }
                    $crate::response::mapping::Mapping::Empty(_) => {
                        Err("empty result".into())
                    }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{response::mapping, Error, Mapping};
    use crate::{DataType, Value};
    use serde::Deserialize;

    #[derive(Debug, serde::Deserialize)]
    #[allow(dead_code)]
    pub(crate) struct Test {
        id: String,
        val: String,
    }

    map_deserialized!(pub(crate) Test => Tests);

    #[test]
    fn map_deserialized_associative_test() {
        let m = Mapping::Associative(mapping::Associative {
            rows: Vec::new(),
            time: None,
            types: HashMap::new(),
        });
        let r = Tests::try_from(m);

        assert!(r.is_ok(), "{:?}", r.unwrap_err());
        let tests = r.unwrap();
        assert!(tests.is_empty());

        let m = Mapping::Associative(mapping::Associative {
            rows: vec![HashMap::from([
                ("id".into(), Value::String("1".into())),
                ("val".into(), Value::String("value".into())),
            ])],
            time: None,
            types: HashMap::from([
                ("id".into(), DataType::Text),
                ("val".into(), DataType::Text),
            ]),
        });
        let r = Tests::try_from(m);

        assert!(r.is_ok(), "{:?}", r.unwrap_err());
        let tests = r.unwrap();
        assert_eq!(tests.len(), 1);
        assert_eq!(
            &format!("{tests:?}"),
            "Tests { inner: [Test { id: \"1\", val: \"value\" }] }"
        );
    }

    #[test]
    fn map_deserialized_empty_test() {
        let m = Mapping::Empty(mapping::Empty { time: None });
        let r = Tests::try_from(m);

        assert!(r.is_err());
        let Error::ResultError(err) = r.unwrap_err() else {
            unreachable!()
        };
        assert_eq!(&err, "empty result");
    }

    #[test]
    fn map_deserialized_error_test() {
        let m = Mapping::Error(mapping::Error {
            error: "error".to_string(),
        });
        let r = Tests::try_from(m);

        assert!(r.is_err());
        let Error::ResultError(err) = r.unwrap_err() else {
            unreachable!()
        };
        assert_eq!(&err, "error");
    }

    #[test]
    #[should_panic]
    #[ignore]
    fn map_deserialized_execute_test() {
        let m = Mapping::Execute(mapping::Execute {
            last_insert_id: 0,
            rows: None,
            rows_affected: 0,
            time: None,
        });
        let r = Tests::try_from(m);

        assert!(r.is_err());
        let Error::ResultError(err) = r.unwrap_err() else {
            unreachable!()
        };
        assert_eq!(&err, "error");
    }

    #[test]
    fn map_deserialized_standard_test() {
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
        let m = Mapping::Standard(standard);
        let r = Tests::try_from(m);

        assert!(r.is_ok(), "{:?}", r.unwrap_err());
        let tests = r.unwrap();
        assert_eq!(tests.len(), 3);
        assert_eq!(
            &format!("{tests:?}"), 
            "Tests { inner: [Test { id: \"123456\", val: \"value\" }, Test { id: \"123456\", val: \"value\" }, Test { id: \"123456\", val: \"value\" }] }"
        );
    }
}

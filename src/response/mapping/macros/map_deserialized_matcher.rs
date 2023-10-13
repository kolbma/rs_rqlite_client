//! Matcher used in [`crate::map_deserialized!`]
#![doc(hidden)]

/// Matcher used in [`crate::map_deserialized!`]
#[macro_export]
#[doc(hidden)]
macro_rules! map_deserialized_matcher {
    ($p:ident, $l:ident, $m:expr) => {{
        match $m {
            $crate::response::mapping::Mapping::Associative(mut associative) => {
                let mut v = Vec::new();

                while let Some(row) = associative.rows.pop() {
                    v.insert(
                        0,
                        $p::deserialize(serde::de::value::MapDeserializer::new(row.into_iter()))
                            .map_err(|err| $crate::Error::ResultError(err.to_string()))?,
                    );
                }

                Ok($l::new(v))
            }
            $crate::response::mapping::Mapping::Error(err) => Err((err.error.as_str()).into()),
            $crate::response::mapping::Mapping::Execute(_) => {
                std::unimplemented!("execute mapping")
            }
            $crate::response::mapping::Mapping::Standard(standard) => {
                let mut associative = $crate::response::mapping::Associative::from(standard);

                let mut v = Vec::new();

                while let Some(row) = associative.rows.pop() {
                    v.insert(
                        0,
                        $p::deserialize(serde::de::value::MapDeserializer::new(row.into_iter()))
                            .map_err(|err| $crate::Error::ResultError(err.to_string()))?,
                    );
                }

                Ok($l::new(v))
            }
            $crate::response::mapping::Mapping::Empty(_) => Err("empty result".into()),
        }
    }};
}

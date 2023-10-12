#![warn(clippy::pedantic)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    // unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![forbid(unsafe_code)]
#![cfg(all(feature = "migration", feature = "ureq"))]

use std::collections::HashMap;
use std::path::Path;

use rqlite_client::{
    migration::{Migration, SchemaVersion},
    response::{self, mapping},
    DataType, Mapping, Value,
};

use test_rqlited::{lock, TEST_RQLITED_DB};

#[test]
fn query_associative_get_test() {
    lock!({
        TEST_RQLITED_DB.run_test(|c| {
            let path = Path::new("./tests/test_migrations");
            let m = Migration::from_path(path);
            let version = m.migrate_to(&c, Some(&SchemaVersion(7))).unwrap();

            assert!(version >= SchemaVersion(7));

            for _ in 0..50 {
                let r = c
                    .query()
                    .set_associative()
                    .set_sql_str("SELECT * FROM test_associative")
                    .request_run();

                assert!(r.is_ok(), "response error: {}", r.err().unwrap());

                let r = response::query::Query::from(r.unwrap());
                let result = r.results().next().unwrap();

                match result {
                    Mapping::Associative(associative) => {
                        assert_eq!(
                            associative,
                            &mapping::Associative {
                                rows: vec![HashMap::from([
                                    ("id".into(), Value::Number(1.into())),
                                    ("val".into(), "test_associative".into())
                                ])],
                                time: None,
                                types: HashMap::from([
                                    ("val".into(), DataType::Text),
                                    ("id".into(), DataType::Integer)
                                ])
                            }
                        );
                    }
                    _ => unreachable!(),
                }
            }
        });
    });
}

#[test]
fn query_associative_post_test() {
    lock!({
        TEST_RQLITED_DB.run_test(|c| {
            let path = Path::new("./tests/test_migrations");
            let m = Migration::from_path(path);
            let version = m.migrate_to(&c, Some(&SchemaVersion(7))).unwrap();

            assert!(version >= SchemaVersion(7));

            for _ in 0..50 {
                let r = c
                    .query()
                    .set_associative()
                    .push_sql_str("SELECT * FROM test_associative")
                    .request_run();

                assert!(r.is_ok(), "response error: {}", r.err().unwrap());

                let r = response::query::Query::from(r.unwrap());
                let result = r.results().next().unwrap();

                match result {
                    Mapping::Associative(associative) => {
                        assert_eq!(
                            associative,
                            &mapping::Associative {
                                rows: vec![HashMap::from([
                                    ("id".into(), Value::Number(1.into())),
                                    ("val".into(), "test_associative".into())
                                ])],
                                time: None,
                                types: HashMap::from([
                                    ("val".into(), DataType::Text),
                                    ("id".into(), DataType::Integer)
                                ])
                            }
                        );
                    }
                    _ => unreachable!(),
                }
            }
        });
    });
}

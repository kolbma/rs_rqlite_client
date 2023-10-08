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
#![cfg(feature = "ureq")]

use lazy_static::lazy_static;

use rqlite_client::{
    request_type::{Get, Post},
    response::mapping,
    Connection, Request, RequestBuilder, Response,
};

mod test_rqlited;

const TEST_CONNECTION_URL: &str = "http://localhost:4001/";

#[cfg(feature = "url")]
lazy_static! {
    static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL).unwrap();
}
#[cfg(not(feature = "url"))]
lazy_static! {
    static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL);
}

#[test]
fn create_table_test_test() {
    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let r = Request::<Post>::new().run(&TEST_CONNECTION.execute().push_sql_str(
            "CREATE TABLE test (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)",
        ));

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = r.unwrap();
        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = r
        else {
            unreachable!()
        };
        let result = r.results().next().unwrap();

        match result {
            mapping::Mapping::Error(err) => assert_eq!(
                err,
                &mapping::Error {
                    error: "table test already exists".to_string()
                }
            ),
            mapping::Mapping::Empty(result) => assert_eq!(result, &mapping::Empty::default()),
            mapping::Mapping::Execute(_) => {}
            _ => unreachable!(),
        }
    });
}

#[test]
fn create_table_error_test() {
    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let r = Request::<Get>::new().run(&TEST_CONNECTION.query().set_sql_str(
            "CREATE TABLE error (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)",
        ));

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = r.unwrap();
        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = r
        else {
            unreachable!()
        };
        let result = r.results().next().unwrap();

        assert_eq!(
            result,
            &mapping::Mapping::Error(mapping::Error {
                error: "attempt to change database via query operation".to_string()
            })
        );
    });
}

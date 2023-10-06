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
    result, Connection, Request, RequestBuilder,
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
        let result = r.results().next().unwrap();

        match result {
            result::Result::Error(err) => assert_eq!(
                err,
                &result::Error {
                    error: "table test already exists".to_string()
                }
            ),
            result::Result::Empty(result) => assert_eq!(result, &result::Empty::default()),
            result::Result::Execute(_) => {}
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
        let result = r.results().next().unwrap();

        assert_eq!(
            result,
            &result::Result::Error(result::Error {
                error: "attempt to change database via query operation".to_string()
            })
        );
    });
}

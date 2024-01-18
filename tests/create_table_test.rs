#![allow(unused_crate_dependencies)]
#![cfg(feature = "ureq")]

use rqlite_client::{
    request_type::{Get, Post},
    response::mapping,
    Request, RequestBuilder, Response,
};
use test_rqlited::TEST_RQLITED_DB;

#[test]
fn create_table_test_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let r = Request::<Post>::new().run(&c.execute().push_sql_str(
            "CREATE TABLE table_test (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)",
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
                    error: "table table_test already exists".to_string()
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
    TEST_RQLITED_DB.run_test(|c| {
        let r = Request::<Get>::new().run(&c.query().set_sql_str(
            "CREATE TABLE table_error (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)",
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

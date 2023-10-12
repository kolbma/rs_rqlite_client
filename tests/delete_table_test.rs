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

use rqlite_client::{request_type::Post, response::mapping, Request, RequestBuilder, Response};
use test_rqlited::TEST_RQLITED_DB;

#[test]
fn delete_table_test_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let r = Request::<Post>::new().run(&c.execute().push_sql_str(
            "CREATE TABLE IF NOT EXISTS delete_table_test (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)",
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
            mapping::Mapping::Empty(result) => assert_eq!(result, &mapping::Empty::default()),
            mapping::Mapping::Execute(result) => assert_eq!(result.rows_affected, 1),
            _ => unreachable!("{:#?}", r),
        }

        let r =
            Request::<Post>::new().run(&c.execute().push_sql_str("DROP TABLE delete_table_test"));

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
            mapping::Mapping::Empty(result) => assert_eq!(result, &mapping::Empty::default()),
            mapping::Mapping::Execute(result) => assert_eq!(result.rows_affected, 1),
            _ => unreachable!("{:#?}", r),
        }
    });
}

#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(feature = "ureq")]

use rqlite_client::{
    request_type::{Get, Post},
    response::Query,
    DataType, Mapping, Request, RequestBuilder,
};
use test_rqlited::TestRqlited;

const TEST_TABLE: &str = "query_without_values";

#[test]
fn query_standard_without_values_test() {
    TestRqlited::get_or_init().run_test(|c| {
        let r = Request::<Post>::new().run(&c.execute().push_sql_str(&format!(
            "CREATE TABLE IF NOT EXISTS {TEST_TABLE} (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)"
        )));

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());

        let r = Request::<Get>::new().run(&c.query().set_sql_str(&format!(
            "SELECT name FROM {TEST_TABLE}"
        )));

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());
        let r = r.unwrap();

        if let Some(Mapping::Standard(result)) = Query::from(r).results().next() {
            assert_eq!(result.columns[0], "name");
            assert_eq!(result.types[0], DataType::Text);
        } else {
            unreachable!()
        }
    });
}

#[test]
fn query_associative_without_values_test() {
    TestRqlited::get_or_init().run_test(|c| {
        let r = Request::<Post>::new().run(&c.execute().push_sql_str(&format!(
            "CREATE TABLE IF NOT EXISTS {TEST_TABLE} (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)"
        )));

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());

        let r = Request::<Post>::new().run(&c.query().push_sql_str(&format!(
            "SELECT name FROM {TEST_TABLE}"
        )).set_associative());

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());
        let r = r.unwrap();

        if let Some(Mapping::Associative(result)) = Query::from(r).results().next() {
            assert!(result.rows.is_empty());
            assert_eq!(result.types["name"], DataType::Text);
        } else {
            unreachable!()
        }
    });
}

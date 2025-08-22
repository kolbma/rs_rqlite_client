#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(feature = "ureq")]

use rqlite_client::{request_type::Post, response::Query, Connection, Request, RequestBuilder};
use test_rqlited::TestRqlited;

const TEST_TABLE: &str = "query_execute_raft_index";

#[test]
fn query_associative_test() {
    TestRqlited::get_or_init().run_test(|c: Connection| {
        let r = Request::<Post>::new().run(&c.execute().push_sql_str(&format!(
                "CREATE TABLE IF NOT EXISTS {TEST_TABLE} (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)"
            )));

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());

        let r = Request::<Post>::new().run(
            &c.execute()
                .push_sql_str(&format!(
                    "INSERT INTO {TEST_TABLE} (name, age) VALUES ('associative', 2)"
                ))
                .set_raft_index()
                .set_associative(),
        );

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());
        let q = Query::from(r.unwrap());
        assert!(q.raft_index().is_some());
    });
}

#[test]
fn query_non_execute_no_raft_index_test() {
    TestRqlited::get_or_init().run_test(|c: Connection| {
        let r = Request::<Post>::new().run(&c.execute().push_sql_str(&format!(
                "CREATE TABLE IF NOT EXISTS {TEST_TABLE} (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)"
            )));

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());

        let r = Request::<Post>::new().run(
            &c.query()
                .push_sql_str(&format!(
                    "SELECT (name, age) FROM {TEST_TABLE}"
                ))
                .set_raft_index(),
        );

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());
        let q = Query::from(r.unwrap());
        assert!(q.raft_index().is_none());
    });
}

#[test]
fn query_standard_test() {
    TestRqlited::get_or_init().run_test(|c: Connection| {
        let r = Request::<Post>::new().run(&c.execute().push_sql_str(&format!(
                "CREATE TABLE IF NOT EXISTS {TEST_TABLE} (id INTEGER NOT NULL PRIMARY KEY, name TEXT, age INTEGER)"
            )));

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());

        let r = Request::<Post>::new().run(
            &c.execute()
                .push_sql_str(&format!(
                    "INSERT INTO {TEST_TABLE} (name, age) VALUES ('standard', 1)"
                ))
                .set_raft_index(),
        );

        assert!(r.is_ok(), "response error: {:?}", r.err().unwrap());
        let q = Query::from(r.unwrap());
        assert!(q.raft_index().is_some());
    });
}

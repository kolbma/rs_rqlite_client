#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(feature = "ureq")]

use rqlite_client::{
    response::{self, mapping},
    Connection, DataType, Mapping,
};
use test_rqlited::TestRqlited;

#[test]
fn level_auto_test() {
    TestRqlited::get_or_init().run_test(|c: Connection| {
        let r = c.query().set_auto().set_sql_str("SELECT 1").request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = response::query::Query::from(r.unwrap());
        let result = r.results().next().unwrap();

        match result {
            Mapping::Standard(result) => {
                assert_eq!(
                    result,
                    &mapping::Standard {
                        columns: vec!["1".to_string()],
                        time: None,
                        types: vec![DataType::Integer],
                        values: Some(vec![vec![1.into()]])
                    }
                );
            }
            _ => unreachable!(),
        }
    });
}

#[test]
fn level_linearizable_test() {
    TestRqlited::get_or_init().run_test(|c: Connection| {
        let r = c
            .query()
            .set_linearizable()
            .set_sql_str("SELECT 1")
            .request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = response::query::Query::from(r.unwrap());
        let result = r.results().next().unwrap();

        match result {
            Mapping::Standard(result) => {
                assert_eq!(
                    result,
                    &mapping::Standard {
                        columns: vec!["1".to_string()],
                        time: None,
                        types: vec![DataType::Integer],
                        values: Some(vec![vec![1.into()]])
                    }
                );
            }
            _ => unreachable!(),
        }
    });
}

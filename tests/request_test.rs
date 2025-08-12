#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(feature = "ureq")]

use std::time::Duration;

use lazy_static::lazy_static;

use test_rqlited::{TEST_RQLITED_DB, TEST_RQLITED_DB_URL};

use rqlite_client::{
    request_type::{Get, Post},
    response::{
        self,
        mapping::{self, Mapping},
    },
    Connection, DataType, Request, RequestBuilder,
};

const TEST_PROXY_URL: &str = "http://proxy.example.com:12345";
const TEST_SOCKS_PROXY_URL: &str = "socks5://user:password@127.0.0.1:12345";

#[cfg(feature = "url")]
lazy_static! {
    static ref TEST_PROXY_CONNECTION: Connection = Connection::new(TEST_RQLITED_DB_URL)
        .unwrap()
        .set_proxy(TEST_PROXY_URL);
    static ref TEST_SOCKS_PROXY_CONNECTION: Connection = Connection::new(TEST_RQLITED_DB_URL)
        .unwrap()
        .set_proxy(TEST_SOCKS_PROXY_URL);
}
#[cfg(not(feature = "url"))]
lazy_static! {
    static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL);
    static ref TEST_PROXY_CONNECTION: Connection =
        Connection::new(TEST_CONNECTION_URL).set_proxy(TEST_PROXY_URL);
    static ref TEST_SOCKS_PROXY_CONNECTION: Connection =
        Connection::new(TEST_CONNECTION_URL).set_proxy(TEST_SOCKS_PROXY_URL);
}

#[test]
fn nolevel_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let r = Request::from(Get).run(&c.query().set_sql_str("SELECT 1"));

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
fn nolevel_request_run_test() {
    TEST_RQLITED_DB.run_test(|c: Connection| {
        let r = c.query().set_sql_str("SELECT 1").request_run();

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
fn proxy_test() {
    TEST_RQLITED_DB.run_test(|_c| {
        let r = Request::<Get>::from(&*TEST_PROXY_CONNECTION).run(
            &TEST_PROXY_CONNECTION
                .query()
                .set_timeout_request(Duration::from_millis(10))
                .set_sql_str("SELECT 1"),
        );

        assert!(r.is_err());
        let err_msg = r.unwrap_err().to_string();
        assert!(
            err_msg.contains("Dns Failed") || err_msg.contains("Connection Failed"),
            "{}",
            err_msg
        );
    });
}

#[test]
fn query_post_switch_test() {
    TEST_RQLITED_DB.run_test(|c: Connection| {
        let r = c
            .query()
            .set_sql_str_slice(&["SELECT COUNT(*) FROM test4zc99f where val = ?", "test"])
            .request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = response::query::Query::from(r.unwrap());
        let result = r.results().next().unwrap();

        match result {
            Mapping::Error(result) => {
                assert!(
                    result.error.contains("no such table: test4zc99f"),
                    "{}",
                    result.error
                );
            }
            _ => unreachable!(),
        }
    });
}

#[test]
fn request_timeout_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let r = Request::<Get>::new().run(
            &c.query()
                .set_timeout_request(Duration::from_nanos(10))
                .set_sql_str("SELECT 1"),
        );

        assert!(r.is_err());
        let err_msg = r.unwrap_err().to_string();
        assert!(
            err_msg.contains("Network Error") && err_msg.contains("timed out"),
            "{}",
            err_msg
        );
    });
}

#[test]
fn socks_proxy_test() {
    TEST_RQLITED_DB.run_test(|_c| {
        let r = Request::<Get>::from(&*TEST_SOCKS_PROXY_CONNECTION).run(
            &TEST_SOCKS_PROXY_CONNECTION
                .query()
                .set_timeout_request(Duration::from_millis(10))
                .set_sql_str("SELECT 1"),
        );

        assert!(r.is_err());
        let err_msg = r.unwrap_err().to_string();
        #[cfg(feature = "ureq_socks_proxy")]
        assert!(
            err_msg.contains("Dns Failed") || err_msg.contains("Connection Failed"),
            "{}",
            err_msg
        );
        #[cfg(not(feature = "ureq_socks_proxy"))]
        assert!(err_msg.contains("SOCKS feature disabled"), "{}", err_msg);
    });
}

#[test]
fn weak_multi_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let r = Request::<Post>::new().run(
            &c.query()
                .set_weak()
                .push_sql_str("SELECT 1")
                .push_sql_str("SELECT date()"),
        );

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = response::query::Query::from(r.unwrap());

        let mut results = r.results();
        let result = results.next().unwrap();

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

        let result = results.next().unwrap();

        match result {
            Mapping::Standard(result) => {
                assert_eq!(
                    result,
                    &mapping::Standard {
                        columns: vec!["date()".to_string()],
                        time: None,
                        types: vec![DataType::Text],
                        values: Some(vec![vec![time::OffsetDateTime::now_utc()
                            .format(
                                &time::format_description::parse("[year]-[month]-[day]").unwrap()
                            )
                            .unwrap()
                            .into()]])
                    }
                );
            }
            _ => unreachable!(),
        }
    });
}

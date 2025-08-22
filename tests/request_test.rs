#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(feature = "ureq")]

use std::{sync::OnceLock, time::Duration};

use test_rqlited::{TestRqlited, TEST_RQLITED_DB_URL};

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

static TEST_PROXY_CONNECTION: OnceLock<Connection> = OnceLock::new();
static TEST_SOCKS_PROXY_CONNECTION: OnceLock<Connection> = OnceLock::new();

fn get_test_proxy_connection() -> &'static Connection {
    TEST_PROXY_CONNECTION.get_or_init(|| {
        #[cfg(feature = "url")]
        let c = Connection::new(TEST_RQLITED_DB_URL)
            .unwrap()
            .set_proxy(TEST_PROXY_URL);

        #[cfg(not(feature = "url"))]
        let c = Connection::new(TEST_RQLITED_DB_URL).set_proxy(TEST_PROXY_URL);

        c
    })
}

fn get_test_socks_proxy_connection() -> &'static Connection {
    TEST_SOCKS_PROXY_CONNECTION.get_or_init(|| {
        #[cfg(feature = "url")]
        let c = Connection::new(TEST_RQLITED_DB_URL)
            .unwrap()
            .set_proxy(TEST_SOCKS_PROXY_URL);

        #[cfg(not(feature = "url"))]
        let c = Connection::new(TEST_RQLITED_DB_URL).set_proxy(TEST_SOCKS_PROXY_URL);

        c
    })
}

#[test]
fn nolevel_test() {
    TestRqlited::get_or_init().run_test(|c: Connection| {
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
    TestRqlited::get_or_init().run_test(|c: Connection| {
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
    TestRqlited::get_or_init().run_test(|_c| {
        let r = Request::<Get>::from(get_test_proxy_connection()).run(
            &get_test_proxy_connection()
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
    TestRqlited::get_or_init().run_test(|c: Connection| {
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
    TestRqlited::get_or_init().run_test(|c| {
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
    TestRqlited::get_or_init().run_test(|_c| {
        let r = Request::<Get>::from(get_test_socks_proxy_connection()).run(
            &get_test_socks_proxy_connection()
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
    TestRqlited::get_or_init().run_test(|c| {
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

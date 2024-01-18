#![allow(unused_crate_dependencies)]
#![cfg(feature = "ureq")]

use std::time::Duration;

use rqlite_client::{response::mapping::Mapping, DataType, Error, Response, Value};
use test_rqlited::TEST_RQLITED_DB;

#[test]
fn queue_write_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let mut q = c
            .execute_queue()
            .push_sql_str("DROP TABLE IF EXISTS temp.queue_write_test")
            .push_sql_str("CREATE TEMP TABLE IF NOT EXISTS queue_write_test (id INTEGER NOT NULL PRIMARY KEY, name TEXT)");

        for i in 0..100 {
            q = q.push_sql_str_slice(&[
                "INSERT INTO temp.queue_write_test (name) VALUES (?)",
                &i.to_string(),
            ]);
        }

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = r.unwrap();
        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = r
        else {
            unreachable!()
        };
        assert!(r.sequence_number().is_some(), "{r:?}");
        assert!(r.results().next().is_none());

        std::thread::sleep(Duration::from_millis(500));

        let r = c
            .query()
            .set_sql_str("SELECT COUNT(*) FROM temp.queue_write_test")
            .request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = r.unwrap();
        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = r
        else {
            unreachable!()
        };

        if let Some(Mapping::Standard(standard)) = r.results().next() {
            assert_eq!(standard.types[0], DataType::Integer);
            assert_eq!(
                standard.value(0, 0).and_then(Value::as_u64).unwrap(),
                100_u64,
                "{:?}",
                standard.values(0)
            );
        }
    });
}

#[test]
fn queue_write_wait_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let mut q = c
            .execute_queue()
            .set_wait()
            .set_timeout(Duration::from_millis(1000).into())
            .push_sql_str("DROP TABLE IF EXISTS temp.queue_write_wait_test")
            .push_sql_str("CREATE TEMP TABLE IF NOT EXISTS queue_write_wait_test (id INTEGER NOT NULL PRIMARY KEY, name TEXT)");

        for i in 0..100 {
            q = q.push_sql_str_slice(&[
                "INSERT INTO temp.queue_write_wait_test (name) VALUES (?)",
                &i.to_string(),
            ]);
        }

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = r.unwrap();
        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = r
        else {
            unreachable!()
        };
        assert!(r.sequence_number().is_some(), "{r:?}");
        assert!(r.results().next().is_none());

        let r = c
            .query()
            .set_sql_str("SELECT COUNT(*) FROM temp.queue_write_wait_test")
            .request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());

        let r = r.unwrap();
        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = r
        else {
            unreachable!()
        };

        if let Some(Mapping::Standard(standard)) = r.results().next() {
            assert_eq!(standard.types[0], DataType::Integer);
            assert_eq!(
                standard.value(0, 0).and_then(Value::as_u64).unwrap(),
                100_u64,
                "{:?}",
                standard.values(0)
            );
        }
    });
}

#[test]
fn queue_write_wait_timeout_test() {
    TEST_RQLITED_DB.run_test(|c| {
        for _ in 0..20 {
            let mut q = c
                .execute_queue()
                .set_wait()
                .set_timeout(Duration::from_millis(1).into())
                .push_sql_str("DROP TABLE IF EXISTS temp.queue_write_wait_test")
                .push_sql_str("CREATE TEMP TABLE IF NOT EXISTS queue_write_wait_test (id INTEGER NOT NULL PRIMARY KEY, name TEXT)");

            for i in 0..100 {
                q = q.push_sql_str_slice(&[
                    "INSERT INTO temp.queue_write_wait_test (name) VALUES (?)",
                    &i.to_string(),
                ]);
            }

            let r = q.request_run();

            if let Err(Error::HttpError(status, _)) = r {
                assert_eq!(status, 408);
            } else {
                unreachable!("{r:?}");
            }
        }
    });
}

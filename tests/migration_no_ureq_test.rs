#![allow(unused_crate_dependencies)]
#![cfg(feature = "migration")]
#![cfg(not(feature = "ureq"))]

use std::path::Path;

use lazy_static::lazy_static;

use rqlite_client::{
    migration::{Migration, MigrationError, SchemaVersion},
    state, Connection, Query, RequestBuilder,
};

struct NoUreqRequest;

impl<T> RequestBuilder<T> for NoUreqRequest
where
    T: state::State,
{
    fn run(&self, _query: &Query<T>) -> rqlite_client::response::Result {
        Err(MigrationError::QueryFail("query not implemented".to_string()).into())
    }
}

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
fn migration_test() {
    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path).set_request_builder(NoUreqRequest {});
        let result = m.migrate(&TEST_CONNECTION);

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }
    });
}

#[test]
fn migration_to_test() {
    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path).set_request_builder(NoUreqRequest {});
        let result = m.migrate_to(&TEST_CONNECTION, Some(&SchemaVersion(u64::MAX)));

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }

        let to_version = SchemaVersion(1_u64);

        let result = m.migrate_to(&TEST_CONNECTION, Some(&to_version));

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }
    });
}

#[test]
fn rollback_to_test() {
    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path).set_request_builder(NoUreqRequest {});

        let result = m.migrate(&TEST_CONNECTION);

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }

        let result = m.rollback_to(&TEST_CONNECTION, &SchemaVersion(u64::MAX));

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }

        let to_version = SchemaVersion(0_u64);

        let result = m.rollback_to(&TEST_CONNECTION, &to_version);

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }
    });
}

#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(all(feature = "migration", not(feature = "ureq")))]

use std::path::Path;

use rqlite_client::{
    migration::{Migration, MigrationError, SchemaVersion},
    state, Query, RequestBuilder,
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

#[test]
fn migration_test() {
    test_rqlited::TestRqlited::get_or_init().run_test(|connection| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path).set_request_builder(NoUreqRequest {});
        let result = m.migrate(&connection);

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }
    });
}

#[test]
fn migration_to_test() {
    test_rqlited::TestRqlited::get_or_init().run_test(|connection| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path).set_request_builder(NoUreqRequest {});
        let result = m.migrate_to(&connection, Some(&SchemaVersion(u64::MAX)));

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }

        let to_version = SchemaVersion(1_u64);

        let result = m.migrate_to(&connection, Some(&to_version));

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }
    });
}

#[test]
fn rollback_to_test() {
    test_rqlited::TestRqlited::get_or_init().run_test(|connection| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path).set_request_builder(NoUreqRequest {});

        let result = m.migrate(&connection);

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }

        let result = m.rollback_to(&connection, &SchemaVersion(u64::MAX));

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }

        let to_version = SchemaVersion(0_u64);

        let result = m.rollback_to(&connection, &to_version);

        if let Err(MigrationError::QueryFail(err)) = result {
            assert_eq!(&err, "query not implemented");
        } else {
            unreachable!();
        }
    });
}

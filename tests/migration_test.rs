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
#![cfg(all(feature = "ureq", feature = "migration"))]

use std::path::Path;

use rqlite_client::migration::{
    Downgrade, Migration, SchemaVersion, Upgrade, M, SCHEMA_VERSION_MAX,
};
use test_rqlited::{lock, TEST_RQLITED_DB};

#[test]
fn migration_test() {
    lock!({
        TEST_RQLITED_DB.run_test(|c| {
            let path = Path::new("./tests/test_migrations");
            let m = Migration::from_path(path);
            let version = m.migrate(&c).unwrap_or_else(|err| {
                unreachable!(
                    "SchemaVersion result for {}: {}",
                    path.canonicalize().unwrap().display(),
                    err
                )
            });
            assert_eq!(version, m.max());
        });
    });
}

#[test]
fn migration_to_test() {
    lock!({
        TEST_RQLITED_DB.run_test(|c| {
            let path = Path::new("./tests/test_migrations");
            let m = Migration::from_path(path);
            let err = m.migrate_to(&c, Some(&SCHEMA_VERSION_MAX));

            assert!(err.is_err());
            assert_eq!(
                err.err().unwrap().to_string(),
                format!("data malformat: no migration {SCHEMA_VERSION_MAX}")
            );

            // do not interfere with other tests
            let to_version = SchemaVersion(1_u64);

            let version = m.migrate_to(&c, Some(&to_version)).unwrap_or_else(|err| {
                unreachable!(
                    "SchemaVersion result for {}: {}",
                    path.canonicalize().unwrap().display(),
                    err
                )
            });

            assert!(version >= to_version);
        });
    });
}

#[test]
fn rollback_to_test() {
    lock!({
        TEST_RQLITED_DB.run_test(|c| {
            let path = Path::new("./tests/test_migrations");
            let m = Migration::from_path(path);

            let db_version = m.migrate(&c).unwrap_or_else(|err| {
                unreachable!(
                    "SchemaVersion result for {}: {}",
                    path.canonicalize().unwrap().display(),
                    err
                )
            });
            assert!(db_version > SchemaVersion(0));

            let err = m.rollback_to(&c, &SCHEMA_VERSION_MAX);

            assert!(err.is_err());
            assert_eq!(
                err.err().unwrap().to_string(),
                format!("data malformat: no rollback {SCHEMA_VERSION_MAX}")
            );

            // last 3 directories are for migration/rollback tests
            let to_version = m.max() - 3;

            let version = m.rollback_to(&c, &to_version).unwrap_or_else(|err| {
                unreachable!(
                    "SchemaVersion result for {}: {}",
                    path.canonicalize().unwrap().display(),
                    err
                )
            });

            assert_eq!(version, to_version);
        });
    });
}

#[test]
fn single_migration_test() {
    let m_sql = M(
        Upgrade::from(
            "CREATE TEMP TABLE IF NOT EXISTS single_migration (id INTEGER, migration TEXT)",
        ),
        Some(Downgrade::from("DROP TABLE temp.single_migration")),
    );

    let m = Migration::from(&m_sql);
    assert_eq!(m.max(), SchemaVersion(1));
    let m_pop_sql = m.pop().unwrap();
    assert_eq!(m_pop_sql, m_sql);
}

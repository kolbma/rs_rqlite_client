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

use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use rqlite_client::{
    migration::{Downgrade, Migration, SchemaVersion, Upgrade, M},
    Connection,
};

mod test_rqlited;

const TEST_CONNECTION_URL: &str = "http://localhost:4001/";

#[cfg(feature = "url")]
lazy_static! {
    static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL).unwrap();
}
#[cfg(not(feature = "url"))]
lazy_static! {
    static ref TEST_CONNECTION: Connection = Connection::new(TEST_CONNECTION_URL);
}

lazy_static! {
    static ref LOCK: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
}

#[test]
fn migration_test() {
    let lock = Arc::clone(&LOCK);
    let lock = lock.lock();
    assert!(lock.is_ok());

    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path);
        let version = m.migrate(&TEST_CONNECTION).unwrap_or_else(|err| {
            unreachable!(
                "SchemaVersion result for {}: {}",
                path.canonicalize().unwrap().display(),
                err
            )
        });
        assert_eq!(u64::from(&version), 3_u64);
    });
}

#[test]
fn migration_to_test() {
    let lock = Arc::clone(&LOCK);
    let lock = lock.lock();
    assert!(lock.is_ok());

    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path);
        let err = m.migrate_to(&TEST_CONNECTION, Some(&SchemaVersion(u64::MAX)));

        assert!(err.is_err());
        assert_eq!(
            err.err().unwrap().to_string(),
            format!("data malformat: no migration {}", u64::MAX)
        );

        let to_version = SchemaVersion(1_u64);

        let version = m
            .migrate_to(&TEST_CONNECTION, Some(&to_version))
            .unwrap_or_else(|err| {
                unreachable!(
                    "SchemaVersion result for {}: {}",
                    path.canonicalize().unwrap().display(),
                    err
                )
            });

        assert!(u64::from(&version) >= u64::from(&to_version));
    });
}

#[test]
fn rollback_to_test() {
    let lock = Arc::clone(&LOCK);
    let lock = lock.lock();
    assert!(lock.is_ok());

    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let path = Path::new("./tests/test_migrations");
        let m = Migration::from_path(path);

        let db_version = u64::from(&m.migrate(&TEST_CONNECTION).unwrap_or_else(|err| {
            unreachable!(
                "SchemaVersion result for {}: {}",
                path.canonicalize().unwrap().display(),
                err
            )
        }));
        assert!(db_version > 0);

        let err = m.rollback_to(&TEST_CONNECTION, &SchemaVersion(u64::MAX));

        assert!(err.is_err());
        assert_eq!(
            err.err().unwrap().to_string(),
            format!("data malformat: no rollback {}", u64::MAX)
        );

        let to_version = SchemaVersion(0_u64);

        let version = m
            .rollback_to(&TEST_CONNECTION, &to_version)
            .unwrap_or_else(|err| {
                unreachable!(
                    "SchemaVersion result for {}: {}",
                    path.canonicalize().unwrap().display(),
                    err
                )
            });

        assert_eq!(u64::from(&version), u64::from(&to_version));
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

    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let m = Migration::from(&m_sql);

        let _version = m
            .migrate(&TEST_CONNECTION)
            .unwrap_or_else(|err| unreachable!("migration failed: {:?}: {}", m_sql, err));
    });
}

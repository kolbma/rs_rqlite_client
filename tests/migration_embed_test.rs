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
#![cfg(all(feature = "ureq", feature = "migration_embed"))]

use lazy_static::lazy_static;

use rqlite_client::{embed_migrations, migration::Migration, Connection};

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

embed_migrations!(pub(crate) MigrationEmbed("tests/test_migrations"));

#[test]
fn migration_test() {
    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let x = MigrationEmbed::get("01_test_table_create/upgrade.sql");
        assert!(x.is_some());

        let m = Migration::from_embed::<MigrationEmbed>();
        let version = m
            .migrate(&TEST_CONNECTION)
            .unwrap_or_else(|err| unreachable!("err: {:?}", err));
        assert_eq!(u64::from(&version), 3_u64);
    });
}

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
#![cfg(all(feature = "migration_embed", feature = "ureq"))]

use rqlite_client::{embed_migrations, migration::Migration};
use test_rqlited::TEST_RQLITED_DB;

embed_migrations!(pub(crate) MigrationEmbed("tests/test_migrations"));

#[test]
fn migration_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let x = MigrationEmbed::get("04_test_embed_table_create/upgrade.sql");
        assert!(x.is_some());

        let m = Migration::from_embed::<MigrationEmbed>();
        let version = m
            .migrate(&c)
            .unwrap_or_else(|err| unreachable!("err: {:?}", err));
        assert_eq!(version, m.max());
    });
}

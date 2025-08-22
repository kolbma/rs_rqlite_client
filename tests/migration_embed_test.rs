#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(all(feature = "migration_embed", feature = "ureq"))]

use rqlite_client::{embed_migrations, migration::Migration};
use test_rqlited::TestRqlited;

embed_migrations!(pub(crate) MigrationEmbed("tests/test_migrations"));

#[test]
fn migration_test() {
    TestRqlited::get_or_init().run_test(|c| {
        let x = MigrationEmbed::get("04_test_embed_table_create/upgrade.sql");
        assert!(x.is_some());

        let m = Migration::from_embed::<MigrationEmbed>();
        let version = m
            .migrate(&c)
            .unwrap_or_else(|err| unreachable!("err: {:?}", err));
        assert_eq!(version, m.max());
    });
}

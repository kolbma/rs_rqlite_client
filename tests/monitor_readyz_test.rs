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
#![cfg(feature = "monitor")]
#![cfg(feature = "ureq")]

use lazy_static::lazy_static;

use rqlite_client::{monitor::response, Connection};

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

#[test]
fn monitor_readyz_test() {
    test_rqlited::TEST_RQLITED_DB.run_test(|| {
        let q = TEST_CONNECTION.monitor().readyz();

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let readyz = response::Readyz::try_from(r.unwrap()).unwrap();

        assert!(readyz.is_leader_ok);
        assert!(readyz.is_node_ok);
        assert!(readyz.is_store_ok);
    });
}
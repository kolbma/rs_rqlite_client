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
#![cfg(all(feature = "monitor", feature = "ureq"))]

use rqlite_client::monitor::response;
use test_rqlited::{TEST_RQLITED_DB, TEST_RQLITED_DB_URL};

#[test]
fn monitor_nodes_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let q = c.monitor().nodes();

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let nodes = response::Nodes::try_from(r.unwrap()).unwrap();
        let node = nodes.get("localhost:4002").unwrap();
        assert!(node.leader);
        assert!(node.reachable);
        assert_eq!(node.api_addr, TEST_RQLITED_DB_URL);
    });
}

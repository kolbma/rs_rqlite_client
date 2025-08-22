#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(all(feature = "monitor", feature = "ureq"))]

use rqlite_client::monitor::response;
use test_rqlited::{TestRqlited, TEST_RQLITED_DB_URL};

#[test]
fn monitor_nodes_v2_test() {
    TestRqlited::get_or_init().run_test(|c| {
        let q = c.monitor().nodes();

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let nodes = response::NodesV2::from(r.unwrap());
        let node = nodes.first().unwrap();
        assert!(node.leader);
        assert!(node.reachable);
        assert_eq!(node.api_addr, TEST_RQLITED_DB_URL);
        assert!(
            node.version.starts_with("v8."),
            "node.version = {}",
            node.version
        );
        assert!(node.voter);
    });
}

#[test]
fn monitor_nodes_v2_ver2_test() {
    TestRqlited::get_or_init().run_test(|c| {
        let q = c.monitor().nodes().enable_version2();

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let nodes = response::NodesV2::from(r.unwrap());
        let node = nodes.first().unwrap();
        assert!(node.leader);
        assert!(node.reachable);
        assert_eq!(node.api_addr, TEST_RQLITED_DB_URL);
        assert!(
            node.version.starts_with("v8."),
            "node.version = {}",
            node.version
        );
        assert!(node.voter);
    });
}

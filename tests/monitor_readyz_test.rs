#![allow(unused_crate_dependencies)]
#![cfg(all(feature = "monitor", feature = "ureq"))]

use rqlite_client::monitor::response;
use test_rqlited::TEST_RQLITED_DB;

#[test]
fn monitor_readyz_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let q = c.monitor().readyz();

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let readyz = response::Readyz::from(r.unwrap());

        assert!(readyz.is_leader_ok);
        assert!(readyz.is_node_ok);
        assert!(readyz.is_store_ok);
    });
}

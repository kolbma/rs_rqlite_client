#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(all(feature = "monitor", feature = "ureq"))]

use std::time::Duration;

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
        assert!(!readyz.is_sync_ok);
    });
}

#[test]
fn monitor_readyz_sync_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let q = c
            .monitor()
            .readyz()
            .enable_sync()
            .set_timeout(Duration::from_secs(1).into());

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let readyz = response::Readyz::from(r.unwrap());

        assert!(readyz.is_leader_ok);
        assert!(readyz.is_node_ok);
        assert!(readyz.is_store_ok);
        assert!(readyz.is_sync_ok);
    });
}

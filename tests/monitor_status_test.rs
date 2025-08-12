#![allow(missing_docs, unused_crate_dependencies)]
#![cfg(all(feature = "monitor", feature = "ureq"))]

use rqlite_client::monitor::response;
use test_rqlited::TEST_RQLITED_DB;

#[test]
fn monitor_status_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let q = c.monitor();

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let r = response::Status::from(r.unwrap());

        let v = r.0.as_object().unwrap();
        let cluster = v["cluster"].as_object().unwrap();

        assert_eq!(cluster["api_addr"], "localhost:4001");
    });
}

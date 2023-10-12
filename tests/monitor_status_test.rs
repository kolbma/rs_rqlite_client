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
use test_rqlited::TEST_RQLITED_DB;

#[test]
fn monitor_status_test() {
    TEST_RQLITED_DB.run_test(|c| {
        let q = c.monitor();

        let r = q.request_run();

        assert!(r.is_ok(), "response error: {}", r.err().unwrap());
        let r = response::Status::try_from(r.unwrap()).unwrap();

        let v = r.0.as_object().unwrap();
        let cluster = v["cluster"].as_object().unwrap();

        assert_eq!(cluster["api_addr"], "localhost:4001");
    });
}

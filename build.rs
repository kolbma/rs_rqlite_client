//! build.rs
//!

use std::env;
use std::fs;
use std::path::Path;
use std::time::SystemTime;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("buildtime.rs");
    fs::write(
        dest_path,
        format!(
            r#"pub(crate) const CRATE_BUILD_TIME: &str = "{}";"#,
            httpdate::fmt_http_date(SystemTime::now())
        ),
    )
    .unwrap();

    // println!("cargo:rerun-if-changed=build.rs");
}

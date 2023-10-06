//! `macro`

/// Embed a provided directory structure with file content compressed into crate
///
/// Might be used in combination with [`Migration`](super::super::Migration).
///
/// See [`rust_embed::RustEmbed`](https://docs.rs/rust-embed/latest/rust_embed/trait.RustEmbed.html) for further usage.
///
/// # Example
///
/// ```no_run
/// use rqlite_client::embed_migrations;
///
/// embed_migrations!(pub(crate) MyEmbeddedData("tests/test_migrations"));
///
/// let filename_iter = MyEmbeddedData::iter();
/// ```
///
#[macro_export]
macro_rules! embed_migrations {
    ( $v:vis $f:ident($p:expr) ) => {
        #[derive(::rust_embed::RustEmbed)]
        #[folder = $p]
        $v struct $f;
    };
}

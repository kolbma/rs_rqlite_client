//! `scheme`

/// Scheme of [`super::Connection`]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Scheme {
    #[default]
    Http,
    Https,
}

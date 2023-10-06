//! `varparam_macro`

/// Handle different convertable `Value`s in a slice
///
/// To protect against [SQL injection](https://owasp.org/www-community/attacks/SQL_Injection) use
/// parameterized statements.  
/// It is possible to use ordered question mark statements or named parameters.
///
/// See <https://rqlite.io/docs/api/api/#parameterized-statements> and <https://rqlite.io/docs/api/api/#named-parameters>.
///
/// # Example
///
/// ```no_run
/// use rqlite_client::varparam;
/// let stmt = varparam!["SELECT * FROM tbl WHERE col = ? and col2 = ?", 123, "dog"];
/// ```
///
#[macro_export]
macro_rules! varparam {
    ( $($v:expr),* ) => {
        [$(Into::<$crate::Value>::into($v)),*]
    };
}

//! `Downgrade` `Sql` statement

use super::Sql;

/// `Downgrade` `Sql` statement
///
/// # Usage
///
/// From `&str`
///
/// ```no_run
/// # use rqlite_client::migration::Downgrade;
/// let downgrade = Downgrade::from("DROP TABLE tbl");
/// ```
///
/// From file `Path`
///
/// ```no_run
/// # use rqlite_client::migration::Downgrade;
/// let downgrade = Downgrade::try_from(std::path::Path::new("./001_table_tbl/downgrade.sql"));
/// ```
///
/// Some _methods_ to work with the migration data
///
/// ```no_run
/// # use rqlite_client::migration::Downgrade;
/// # let downgrade = Downgrade::from("DROP TABLE tbl");
/// let sql: &str = downgrade.as_str();
/// let first_line: &str = downgrade.lines().next().expect("no line");
/// ```
///
pub type Downgrade<'a> = Sql<'a>;

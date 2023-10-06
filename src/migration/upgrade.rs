//! `Upgrade` `Sql` statement

use super::Sql;

/// `Upgrade` `Sql` statement
///
/// # Usage
///
/// From `&str`
///
/// ```no_run
/// # use rqlite_client::migration::Upgrade;
/// let upgrade = Upgrade::from("CREATE TABLE tbl");
/// ```
///
/// From file `Path`
///
/// ```no_run
/// # use rqlite_client::migration::Upgrade;
/// let upgrade = Upgrade::try_from(std::path::Path::new("./001_table_tbl/upgrade.sql"));
/// ```
///
/// Some _methods_ to work with the migration data
///
/// ```no_run
/// # use rqlite_client::migration::Upgrade;
/// # let upgrade = Upgrade::from("CREATE TABLE tbl");
/// let sql: &str = upgrade.as_str();
/// let first_line: &str = upgrade.lines().next().expect("no line");
/// ```
///
pub type Upgrade<'a> = Sql<'a>;

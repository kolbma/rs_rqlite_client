//! `sql`

use std::{
    path::Path,
    str::{FromStr, Lines},
};

use super::MigrationError;
use crate::Value;

/// `Sql`
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Sql<'a> {
    sql_str: Option<&'a str>,
    sql_string: Option<String>,
}

impl Sql<'_> {
    #[inline]
    pub fn as_str(&self) -> &'_ str {
        if let Some(sql) = &self.sql_string {
            sql
        } else {
            self.sql_str.unwrap()
        }
    }

    #[inline]
    pub fn lines(&self) -> Lines {
        self.as_str().lines()
    }

    fn parse_str(sql: &str) -> String {
        let mut filter_sql = String::new();

        for line in sql.lines() {
            let is_multi_line;

            let line = if let Some(line) = line.strip_suffix('\\') {
                is_multi_line = true;
                line.trim_start()
            } else {
                is_multi_line = false;
                line.trim()
            };

            if let Some(first_char) = &line.chars().next() {
                if !['#', ';', '/', '-'].contains(first_char) {
                    filter_sql.push_str(line);
                    if !is_multi_line {
                        filter_sql.push('\n');
                    }
                }
            }
        }

        filter_sql.trim_end().to_string()
    }
}

impl<'a> From<&'a str> for Sql<'a> {
    fn from(value: &'a str) -> Self {
        let sql_string = Self::parse_str(value);
        if value == sql_string {
            Self {
                sql_str: Some(value),
                sql_string: None,
            }
        } else {
            Self {
                sql_str: None,
                sql_string: Some(sql_string),
            }
        }
    }
}

impl<'a> From<&'a Sql<'a>> for &'a str {
    fn from(value: &'a Sql<'a>) -> &'a str {
        value.as_str()
    }
}

impl<'a> TryFrom<Sql<'a>> for Value {
    type Error = serde_json::Error;

    fn try_from(value: Sql<'a>) -> Result<Self, Self::Error> {
        Value::from_str(value.as_str())
    }
}

impl<'a> TryFrom<&Path> for Sql<'a> {
    type Error = MigrationError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let sql_string = std::fs::read_to_string(path).map_err(|_err| MigrationError::NoData)?;

        Ok(Self {
            sql_str: None,
            sql_string: Some(Self::parse_str(&sql_string)),
        })
    }
}

#[cfg(feature = "migration_embed")]
impl<'a> TryFrom<std::borrow::Cow<'a, [u8]>> for Sql<'a> {
    type Error = MigrationError;

    fn try_from(cow: std::borrow::Cow<'a, [u8]>) -> Result<Self, Self::Error> {
        let sql_string =
            String::from_utf8(cow.into_owned()).map_err(|_err| MigrationError::NoData)?;

        Ok(Self {
            sql_str: None,
            sql_string: Some(Self::parse_str(&sql_string)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Sql;

    #[test]
    fn from_str_test() {
        let sql = "   CREATE TABLE account(account_id INTEGER PRIMARY KEY, confirm_at NUMERIC DEFAULT NULL );   \n";
        let result = sql.trim();

        assert_eq!(Sql::from(sql).as_str(), result);
    }

    #[test]
    fn from_str_multiline_test() {
        let sql = "   CREATE TABLE account(account_id INTEGER PRIMARY KEY, confirm_at NUMERIC DEFAULT NULL );   \nCREATE TABLE IF NOT EXISTS account(account_id INTEGER PRIMARY KEY);\n\n";
        let result = "CREATE TABLE account(account_id INTEGER PRIMARY KEY, confirm_at NUMERIC DEFAULT NULL );\nCREATE TABLE IF NOT EXISTS account(account_id INTEGER PRIMARY KEY);";

        assert_eq!(Sql::from(sql).as_str(), result);
    }

    #[test]
    fn from_str_with_comments_test() {
        let sql = r"CREATE TABLE account(\
            account_id INTEGER PRIMARY KEY, \
            -- some comment email TEXT UNIQUE NOT NULL, \
            # more comment accountname TEXT NOT NULL, \
            ; another comment pwhash TEXT NOT NULL, \
            confirm_at NUMERIC DEFAULT NULL \
            /created_at NUMERIC NOT NULL DEFAULT (unixepoch()), \
            /updated_at NUMERIC NOT NULL DEFAULT (unixepoch())\
        );
        ";
        let result = "CREATE TABLE account(account_id INTEGER PRIMARY KEY, confirm_at NUMERIC DEFAULT NULL );";

        assert_eq!(Sql::from(sql).as_str(), result);
    }

    #[test]
    fn parse_str_test() {
        let sql = r"CREATE TABLE account(\
            account_id INTEGER PRIMARY KEY, \
            email TEXT UNIQUE NOT NULL, \
            accountname TEXT NOT NULL, \
            pwhash TEXT NOT NULL, \
            confirm_at NUMERIC DEFAULT NULL, \
            created_at NUMERIC NOT NULL DEFAULT (unixepoch()), \
            updated_at NUMERIC NOT NULL DEFAULT (unixepoch())\
        );
        ";
        let result = "CREATE TABLE account(account_id INTEGER PRIMARY KEY, email TEXT UNIQUE NOT NULL, accountname TEXT NOT NULL, pwhash TEXT NOT NULL, confirm_at NUMERIC DEFAULT NULL, created_at NUMERIC NOT NULL DEFAULT (unixepoch()), updated_at NUMERIC NOT NULL DEFAULT (unixepoch()));";

        assert_eq!(&Sql::parse_str(sql), result);
    }

    #[test]
    fn parse_str_with_comments_test() {
        let sql = r"CREATE TABLE account(\
            account_id INTEGER PRIMARY KEY, \
            -- some comment email TEXT UNIQUE NOT NULL, \
            # more comment accountname TEXT NOT NULL, \
            ; another comment pwhash TEXT NOT NULL, \
            confirm_at NUMERIC DEFAULT NULL \
            /created_at NUMERIC NOT NULL DEFAULT (unixepoch()), \
            /updated_at NUMERIC NOT NULL DEFAULT (unixepoch())\
        );
        ";
        let result = "CREATE TABLE account(account_id INTEGER PRIMARY KEY, confirm_at NUMERIC DEFAULT NULL );";

        assert_eq!(&Sql::parse_str(sql), result);
    }
}

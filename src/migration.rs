//! [`Migration`] and rollback of database definition with _SQL_

use std::path::{Path, PathBuf};

pub use downgrade::Downgrade;
#[allow(clippy::module_name_repetitions)]
pub use error::Error as MigrationError;
pub use schema_version::SchemaVersion;
pub(crate) use sql::Sql;
pub use upgrade::Upgrade;

use crate::{
    log, query::state, response::mapping::Mapping, tracing, Connection, Query, RequestBuilder,
    Response, Value,
};

mod downgrade;
mod embed;
mod error;
mod schema_version;
mod sql;
mod upgrade;

/// Single migration `M` with [`Upgrade`] and optional [`Downgrade`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M<'a>(pub Upgrade<'a>, pub Option<Downgrade<'a>>);

impl M<'_> {
    /// Use tuple struct as tuple
    #[must_use]
    #[inline]
    pub fn as_tuple(&'_ self) -> Mtuple<'_> {
        (&self.0, self.1.as_ref())
    }
}

pub(crate) type Mtuple<'a> = (&'a Upgrade<'a>, Option<&'a Downgrade<'a>>);

impl<'a> From<&'a M<'a>> for Mtuple<'a> {
    fn from(value: &'a M<'a>) -> Self {
        value.as_tuple()
    }
}

/// [`Migration`] and rollback of database definition with _SQL_
///
/// Requires feature `migration`.
///
/// Feature `migration_embed` provides macro [`embed_migrations!`](crate::embed_migrations!) to embed a directory
/// structure with SQL-files from filesystem into the application binary.
///
/// # Usage
///
/// Constructing `Migration` from code
///
/// ```no_run
/// use rqlite_client::migration::{Downgrade, Migration, Upgrade, M};
/// use rqlite_client::Connection;
///
/// #[cfg(feature = "ureq")]
/// {
///     let migration = Migration::default().push(M(
///         Upgrade::from("CREATE TEMP TABLE tmp"),
///         Some(Downgrade::from("DROP TABLE temp.tmp")),
///     ));
///
///     let con = Connection::new("http://localhost:4001");
///     #[cfg(feature = "url")]
///     let con = con.unwrap();
///
///     let version = migration.migrate(&con).expect("migration fail");
///     let version = migration
///         .rollback_to(&con, &(version - 1))
///         .expect("rollback fail");
/// }
/// ```
///
/// If you prefer to provide some SQL-files for _upgrade_ and _downgrade_ steps with your
/// application there is [`Migration::from_path()`].
///
/// And you can also include these files in your application with the combination of
/// [`embed_migrations!`](crate::embed_migrations!) and [`Migration::from_embed()`].
///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Migration<'a, T>
where
    T: RequestBuilder<state::NoLevelMulti>,
{
    migrations: Vec<M<'a>>,
    request_builder: Option<T>,
}

impl<'a, T> Migration<'a, T>
where
    T: RequestBuilder<state::NoLevelMulti>,
{
    /// Create `Migration` from [`rust_embed::RustEmbed`](https://docs.rs/rust-embed/latest/rust_embed/trait.RustEmbed.html)
    ///
    /// The `migrations` are __ordered__ like the sorted sub-directories in `path`.
    /// So one possibility is to start dir-names with fixed length numbers.
    ///
    /// In sub-directories you have to provide a file __upgrade.sql__ (case sensitive).
    /// And __optional__ a file __downgrade.sql__ (case sensitive).
    ///
    /// All files and directories need to have correct permissions to be readable
    /// during build of the crate.
    ///
    /// See [`embed`] module documentation how to embed the data.
    ///
    /// Requires feature `migration_embed`.
    ///
    #[cfg(feature = "migration_embed")]
    #[cfg(not(feature = "ureq"))]
    #[must_use]
    #[inline]
    pub fn from_embed<S>() -> Self
    where
        S: rust_embed::RustEmbed,
    {
        Self::new(embed::migrations::<S>())
    }

    /// Create `Migration` with `migrations` from __directory__ structure in `path`
    ///
    /// The `migrations` are __ordered__ like the sorted sub-directories in `path`.
    /// So one possibility is to start dir-names with fixed length numbers.
    ///
    /// In sub-directories you have to provide a file __upgrade.sql__ (case sensitive).
    /// And __optional__ a file __downgrade.sql__ (case sensitive).
    ///
    /// All files and directories need to have correct permissions or [`Migration#migrate`]
    /// will fail with [`MigrationError#NoData`].
    ///
    #[cfg(not(feature = "ureq"))]
    #[inline]
    pub fn from_path<'p, P>(path: P) -> Self
    where
        P: Into<&'p Path>,
    {
        Self::new(Self::migrations(path))
    }

    /// Create `Migration` with `Into<Vec<M>>`
    #[cfg(not(feature = "ureq"))]
    #[inline]
    pub fn new<S>(migrations: S) -> Self
    where
        S: Into<Vec<M<'a>>>,
    {
        Self {
            migrations: migrations.into(),
            request_builder: None,
        }
    }

    /// Migrate provided `Migration`
    ///
    /// # Return
    ///
    /// [`SchemaVersion`] after `Migration`
    ///
    /// # Errors
    ///
    /// [`MigrationError`] on failed `Migration`
    ///
    /// # Panics
    ///
    /// If there is no `RequestBuilder` provided with [`Migration::set_request_builder`](#method.set_request_builder)
    ///
    #[inline]
    pub fn migrate(&self, connection: &Connection) -> Result<SchemaVersion, MigrationError> {
        self.migrate_to(connection, None::<&SchemaVersion>)
    }

    /// Migrate provided `Migration` to provided [`SchemaVersion`]
    ///
    /// # Return
    ///
    /// [`SchemaVersion`] after `Migration`
    ///
    /// # Errors
    ///
    /// [`MigrationError`] on failed `Migration`
    ///
    /// # Panics
    ///
    /// If there is no `RequestBuilder` provided with [`Migration::set_request_builder`](#method.set_request_builder)
    ///
    // #[allow(clippy::too_many_lines)]
    pub fn migrate_to(
        &self,
        connection: &Connection,
        to_version: Option<&SchemaVersion>,
    ) -> Result<SchemaVersion, MigrationError> {
        if self.migrations.is_empty() {
            return Err(MigrationError::NoData);
        }
        if self.request_builder.is_none() {
            return Err(MigrationError::NoRequestBuilder);
        }

        let rb = self
            .request_builder
            .as_ref()
            .expect("no request_builder checked and found");

        let db_version = Self::pragma_user_version(connection, rb)?;

        // migrate from db_version onwards
        let mut query = connection.execute().enable_transaction();

        let mut version = SchemaVersion::default();

        for (upgrade, _) in self.migrations.iter().map(Mtuple::from) {
            if let Some(to_version) = to_version.filter(|v| &version > *v) {
                let _ = to_version;
                log::trace!("db_version: {db_version} - migrated to version {to_version}");
                tracing::trace!("db_version: {db_version} - migrated to version {to_version}");
                break;
            } else if db_version <= version {
                log::debug!("db_version: {db_version} migrating: {version}");
                tracing::debug!("db_version: {db_version} migrating: {version}");

                for line in upgrade.lines() {
                    let line = line.trim();
                    if let Some(first_char) = &line.chars().next() {
                        if !['#', ';', '/', '-'].contains(first_char) {
                            let v = Value::from(line);
                            query = query.push_sql(v);
                        }
                    }
                }
            } else {
                log::trace!("db_version: {db_version} - already migrated with version {version}");
                tracing::trace!(
                    "db_version: {db_version} - already migrated with version {version}"
                );
            }
            version += 1;
        }

        if let Some(to_version) = to_version {
            version = version.checked_sub(1).unwrap_or_default();
            if version != *to_version {
                return Err(MigrationError::DataMalformat(format!(
                    "no migration {to_version}"
                )));
            }
            if version < db_version {
                version = db_version;
            }
        }

        // at the end set new user_version
        Self::run_n_set_pragma_user_version(rb, query, version)?;

        log::info!("migrated to version {version}");
        tracing::info!("migrated to version {version}");

        Ok(version)
    }

    /// Removes the last migration and returns it, or None if there is no migration.
    #[must_use]
    #[inline]
    pub fn pop(mut self) -> Option<M<'a>> {
        self.migrations.pop()
    }

    /// Retrieve pragma `user_version` from DB
    fn pragma_user_version(
        connection: &Connection,
        rb: &T,
    ) -> Result<SchemaVersion, MigrationError> {
        let query = connection.query().push_sql_str("PRAGMA user_version");

        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = rb
            .run(&query)
            .map_err(|err| MigrationError::try_from(err).unwrap_err())?
        else {
            return Err(MigrationError::Internal("query_response required"));
        };

        let mut db_version = None;
        for (index, result) in r.results().enumerate() {
            match result {
                Mapping::Error(err) => {
                    return Err(MigrationError::QueryFail(format!(
                        "{} - {}",
                        err.error,
                        query.sql()[index]
                    )));
                }
                Mapping::Standard(standard) => {
                    db_version = standard
                        .value(0, 0)
                        .and_then(Value::as_u64)
                        .map(SchemaVersion);
                }
                _ => return Err(MigrationError::QueryFail("result not handled".to_string())),
            }
            if db_version.is_some() {
                break;
            }
        }

        db_version.ok_or(MigrationError::QueryFail("no schema version".to_string()))
    }

    /// Add single `migration` `M`
    #[must_use]
    #[inline]
    pub fn push<'m: 'a>(mut self, migration: M<'m>) -> Self {
        self.migrations.push(migration);
        self
    }

    /// Rollback provided `Migration` to provided [`SchemaVersion`]
    ///
    /// # Return
    ///
    /// [`SchemaVersion`] after `Migration` rollback
    ///
    /// # Errors
    ///
    /// [`MigrationError`] on failed `Migration` rollback
    ///
    /// # Panics
    ///
    /// If there is no `RequestBuilder` provided with [`Migration::set_request_builder`](#method.set_request_builder)
    ///
    pub fn rollback_to(
        &self,
        connection: &Connection,
        to_version: &SchemaVersion,
    ) -> Result<SchemaVersion, MigrationError> {
        if self.migrations.is_empty() {
            return Err(MigrationError::NoData);
        }
        if self.request_builder.is_none() {
            return Err(MigrationError::NoRequestBuilder);
        }

        let rb = self
            .request_builder
            .as_ref()
            .expect("request_builder checked and found");

        let db_version = Self::pragma_user_version(connection, rb)?;

        if *to_version >= db_version {
            return Err(MigrationError::DataMalformat(format!(
                "no rollback {to_version}"
            )));
        }

        // rollback from db_version backwards
        let mut query = connection.execute().enable_transaction();

        let mut version = db_version;

        for (_, downgrade) in self.migrations.iter().rev().map(Mtuple::from) {
            if let Some(downgrade) = downgrade {
                if version > *to_version {
                    version = version.checked_sub(1).unwrap_or_default();

                    log::debug!("db_version: {db_version} rollback: {version}");
                    tracing::debug!("db_version: {db_version} rollback: {version}");

                    for line in downgrade.lines() {
                        let line = line.trim();
                        if let Some(first_char) = &line.chars().next() {
                            if !['#', ';', '/', '-'].contains(first_char) {
                                let v = Value::from(line);
                                query = query.push_sql(v);
                            }
                        }
                    }
                }
            } else {
                version = version.checked_sub(1).unwrap_or_default();

                log::debug!("db_version: {db_version} rollback: {version}");
                tracing::debug!("db_version: {db_version} rollback: {version}");

                continue;
            }
        }

        // at the end set new user_version
        Self::run_n_set_pragma_user_version(rb, query, version)?;

        log::info!("rollback to version {version}");
        tracing::info!("rollback to version {version}");

        Ok(version)
    }

    /// Set pragma `user_version`
    fn run_n_set_pragma_user_version(
        rb: &T,
        query: Query<'_, state::NoLevelMulti>,
        version: SchemaVersion,
    ) -> Result<(), MigrationError> {
        // at the end set new user_version
        let query = query.push_sql_str(&format!("PRAGMA user_version={version}"));

        // irrefutable_let_patterns: with no monitor feature
        #[allow(irrefutable_let_patterns)]
        let Response::Query(r) = rb
            .run(&query)
            .map_err(|err| MigrationError::try_from(err).unwrap_err())?
        else {
            return Err(MigrationError::Internal("query response required"));
        };

        // check for error fields
        for (index, result) in r.results().enumerate() {
            if let Mapping::Error(err) = result {
                return Err(MigrationError::QueryFail(format!(
                    "{} - {}",
                    err.error,
                    query.sql()[index]
                )));
            }
        }

        Ok(())
    }

    /// Set `request_builder` to execute [`Migration`]
    #[cfg(not(feature = "ureq"))]
    #[must_use]
    #[inline]
    pub fn set_request_builder(mut self, builder: T) -> Self {
        self.request_builder = Some(builder);
        self
    }

    /// Shortens the migrations, keeping the first len elements and dropping the rest.
    ///
    /// If len is greater than the vector's current length, this has no effect.
    #[must_use]
    #[inline]
    pub fn truncate(mut self, len: usize) -> Self {
        self.migrations.truncate(len);
        self
    }

    fn migrations<'p, P>(path: P) -> Vec<M<'a>>
    where
        P: Into<&'p Path>,
    {
        let mut migrations = Vec::new();

        if let Ok(migration_files) = Self::migration_files(path) {
            for (upgrade_path, downgrade_path) in migration_files {
                if let Ok(upgrade_sql) = Sql::try_from(upgrade_path.as_path()) {
                    if let Some(downgrade_path) = downgrade_path {
                        migrations
                            .push(M(upgrade_sql, Sql::try_from(downgrade_path.as_path()).ok()));
                    } else {
                        migrations.push(M(upgrade_sql, None));
                    }
                }
            }
        }

        migrations
    }

    fn migration_files<'p, P>(path: P) -> Result<Vec<(PathBuf, Option<PathBuf>)>, MigrationError>
    where
        P: Into<&'p Path>,
    {
        let mut entries = std::fs::read_dir(path.into())
            .map_err(|_err| MigrationError::NoData)?
            .filter_map(|dir| {
                if let Ok(dir) = dir {
                    if dir.path().is_dir() {
                        let upgrade = dir.path().join("upgrade.sql");
                        if upgrade.is_file() {
                            let downgrade = dir.path().join("downgrade.sql");
                            if downgrade.is_file() {
                                return Some((upgrade, Some(downgrade)));
                            }
                            return Some((upgrade, None));
                        }
                    }
                }
                None
            })
            .collect::<Vec<(PathBuf, Option<PathBuf>)>>();

        entries.sort_unstable();

        Ok(entries)
    }
}

impl<'a, T> std::ops::Add for Migration<'a, T>
where
    T: RequestBuilder<state::NoLevelMulti> + Clone,
{
    type Output = Migration<'a, T>;

    fn add(self, rhs: Self) -> Self::Output {
        let mut m = self.clone(); // need a clone
        m.migrations.extend(rhs.migrations);
        m
    }
}

impl<T> std::ops::AddAssign for Migration<'_, T>
where
    T: RequestBuilder<state::NoLevelMulti>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.migrations.extend(rhs.migrations);
    }
}

#[cfg(feature = "ureq")]
impl<'a> Migration<'a, crate::Request<crate::request_type::Post>> {
    /// Create `Migration` from [`rust_embed::RustEmbed`](https://docs.rs/rust-embed/latest/rust_embed/trait.RustEmbed.html)
    ///
    /// The `migrations` are __ordered__ like the sorted sub-directories in `path`.
    /// So one possibility is to start dir-names with fixed length numbers.
    ///
    /// In sub-directories you have to provide a file __upgrade.sql__ (case sensitive).
    /// And __optional__ a file __downgrade.sql__ (case sensitive).
    ///
    /// All files and directories need to have correct permissions to be readable
    /// during build of the crate.
    ///
    /// See [`Migration`] documentation how to embed the data.
    ///
    /// Requires feature `migration_embed`.
    ///
    #[cfg(feature = "migration_embed")]
    #[must_use]
    #[inline]
    pub fn from_embed<S>() -> Self
    where
        S: rust_embed::RustEmbed,
    {
        Self::new(embed::migrations::<S>())
    }

    /// Create `Migration` with `migrations` from __directory__ structure in `path`
    ///
    /// The `migrations` are __ordered__ like the sorted sub-directories in `path`.
    /// So one possibility is to start dir-names with fixed length numbers.
    ///
    /// In sub-directories you have to provide a file __upgrade.sql__ (case sensitive).
    /// And __optional__ a file __downgrade.sql__ (case sensitive).
    ///
    /// All files and directories need to have correct permissions or [`Migration#migrate`]
    /// will fail with [`MigrationError#NoData`].
    ///
    #[inline]
    pub fn from_path<'p, P>(path: P) -> Self
    where
        P: Into<&'p Path>,
    {
        Self::new(Self::migrations(path))
    }

    /// Create `Migration` with `Into<Vec<M>>`
    #[inline]
    pub fn new<S>(migrations: S) -> Self
    where
        S: Into<Vec<M<'a>>>,
    {
        Self {
            migrations: migrations.into(),
            request_builder: Some(crate::Request::<crate::request_type::Post>::new()),
        }
    }
}

#[cfg(feature = "ureq")]
impl<'a> Default for Migration<'a, crate::Request<crate::request_type::Post>> {
    fn default() -> Self {
        Self {
            migrations: Vec::new(),
            request_builder: Some(crate::Request::<crate::request_type::Post>::new()),
        }
    }
}

/// Create [`Migration`] from single [`M`]
#[cfg(feature = "ureq")]
impl<'a> From<M<'a>> for Migration<'a, crate::Request<crate::request_type::Post>> {
    fn from(value: M<'a>) -> Self {
        Self::new(vec![value])
    }
}

/// Create [`Migration`] from single [`M`] reference
#[cfg(feature = "ureq")]
impl<'a> From<&'a M<'a>> for Migration<'a, crate::Request<crate::request_type::Post>> {
    fn from(value: &'a M<'a>) -> Self {
        Self::new(vec![value.clone()])
    }
}

#[cfg(not(feature = "ureq"))]
impl<'a, T> Default for Migration<'a, T>
where
    T: RequestBuilder<state::NoLevelMulti>,
{
    fn default() -> Self {
        Self {
            migrations: Vec::new(),
            request_builder: None,
        }
    }
}

/// Create [`Migration`] from single [`M`]
#[cfg(not(feature = "ureq"))]
impl<'a, T> From<M<'a>> for Migration<'a, T>
where
    T: RequestBuilder<state::NoLevelMulti>,
{
    fn from(value: M<'a>) -> Self {
        Self::new(vec![value])
    }
}

/// Create [`Migration`] from single [`M`] reference
#[cfg(not(feature = "ureq"))]
impl<'a, T> From<&'a M<'a>> for Migration<'a, T>
where
    T: RequestBuilder<state::NoLevelMulti>,
{
    fn from(value: &'a M<'a>) -> Self {
        Self::new(vec![value.clone()])
    }
}

/// Create `Migration` with `migrations` from __directory__ structure in `path`
///
/// The `migrations` are __ordered__ like the sorted sub-directories in `path`.
/// So one possibility is to start dir-names with fixed length numbers.
///
/// In sub-directories you have to provide a file __upgrade.sql__ (case sensitive).
/// And __optional__ a file __downgrade.sql__ (case sensitive).
///
/// All files and directories need to have correct permissions or [`Migration#migrate`]
/// will fail with [`MigrationError#NoData`].
///
#[cfg(feature = "ureq")]
impl<'p, P> From<P> for Migration<'_, crate::Request<crate::request_type::Post>>
where
    P: Into<&'p Path>,
{
    fn from(path: P) -> Self {
        Self::new(Self::migrations(path))
    }
}

/// Create `Migration` with `migrations` from __directory__ structure in `path`
///
/// The `migrations` are __ordered__ like the sorted sub-directories in `path`.
/// So one possibility is to start dir-names with fixed length numbers.
///
/// In sub-directories you have to provide a file __upgrade.sql__ (case sensitive).
/// And __optional__ a file __downgrade.sql__ (case sensitive).
///
/// All files and directories need to have correct permissions or [`Migration#migrate`]
/// will fail with [`MigrationError#NoData`].
///
#[cfg(not(feature = "ureq"))]
impl<'p, P, T> From<P> for Migration<'_, T>
where
    P: Into<&'p Path>,
    T: RequestBuilder<state::NoLevelMulti>,
{
    fn from(path: P) -> Self {
        Self::new(Self::migrations(path))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{migration::Migration, response, state, Error, Query, RequestBuilder};

    struct ImplRequestTest {}

    impl<T> RequestBuilder<T> for ImplRequestTest
    where
        T: state::State,
    {
        fn run(&self, query: &Query<T>) -> response::Result {
            Err(Error::ResultError(format!(
                "ImplRequestTest is dummy impl: {query}"
            )))
        }
    }

    #[test]
    fn sort_migrations_test() {
        for _ in 0..50 {
            let v =
                Migration::<ImplRequestTest>::migration_files(Path::new("tests/test_migrations"))
                    .unwrap();

            assert!(v[0]
                .0
                .to_string_lossy()
                .starts_with("tests/test_migrations/01_test_table_create/"));
            assert!(v[2]
                .0
                .to_string_lossy()
                .starts_with("tests/test_migrations/03_system_table_create/"));
        }
    }
}

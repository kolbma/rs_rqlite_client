//! `include`
#![cfg(feature = "migration_embed")]

use rust_embed::RustEmbed;

use super::Sql;

mod macros;

pub(crate) fn migrations<'a, T>() -> Vec<super::M<'a>>
where
    T: RustEmbed,
{
    let mut migrations = Vec::new();

    for filename in T::iter().filter_map(|f| {
        if f.ends_with("/upgrade.sql") {
            Some(f)
        } else {
            None
        }
    }) {
        // upgrade
        let mut m = read_data::<T>(&filename, None);

        let downgrade = filename.split_at(filename.len() - 11).0.to_string() + "downgrade.sql";
        // downgrade
        m = read_data::<T>(&downgrade, m);

        if let Some(m) = m {
            migrations.push(m);
        }
    }

    migrations
}

fn read_data<T>(filename: &str, mut m: Option<super::M<'static>>) -> Option<super::M<'static>>
where
    T: RustEmbed,
{
    if let Some(file) = T::get(filename) {
        let m = if let Some(mut m) = m.take() {
            // downgrade
            m.1 = Sql::try_from(file.data).ok();
            m
        } else if let Ok(upgrade) = Sql::try_from(file.data) {
            // upgrade
            super::M(upgrade, None)
        } else {
            return None;
        };

        return Some(m);
    }

    m
}

//! Limiting read staleness with [`Freshness`]

use std::time::Duration;

use super::duration_string::DurationString;

/// You can tell the receiving node not to return results staler than a certain duration, however.  
/// If a read request sets the query parameter freshness, the node serving the read will check that less time has
/// passed since it was last in contact with the Leader, than that specified via freshness.  
/// If more time has passed the node will return an error.  
/// This approach can be useful if you want to maximize successful query operations, but are willing to tolerate
/// occassional, short-lived networking issues between nodes.
///
/// If you decide to deploy read-only nodes however, none combined with freshness can be a particularly effective
/// at adding read scalability to your system. You can use lots of read-only nodes, yet be sure that a given node
/// serving a request has not fallen too far behind the Leader (or even become disconnected from the cluster).
///
/// See <https://rqlite.io/docs/api/read-consistency/#limiting-read-staleness>
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Freshness(Duration);

impl DurationString for Freshness {
    fn duration(&self) -> &Duration {
        &self.0
    }
}

impl From<Duration> for Freshness {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

impl Default for Freshness {
    fn default() -> Self {
        FRESHNESS_DEFAULT
    }
}

impl std::fmt::Display for Freshness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&DurationString::to_string(self))
    }
}

const FRESHNESS_DEFAULT: Freshness = Freshness(Duration::from_secs(1));

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Freshness;

    #[test]
    fn display_test() {
        assert_eq!(
            &Freshness::from(Duration::from_millis(1_001)).to_string(),
            "1s1ms"
        );
    }
}

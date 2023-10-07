//! [`Timeout`] parameter for [`Query`](super::Query)

use std::time::Duration;

use super::duration_string::DurationString;

/// [`Timeout`] parameter for [`Query`](super::Query)
///
/// __Request Forwarding Timeouts__
/// [<rqlite.io/docs>](https://rqlite.io/docs/api/api/#request-forwarding-timeouts)
///
/// If a Follower forwards a request to a Leader, by default the Leader must respond within 30 seconds.
/// You can control this timeout by setting the `timeout` parameter.
///
/// __Waiting for a queue to flush__
/// [<rqlite.io/docs>](https://rqlite.io/docs/api/queued-writes/#waiting-for-a-queue-to-flush)
///
/// During queued writes `timeout` controls the max. work time for the queue, before the write attempt fails.
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Timeout(Duration);

impl DurationString for Timeout {
    fn duration(&self) -> &Duration {
        &self.0
    }
}

impl From<Duration> for Timeout {
    fn from(value: Duration) -> Self {
        Self(value)
    }
}

/// [`Timeout`] default is 30 seconds
///
/// See <https://rqlite.io/docs/api/queued-writes/#waiting-for-a-queue-to-flush>
/// and <https://rqlite.io/docs/api/api/#request-forwarding-timeouts>
///
impl Default for Timeout {
    /// Get `Timeout` set to 30 seconds
    fn default() -> Self {
        TIMEOUT_DEFAULT
    }
}

impl std::fmt::Display for Timeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&DurationString::to_string(self))
    }
}

const TIMEOUT_DEFAULT: Timeout = Timeout(Duration::from_secs(30));

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::Timeout;

    #[test]
    fn display_test() {
        assert_eq!(&Timeout::default().to_string(), "30s");
        assert_eq!(
            &Timeout::from(Duration::from_millis(1_001)).to_string(),
            "1s1ms"
        );
    }
}

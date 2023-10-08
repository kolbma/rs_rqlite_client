//! Trait implementation for `time` value as `u64` and `Duration`

/// Trait implementation for `time` value as `u64` and `Duration`
pub trait Timed {
    /// Get optional `time` of response timing
    #[must_use]
    fn time(&self) -> Option<f64>;

    /// Get optional `time` as `Duration`
    #[must_use]
    #[inline]
    fn duration(&self) -> Option<std::time::Duration> {
        self.time().map(std::time::Duration::from_secs_f64)
    }
}

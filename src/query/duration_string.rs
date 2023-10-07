//! Formatable `Duration` as `String`

use std::time::Duration;

/// Trait [`DurationString`] providing [`DurationString::to_string()`]
pub trait DurationString {
    fn duration(&self) -> &Duration;

    /// Formats `DurationString` to something like `1h0m30s` or `1m9s` or `100ms` or `1us734ns`
    fn to_string(&self) -> String {
        let value = self.duration();
        let secs = value.as_secs();
        let hours = secs / 3600;
        let minutes = (secs - hours * 3600) / 60;
        let seconds = secs - hours * 3600 - minutes * 60;
        let millis = value.subsec_millis();
        let micros = value.subsec_micros() - millis * 1_000;
        let nanos = value.subsec_nanos() - micros * 1_000 - millis * 1_000_000;

        let data = [
            hours,
            minutes,
            seconds,
            u64::from(millis),
            u64::from(micros),
            u64::from(nanos),
        ];

        let mut v = String::new();
        let mut backlog = String::new();

        for (i, f) in DURATION_FORMATS.iter().enumerate() {
            if data[i] > 0 {
                if !backlog.is_empty() {
                    v.push_str(&backlog);
                    backlog.clear();
                }
                // padding with 0 for minutes and seconds
                // if !v.is_empty() && (1..=2).contains(&i) {
                //     v.push_str(&format!("{:02}{f}", data[i]));
                // } else {
                v.push_str(&data[i].to_string());
                v.push_str(f);
                // }
            } else if !v.is_empty() {
                // padding with 0 for minutes and seconds
                // if (1..=2).contains(&i) {
                //     backlog.push_str(&format!("{:02}{f}", data[i]));
                // } else {
                backlog.push_str(&data[i].to_string());
                backlog.push_str(f);
                // }
            }
        }

        v
    }
}

const DURATION_FORMATS: &[&str] = &["h", "m", "s", "ms", "us", "ns"];

impl From<&dyn DurationString> for Duration {
    fn from(value: &dyn DurationString) -> Self {
        *value.duration()
    }
}

impl From<Box<dyn DurationString>> for Duration {
    fn from(value: Box<dyn DurationString>) -> Self {
        *value.duration()
    }
}

impl<'a> From<&'a dyn DurationString> for &'a Duration {
    fn from(value: &'a dyn DurationString) -> Self {
        value.duration()
    }
}

impl std::fmt::Display for Box<dyn DurationString> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&(*self).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DurationStringImpl {
        duration: Duration,
    }

    impl DurationString for DurationStringImpl {
        fn duration(&self) -> &Duration {
            &self.duration
        }
    }

    #[test]
    fn duration_string_impl_test() {
        let mut d = DurationStringImpl {
            duration: Duration::from_secs(3600),
        };
        assert_eq!(&d.to_string(), "1h");

        d = DurationStringImpl {
            duration: Duration::from_secs(1),
        };
        assert_eq!(&d.to_string(), "1s");

        d = DurationStringImpl {
            duration: Duration::from_secs(61),
        };
        assert_eq!(&d.to_string(), "1m1s");

        d = DurationStringImpl {
            duration: Duration::from_secs(3601),
        };
        assert_eq!(&d.to_string(), "1h0m1s");

        d = DurationStringImpl {
            duration: Duration::from_nanos(1),
        };
        assert_eq!(&d.to_string(), "1ns");

        d = DurationStringImpl {
            duration: Duration::from_nanos(1001),
        };
        assert_eq!(&d.to_string(), "1us1ns");

        d = DurationStringImpl {
            duration: Duration::from_nanos(999_001),
        };
        assert_eq!(&d.to_string(), "999us1ns");

        d = DurationStringImpl {
            duration: Duration::from_nanos(1_001_001),
        };
        assert_eq!(&d.to_string(), "1ms1us1ns");

        d = DurationStringImpl {
            duration: Duration::from_micros(1_001_001),
        };
        assert_eq!(&d.to_string(), "1s1ms1us");

        d = DurationStringImpl {
            duration: Duration::from_millis(1_001_001),
        };
        assert_eq!(&d.to_string(), "16m41s1ms");
    }
}

//! This crate provides a convenient way to create `chrono::Duration` from numbers.
//!
//! Example:
//!
//! ```rust
//! use num_chrono_duration::NumChronoDuration;
//!
//! let today = chrono::Utc::today();
//! assert_eq!(today + 1.hours(), today + chrono::Duration::hours(1));
//! assert_eq!(today + 1.days(), today + chrono::Duration::days(1));
//! assert_eq!(today - 1.weeks(), today - chrono::Duration::weeks(1));
//! ```

use chrono::Duration;

/// Creates a new Duration from the specified number.
pub trait NumChronoDuration: num_traits::PrimInt {
    /// Creates a new Duration from the specified number of nanoseconds.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.nanoseconds(), Duration::nanoseconds(1));
    /// ```
    fn nanoseconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of microseconds.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.microseconds(), Duration::microseconds(1));
    /// ```
    fn microseconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of milliseconds.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.milliseconds(), Duration::milliseconds(1));
    /// ```
    fn milliseconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of seconds.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.seconds(), Duration::seconds(1));
    /// ```
    fn seconds(&self) -> Duration;

    /// Creates a new Duration from the specified number of minutes.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.minutes(), Duration::minutes(1));
    /// ```
    fn minutes(&self) -> Duration;

    /// Creates a new Duration from the specified number of hours.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.hours(), Duration::hours(1));
    /// ```
    fn hours(&self) -> Duration;

    /// Creates a new Duration from the specified number of days.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.days(), Duration::days(1));
    /// ```
    fn days(&self) -> Duration;

    /// Creates a new Duration from the specified number of weeks.
    ///
    /// ```rust
    /// use num_chrono_duration::NumChronoDuration;
    /// use chrono::Duration;
    ///
    /// assert_eq!(1.weeks(), Duration::weeks(1));
    /// ```
    fn weeks(&self) -> Duration;
}

impl NumChronoDuration for i32 {
    fn nanoseconds(&self) -> Duration {
        Duration::nanoseconds(*self as i64)
    }

    fn microseconds(&self) -> Duration {
        Duration::microseconds(*self as i64)
    }

    fn milliseconds(&self) -> Duration {
        Duration::milliseconds(*self as i64)
    }

    fn seconds(&self) -> Duration {
        Duration::seconds(*self as i64)
    }

    fn minutes(&self) -> Duration {
        Duration::minutes(*self as i64)
    }

    fn hours(&self) -> Duration {
        Duration::hours(*self as i64)
    }

    fn days(&self) -> Duration {
        Duration::days(*self as i64)
    }

    fn weeks(&self) -> Duration {
        Duration::weeks(*self as i64)
    }
}

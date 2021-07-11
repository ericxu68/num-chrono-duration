# num-chrono-duration

This crate provides a convenient way to create `chrono::Duration` from numbers.

Example:

```rust
use num_chrono_duration::NumChronoDuration;

let today = chrono::Utc::today();
assert_eq!(today + 1.hours(), today + chrono::Duration::hours(1));
assert_eq!(today + 1.days(), today + chrono::Duration::days(1));
assert_eq!(today - 1.weeks(), today - chrono::Duration::weeks(1));
```

## Usage

To use `num-chrono-duration`, add this to your `Cargo.toml`:

```toml
[dependencies]
num-chrono-duration = "0.1.0"
```

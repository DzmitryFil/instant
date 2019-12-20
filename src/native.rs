pub type Instant = std::time::Instant;

/// The current time, in milliseconds.
pub fn now() -> f64 {
    time::precise_time_s() * 1000.0
}

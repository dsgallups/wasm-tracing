use bevy_platform::time::Instant;

pub struct Timings {
    idle: u64,
    busy: u64,
    last: Instant,
}

impl Default for Timings {
    fn default() -> Self {
        Self {
            idle: 0,
            busy: 0,
            last: Instant::now(),
        }
    }
}

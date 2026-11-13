use std::time::Instant;

#[derive(Debug)]
pub struct Time {
    started_at: Instant,
    last_update: Instant,
    delta_seconds: f32,
    frame_count: u64,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();

        Self {
            started_at: now,
            last_update: now,
            delta_seconds: 0.0,
            frame_count: 0,
        }
    }

    pub fn tick(&mut self) {
        let now = Instant::now();

        self.delta_seconds = now.duration_since(self.last_update).as_secs_f32();

        self.last_update = now;
        self.frame_count = self.frame_count.saturating_add(1);
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.started_at.elapsed().as_secs_f64()
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_time_is_zero() {
        let time = Time::new();

        assert_eq!(time.frame_count(), 0);
        assert_eq!(time.delta_seconds(), 0.0);
        assert!(time.elapsed_seconds() >= 0.0);
    }

    #[test]
    fn tick_increments_frame_count() {
        let mut time = Time::new();

        time.tick();
        assert_eq!(time.frame_count(), 1);

        time.tick();
        assert_eq!(time.frame_count(), 2);
    }

    #[test]
    fn delta_is_non_negative() {
        let mut time = Time::new();

        for _ in 0..100 {
            time.tick();

            assert!(time.delta_seconds() >= 0.0);
            assert!(time.delta_seconds().is_finite());
        }
    }

    #[test]
    fn elapsed_time_is_monotonic() {
        let time = Time::new();

        let before = time.elapsed_seconds();
        let after = time.elapsed_seconds();

        assert!(after >= before);
    }
}

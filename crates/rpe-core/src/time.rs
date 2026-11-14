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

    /// Resets the frame baseline after a pause without changing frame count.
    pub fn reset_baseline(&mut self) {
        self.last_update = Instant::now();
        self.delta_seconds = 0.0;
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

/// Fixed simulation cadence and catch-up limit.
#[derive(Debug, Clone, Copy)]
pub struct TimingConfig { pub fixed_hz: u32, pub max_steps: u32 }
impl Default for TimingConfig {
    fn default() -> Self { Self { fixed_hz: 60, max_steps: 5 } }
}

/// Accumulates elapsed time and discards whole steps beyond the catch-up limit.
#[derive(Debug)]
pub struct FixedScheduler { config: TimingConfig, accumulator: f64 }
impl FixedScheduler {
    pub fn new(config: TimingConfig) -> Result<Self, crate::EngineError> {
        if config.fixed_hz == 0 || config.max_steps == 0 {
            return Err(crate::EngineError::InvalidConfiguration("fixed_hz and max_steps must be greater than zero".into()));
        }
        Ok(Self { config, accumulator: 0.0 })
    }
    pub fn step_seconds(&self) -> f32 { 1.0 / self.config.fixed_hz as f32 }
    /// Returns (number of updates, discarded seconds).
    pub fn advance(&mut self, delta: f64) -> (u32, f64) {
        if !delta.is_finite() {
            return (0, 0.0);
        }
        self.accumulator += delta.max(0.0);
        let step = 1.0 / f64::from(self.config.fixed_hz);
        let available = (self.accumulator / step).floor();
        let steps = available.min(f64::from(self.config.max_steps)) as u32;
        let remainder = self.accumulator % step;
        let discarded = if available > f64::from(steps) {
            (self.accumulator - remainder - f64::from(steps) * step).max(0.0)
        } else {
            0.0
        };
        self.accumulator = remainder;
        (steps, discarded)
    }
}

#[cfg(test)]
mod fixed_tests {
    use super::*;
    #[test]
    fn validates_and_limits_catch_up() {
        assert!(FixedScheduler::new(TimingConfig { fixed_hz: 0, max_steps: 1 }).is_err());
        assert!(FixedScheduler::new(TimingConfig { fixed_hz: 60, max_steps: 0 }).is_err());
        let mut scheduler = FixedScheduler::new(TimingConfig { fixed_hz: 10, max_steps: 2 }).unwrap();
        assert_eq!(scheduler.advance(0.05).0, 0);
        let (steps, dropped) = scheduler.advance(0.5);
        assert_eq!(steps, 2);
        assert!((dropped - 0.3).abs() < 0.000001);
    }

    #[test]
    fn preserves_fractional_time_after_dropping_excess_steps() {
        let mut scheduler = FixedScheduler::new(TimingConfig { fixed_hz: 10, max_steps: 2 }).unwrap();
        let (steps, dropped) = scheduler.advance(0.55);
        assert_eq!(steps, 2);
        assert!((dropped - 0.3).abs() < 1e-9);
        assert_eq!(scheduler.advance(0.051).0, 1);
    }

    #[test]
    fn ignores_non_finite_deltas_without_poisoning_scheduler() {
        let mut scheduler = FixedScheduler::new(TimingConfig::default()).unwrap();
        assert_eq!(scheduler.advance(f64::INFINITY), (0, 0.0));
        assert_eq!(scheduler.advance(f64::NAN), (0, 0.0));
        assert_eq!(scheduler.advance(1.0 / 60.0).0, 1);
        let (steps, dropped) = scheduler.advance(f64::MAX);
        assert_eq!(steps, 5);
        assert!(dropped.is_finite());
        assert_eq!(scheduler.advance(1.0 / 30.0).0, 2);
    }
}

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

        self.delta_seconds =
            now.duration_since(self.last_update).as_secs_f32();

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
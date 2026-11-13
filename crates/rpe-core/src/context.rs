use crate::time::Time;

pub struct Context {
    time: Time,
    input: rpe_input::InputState,
    logical_width: u32,
    logical_height: u32,
    exit_requested: bool,
}

impl Context {
    pub fn new(logical_width: u32, logical_height: u32) -> Self {
        Self {
            time: Time::new(),
            input: rpe_input::InputState::default(),
            logical_width,
            logical_height,
            exit_requested: false,
        }
    }

    pub fn time(&self) -> &Time {
        &self.time
    }

    pub fn input(&self) -> &rpe_input::InputState {
        &self.input
    }

    /// Mutable access for platform event processing.
    pub fn input_mut(&mut self) -> &mut rpe_input::InputState {
        &mut self.input
    }

    pub fn logical_size(&self) -> (u32, u32) {
        (self.logical_width, self.logical_height)
    }

    pub fn request_exit(&mut self) {
        self.exit_requested = true;
    }

    pub fn is_exit_requested(&self) -> bool {
        self.exit_requested
    }

    /// Resets the time baseline when the application resumes.
    pub fn reset_time_baseline(&mut self) {
        self.time.reset_baseline();
    }

    // Internal engine access.
    pub fn advance_time(&mut self) {
        self.time.tick();
    }
}

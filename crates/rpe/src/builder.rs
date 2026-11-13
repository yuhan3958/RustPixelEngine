use crate::engine::Engine;
use rpe_core::{EngineError, Game};

pub struct EngineBuilder {
    pub(crate) title: String,
    pub(crate) logical_width: u32,
    pub(crate) logical_height: u32,
    pub(crate) window_width: u32,
    pub(crate) window_height: u32,
}

impl Default for EngineBuilder {
    fn default() -> Self {
        Self {
            title: String::from("RustPixelEngine"),
            logical_width: 320,
            logical_height: 180,
            window_width: 1280,
            window_height: 720,
        }
    }
}

impl EngineBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.logical_width = width;
        self.logical_height = height;
        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn run<G: Game>(self) -> Result<(), EngineError> {
        self.validate()?;
        Engine::run::<G>(self)
    }

    fn validate(&self) -> Result<(), EngineError> {
        if self.logical_width == 0 || self.logical_height == 0 {
            return Err(EngineError::InvalidConfiguration(
                "Logical resolution must be greater than zero".into(),
            ));
        }

        if self.window_width == 0 || self.window_height == 0 {
            return Err(EngineError::InvalidConfiguration(
                "Window size must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_configuration_is_valid() {
        assert!(EngineBuilder::new().validate().is_ok());
    }

    #[test]
    fn rejects_zero_resolution() {
        let config = EngineBuilder::new().resolution(0, 180);
        assert!(config.validate().is_err());
    }

    #[test]
    fn rejects_zero_window_size() {
        let config = EngineBuilder::new().window_size(1280, 0);
        assert!(config.validate().is_err());
    }
}

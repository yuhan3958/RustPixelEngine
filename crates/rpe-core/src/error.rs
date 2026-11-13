use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GameError {
    message: String,
}

impl GameError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for GameError {}

#[derive(Debug)]
pub enum EngineError {
    InvalidConfiguration(String),
    Platform(String),
    GameInitialization(GameError),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration(msg) => {
                write!(f, "Invalid engine configuration: {msg}")
            }
            Self::Platform(msg) => {
                write!(f, "Platform error: {msg}")
            }
            Self::GameInitialization(err) => {
                write!(f, "Game initialization failed: {err}")
            }
        }
    }
}

impl Error for EngineError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::GameInitialization(err) => Some(err),
            _ => None,
        }
    }
}

impl From<GameError> for EngineError {
    fn from(error: GameError) -> Self {
        Self::GameInitialization(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn game_error_message() {
        let error = GameError::new("Initialization failed");

        assert_eq!(error.to_string(), "Initialization failed");
    }

    #[test]
    fn game_error_conversion() {
        let game_error = GameError::new("Test error");

        let engine_error: EngineError = game_error.into();

        assert!(matches!(engine_error, EngineError::GameInitialization(_)));
    }

    #[test]
    fn engine_error_preserves_source() {
        let error = EngineError::GameInitialization(GameError::new("Original error"));

        let source = error
            .source()
            .expect("game initialization error should have a source");

        assert_eq!(source.to_string(), "Original error");
    }
}

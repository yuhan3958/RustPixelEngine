use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GameError {
    message: String,
}

impl GameError {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
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
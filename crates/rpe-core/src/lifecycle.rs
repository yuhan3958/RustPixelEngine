#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleState {
    Created,
    Initializing,
    Running,
    Stopping,
    Stopped,
}

impl LifecycleState {
    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Created, Self::Initializing)
                | (Self::Initializing, Self::Running)
                | (Self::Initializing, Self::Stopping)
                | (Self::Running, Self::Stopping)
                | (Self::Stopping, Self::Stopped)
        )
    }

    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Stopped)
    }

    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidLifecycleTransition {
    pub from: LifecycleState,
    pub to: LifecycleState,
}

impl std::fmt::Display for InvalidLifecycleTransition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid lifecycle transition: {:?} -> {:?}",
            self.from, self.to
        )
    }
}

impl std::error::Error for InvalidLifecycleTransition {}

#[derive(Debug)]
pub struct Lifecycle {
    state: LifecycleState,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl Lifecycle {
    pub const fn new() -> Self {
        Self {
            state: LifecycleState::Created,
        }
    }

    pub const fn state(&self) -> LifecycleState {
        self.state
    }

    pub fn transition(&mut self, next: LifecycleState) -> Result<(), InvalidLifecycleTransition> {
        if !self.state.can_transition_to(next) {
            return Err(InvalidLifecycleTransition {
                from: self.state,
                to: next,
            });
        }

        self.state = next;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_is_created() {
        let lifecycle = Lifecycle::new();

        assert_eq!(lifecycle.state(), LifecycleState::Created);
    }

    #[test]
    fn valid_lifecycle_transitions() {
        let mut lifecycle = Lifecycle::new();

        for next in [
            LifecycleState::Initializing,
            LifecycleState::Running,
            LifecycleState::Stopping,
            LifecycleState::Stopped,
        ] {
            assert!(lifecycle.transition(next).is_ok());
        }

        assert!(lifecycle.state().is_terminal());
    }

    #[test]
    fn initialization_failure_transition() {
        let mut lifecycle = Lifecycle::new();

        lifecycle
            .transition(LifecycleState::Initializing)
            .expect("initializing transition should succeed");

        assert!(lifecycle.transition(LifecycleState::Stopping).is_ok());

        assert!(lifecycle.transition(LifecycleState::Stopped).is_ok());
    }

    #[test]
    fn invalid_transition_does_not_change_state() {
        let mut lifecycle = Lifecycle::new();

        let result = lifecycle.transition(LifecycleState::Running);

        assert!(result.is_err());
        assert_eq!(lifecycle.state(), LifecycleState::Created);
    }

    #[test]
    fn terminal_state_cannot_restart() {
        let mut lifecycle = Lifecycle::new();

        for next in [
            LifecycleState::Initializing,
            LifecycleState::Running,
            LifecycleState::Stopping,
            LifecycleState::Stopped,
        ] {
            lifecycle
                .transition(next)
                .expect("valid transition should succeed");
        }

        assert!(lifecycle.transition(LifecycleState::Running).is_err());
    }
}

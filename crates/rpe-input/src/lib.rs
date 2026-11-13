#![forbid(unsafe_code)]
use std::collections::HashSet;

/// Physical keyboard keys supported by the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Escape,
    Space,
    Enter,
    Tab,
    Backspace,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,
}

#[derive(Debug, Default)]
pub struct InputState {
    pressed: HashSet<Key>,
    just_pressed: HashSet<Key>,
    just_released: HashSet<Key>,
}
impl InputState {
    pub fn pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }
    pub fn just_pressed(&self, key: Key) -> bool {
        self.just_pressed.contains(&key)
    }
    pub fn just_released(&self, key: Key) -> bool {
        self.just_released.contains(&key)
    }
    pub fn press(&mut self, key: Key) {
        if self.pressed.insert(key) {
            self.just_pressed.insert(key);
        }
    }
    pub fn release(&mut self, key: Key) {
        if self.pressed.remove(&key) {
            self.just_released.insert(key);
        }
    }
    /// Releases all held keys while preserving transitions for the next update.
    pub fn release_all(&mut self) {
        self.just_released.extend(self.pressed.drain());
    }
    pub fn end_frame(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn transitions_and_repeats_are_retained_until_frame_end() {
        let mut input = InputState::default();
        input.press(Key::A);
        input.press(Key::A);
        input.release(Key::A);
        assert!(!input.pressed(Key::A));
        assert!(input.just_pressed(Key::A));
        assert!(input.just_released(Key::A));
        input.end_frame();
        assert!(!input.just_pressed(Key::A));
        assert!(!input.just_released(Key::A));
    }
    #[test]
    fn focus_loss_releases_held_keys() {
        let mut input = InputState::default();
        input.press(Key::A);
        input.end_frame();
        input.release_all();
        assert!(!input.pressed(Key::A));
        assert!(input.just_released(Key::A));
    }
}

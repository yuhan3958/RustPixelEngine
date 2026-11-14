#![forbid(unsafe_code)]
use std::collections::{HashMap, HashSet};

/// Physical keyboard keys supported by the runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

/// Mouse buttons supported by the platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MouseButton { Left, Right, Middle, Back, Forward, Other(u16) }
/// A physical input that can be bound to an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button { Key(Key), Mouse(MouseButton) }

/// An input event in the order it was received by the engine.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputEvent {
    Pressed(Button),
    Released(Button),
    /// Mouse position at the time of the event, or `None` after leaving the window.
    CursorMoved { physical: Option<(f64, f64)>, logical: Option<(f64, f64)> },
}

#[derive(Debug, Clone, Default)]
pub struct InputState {
    fixed_mode: bool,
    frame_events: Vec<InputEvent>,
    fixed_events: Vec<InputEvent>,
    fixed_pressed_edges: HashSet<Key>,
    fixed_released_edges: HashSet<Key>,
    fixed_mouse_pressed_edges: HashSet<MouseButton>,
    fixed_mouse_released_edges: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_just_pressed: HashSet<MouseButton>,
    mouse_just_released: HashSet<MouseButton>,
    position: Option<(f64, f64)>,
    logical_position: Option<(f64, f64)>,
    actions: HashMap<String, Vec<Button>>,
    pressed: HashSet<Key>,
    just_pressed: HashSet<Key>,
    just_released: HashSet<Key>,
}
impl InputState {
    /// Input events in collection order for the current consumer.
    /// Fixed updates see events accumulated since the previous fixed step;
    /// variable updates see events since the previous variable update.
    pub fn events(&self) -> &[InputEvent] {
        if self.fixed_mode { &self.fixed_events } else { &self.frame_events }
    }

    /// Number of events awaiting the next fixed update.
    #[doc(hidden)]
    pub fn pending_fixed_events(&self) -> usize { self.fixed_events.len() }

    fn record_event(&mut self, event: InputEvent) {
        self.frame_events.push(event);
        self.fixed_events.push(event);
    }

    pub fn bind_action(&mut self, name: impl Into<String>, buttons: Vec<Button>) { self.actions.insert(name.into(), buttons); }
    pub fn action_pressed(&self, name: &str) -> bool {
        self.actions.get(name).is_some_and(|buttons| buttons.iter().any(|button| match button {
            Button::Key(key) => self.pressed(*key), Button::Mouse(button) => self.mouse_pressed(*button),
        }))
    }
    pub fn mouse_pressed(&self, button: MouseButton) -> bool { self.mouse_pressed.contains(&button) }
    pub fn mouse_just_pressed(&self, button: MouseButton) -> bool { if self.fixed_mode { self.fixed_mouse_pressed_edges.contains(&button) } else { self.mouse_just_pressed.contains(&button) } }
    pub fn mouse_just_released(&self, button: MouseButton) -> bool { if self.fixed_mode { self.fixed_mouse_released_edges.contains(&button) } else { self.mouse_just_released.contains(&button) } }
    pub fn mouse_position(&self) -> Option<(f64, f64)> { self.position }
    pub fn logical_mouse_position(&self) -> Option<(f64, f64)> { self.logical_position }
    pub fn set_mouse_position(&mut self, position: Option<(f64, f64)>, window: (u32, u32), logical: (u32, u32)) {
        self.position = position;
        self.logical_position = position.and_then(|(x,y)| {
            if logical.0 == 0 || logical.1 == 0 { return None; }
            let scale = (window.0 / logical.0).min(window.1 / logical.1);
            if scale == 0 { return None; }
            let left = (window.0 - logical.0 * scale) / 2;
            let top = (window.1 - logical.1 * scale) / 2;
            let x = (x - f64::from(left)) / f64::from(scale);
            let y = (y - f64::from(top)) / f64::from(scale);
            if x >= 0.0 && y >= 0.0 && x < f64::from(logical.0) && y < f64::from(logical.1) { Some((x,y)) } else { None }
        });
    }
    /// Records a platform cursor event and updates the current position.
    #[doc(hidden)]
    pub fn record_mouse_position(&mut self, position: Option<(f64, f64)>, window: (u32, u32), logical: (u32, u32)) {
        self.set_mouse_position(position, window, logical);
        self.record_event(InputEvent::CursorMoved { physical: position, logical: self.logical_position });
    }
    pub fn press_mouse(&mut self, button: MouseButton) { if self.mouse_pressed.insert(button) { self.mouse_just_pressed.insert(button); self.fixed_mouse_pressed_edges.insert(button); self.record_event(InputEvent::Pressed(Button::Mouse(button))); } }
    pub fn release_mouse(&mut self, button: MouseButton) { if self.mouse_pressed.remove(&button) { self.mouse_just_released.insert(button); self.fixed_mouse_released_edges.insert(button); self.record_event(InputEvent::Released(Button::Mouse(button))); } }
    pub fn pressed(&self, key: Key) -> bool {
        self.pressed.contains(&key)
    }
    pub fn just_pressed(&self, key: Key) -> bool {
        if self.fixed_mode { self.fixed_pressed_edges.contains(&key) } else { self.just_pressed.contains(&key) }
    }
    pub fn just_released(&self, key: Key) -> bool {
        if self.fixed_mode { self.fixed_released_edges.contains(&key) } else { self.just_released.contains(&key) }
    }
    pub fn press(&mut self, key: Key) {
        if self.pressed.insert(key) {
            self.just_pressed.insert(key);
            self.fixed_pressed_edges.insert(key);
            self.record_event(InputEvent::Pressed(Button::Key(key)));
        }
    }
    pub fn release(&mut self, key: Key) {
        if self.pressed.remove(&key) {
            self.just_released.insert(key);
            self.fixed_released_edges.insert(key);
            self.record_event(InputEvent::Released(Button::Key(key)));
        }
    }
    /// Releases held buttons. Synthetic releases use key order, then mouse-button order.
    pub fn release_all(&mut self) {
        let mut keys: Vec<_> = self.pressed.iter().copied().collect();
        keys.sort_unstable();
        for key in keys { self.release(key); }
        let mut buttons: Vec<_> = self.mouse_pressed.iter().copied().collect();
        buttons.sort_unstable();
        for button in buttons { self.release_mouse(button); }
    }
    #[doc(hidden)]
    pub fn set_fixed_mode(&mut self, enabled: bool) { self.fixed_mode = enabled; }
    #[doc(hidden)]
    pub fn end_fixed_step(&mut self) {
        self.fixed_events.clear();
        self.fixed_pressed_edges.clear(); self.fixed_released_edges.clear();
        self.fixed_mouse_pressed_edges.clear(); self.fixed_mouse_released_edges.clear();
    }
    pub fn end_frame(&mut self) {
        self.frame_events.clear();
        self.mouse_just_pressed.clear();
        self.mouse_just_released.clear();
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

#[cfg(test)]
mod fixed_input_tests {
    use super::*;
    #[test]
    fn fixed_edges_survive_frames_and_are_consumed_once() {
        let mut input = InputState::default();
        input.press(Key::A); input.release(Key::A); input.end_frame();
        input.set_fixed_mode(true);
        assert!(input.just_pressed(Key::A)); assert!(input.just_released(Key::A));
        input.end_fixed_step();
        assert!(!input.just_pressed(Key::A)); assert!(!input.just_released(Key::A));
    }
    #[test]
    fn viewport_and_action_mapping() {
        let mut input = InputState::default();
        input.set_mouse_position(Some((0.0, 0.0)), (800, 600), (320, 180));
        assert_eq!(input.logical_mouse_position(), None);
        input.set_mouse_position(Some((80.0, 120.0)), (800, 600), (320, 180));
        assert_eq!(input.logical_mouse_position(), Some((0.0, 0.0)));
        input.bind_action("fire", vec![Button::Key(Key::A), Button::Mouse(MouseButton::Left)]);
        input.press(Key::A); input.press_mouse(MouseButton::Left); input.release(Key::A);
        assert!(input.action_pressed("fire"));
    }
}

#[cfg(test)]
mod ordered_event_tests {
    use super::*;

    #[test]
    fn events_preserve_order_across_frames_for_fixed_updates() {
        let mut input = InputState::default();
        input.press(Key::A);
        input.end_frame();
        input.press_mouse(MouseButton::Left);
        input.release(Key::A);
        input.press(Key::A);
        let expected = [InputEvent::Pressed(Button::Key(Key::A)), InputEvent::Pressed(Button::Mouse(MouseButton::Left)), InputEvent::Released(Button::Key(Key::A)), InputEvent::Pressed(Button::Key(Key::A))];
        assert_eq!(input.events(), &expected[1..]);
        input.set_fixed_mode(true);
        assert_eq!(input.events(), &expected);
        input.end_fixed_step();
        assert!(input.events().is_empty());
        input.set_fixed_mode(false);
        assert_eq!(input.events(), &expected[1..]);
    }

    #[test]
    fn repeated_presses_are_ignored_and_focus_loss_records_releases() {
        let mut input = InputState::default();
        input.press(Key::A);
        input.press(Key::A);
        input.press_mouse(MouseButton::Left);
        input.end_frame();
        input.release_all();
        assert_eq!(input.events(), &[InputEvent::Released(Button::Key(Key::A)), InputEvent::Released(Button::Mouse(MouseButton::Left))]);
    }

    #[test]
    fn cursor_and_button_events_keep_their_collection_order() {
        let mut input = InputState::default();
        input.record_mouse_position(Some((10.0, 20.0)), (320, 180), (320, 180));
        input.press_mouse(MouseButton::Left);
        input.record_mouse_position(None, (320, 180), (320, 180));
        assert_eq!(input.events(), &[
            InputEvent::CursorMoved { physical: Some((10.0, 20.0)), logical: Some((10.0, 20.0)) },
            InputEvent::Pressed(Button::Mouse(MouseButton::Left)),
            InputEvent::CursorMoved { physical: None, logical: None },
        ]);
        input.end_frame();
        input.set_fixed_mode(true);
        assert_eq!(input.events().len(), 3);
        input.end_fixed_step();
        assert!(input.events().is_empty());
    }
}

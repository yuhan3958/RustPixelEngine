

## Runtime foundation

`Context::input()` exposes physical keyboard state through `Key`, `pressed`,
`just_pressed`, and `just_released`. Transitions are retained until the next
update finishes, including press and release in the same update interval.
Focus loss releases held keys. The current key mapping covers letters, digits,
common navigation and modifier keys; unmapped physical keys are ignored.

The core and platform crates depend on `rpe-input` for platform-independent key
state and event conversion. The public crate uses the existing workspace `log`
dependency for diagnostics and does not install a logger. Platform errors from
the engine preserve their underlying source via `EngineError::PlatformSource`;
the existing `Platform(String)` variant remains available.

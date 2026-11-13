# RustPixelEngine
## Milestone 001 — Runtime Foundation

**상태:** 완료 (사용자 확인 기준)  
**Workspace 버전:** 0.1.0  
**언어:** Rust 2024 Edition  
**플랫폼:** winit 0.30 기반, Windows 우선

## 1. 목표와 범위

Rust 코드에서 창을 생성하고 게임 생명주기, 시간, 키보드 입력과 종료를 처리하는 실행 기반을 제공한다. 논리 해상도는 설정값으로 보관하며 화면에 픽셀을 출력하지 않는다.

렌더링, 프레임버퍼, 이미지와 리소스 로딩, 오디오, 물리, 씬, ECS, UI, 마우스 및 게임패드 입력은 이 마일스톤의 범위 밖이다.

## 2. 크레이트 구성

| 크레이트 | 현재 책임 |
|---|---|
| `rpe` | 공개 진입점, Builder, 런타임과 prelude |
| `rpe-core` | Game, Context, 생명주기, 시간과 오류 |
| `rpe-platform` | winit 이벤트 루프, 창 속성, 물리 키 변환 |
| `rpe-input` | 키와 입력 상태 및 전이 |
| `rpe-math` | Size2D와 glam 벡터 타입 재노출 |

나머지 크레이트는 Workspace에 포함되어 있으며 후속 기능을 위한 자리다. `Context`의 크기 API는 `Size2D`가 아닌 `(u32, u32)`를 반환한다.

## 3. 공개 진입점

```rust
use rust_pixel_engine::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn init(_ctx: &mut Context) -> Result<Self, GameError> {
        Ok(Self)
    }

    fn update(&mut self, ctx: &mut Context, _dt: f32) {
        if ctx.input().just_pressed(Key::Escape) {
            ctx.request_exit();
        }
    }
}

fn main() -> Result<(), EngineError> {
    Engine::builder()
        .title("RustPixelEngine")
        .resolution(320, 180)
        .window_size(1280, 720)
        .run::<MyGame>()
}
```

`init`은 필수이며 `update`와 `shutdown`에는 기본 구현이 있다. prelude는 Engine, EngineBuilder, Context, Game, GameError, EngineError, Time, InputState, Key를 재노출한다.

### Builder 기본값과 검증

| 설정 | 기본값 |
|---|---|
| 제목 | RustPixelEngine |
| 논리 해상도 | 320 × 180 |
| 창 크기 | 1280 × 720 |

`run`은 논리 해상도와 창 크기의 각 차원이 0보다 큰지 검증한 뒤 이벤트 루프를 시작한다. 창 크기는 winit의 논리 크기로 전달된다. 크기 변경 여부를 설정하는 Builder API는 없다.

## 4. 생명주기와 이벤트 루프

내부 생명주기는 `Created → Initializing → Running → Stopping → Stopped` 순서로 진행한다. 초기화가 실패하면 종료 경로로 이동하며 게임 객체가 없으므로 `shutdown`을 호출하지 않는다. 초기화에 성공한 게임은 정상 종료 경로에서 한 번 정리된다. 패닉이나 강제 프로세스 종료 시 정리는 보장되지 않는다.

winit의 `ApplicationHandler`에서 다음을 처리한다.

- `resumed`: 시간 기준을 재설정하고 필요한 경우 창과 게임을 생성한다.
- `window_event`: 창 닫기, 크기 변경, 포커스와 물리 키 이벤트를 처리한다.
- `about_to_wait`: 실행 중인 게임을 갱신하고 다음 갱신까지 대기한다.
- `suspended`: 갱신을 중단하고 눌린 키를 해제한다.
- `exiting`: 게임과 창을 정리한다. 이벤트 루프가 오류로 반환되는 경우에도 정리를 시도한다.

갱신 간격은 내부적으로 약 16.67ms이며 `WaitUntil`로 대기한다. 지연된 갱신을 보충하는 반복은 없고, 이는 고정 시간 간격 시뮬레이션이나 공개 FPS 제한 기능이 아니다.

## 5. Context와 시간

`Context`는 `time()`, `input()`, `logical_size()`, `window_size()`, `is_focused()`, `request_exit()`, `is_exit_requested()`를 제공한다. 창 크기는 마지막으로 보고된 물리 픽셀 크기다.

`Time`은 `Instant`를 사용한다. `delta_seconds()`는 직전 기준 시점부터의 간격을 `f32` 초로, `elapsed_seconds()`는 생성 이후 경과 시간을 `f64` 초로, `frame_count()`는 갱신 횟수를 반환한다. 재개 시 delta 기준을 초기화하지만 누적 시간과 프레임 수는 초기화하지 않는다. 따라서 누적 시간에는 중단 기간도 포함된다.

## 6. 키보드 입력

`Context::input()`에서 `pressed`, `just_pressed`, `just_released`를 조회한다. 입력 전이는 게임 업데이트가 끝날 때까지 유지되므로 같은 구간에 누르고 놓은 키는 두 전이가 모두 기록된다. 반복 눌림은 새로운 전이를 만들지 않는다. 포커스 상실이나 애플리케이션 중단 시 눌린 키를 해제한다.

현재 물리 키 매핑은 영문자, 숫자, Escape, Space, Enter, Tab, Backspace, 방향키와 좌우 Shift·Control·Alt를 포함한다. 매핑되지 않은 키는 무시한다.

## 7. 오류와 로그

`EngineError`는 `InvalidConfiguration`, `Platform`, `PlatformSource`, `GameInitialization`을 제공한다. 플랫폼 원본 오류와 게임 초기화 오류는 오류 원인을 보존한다. `GameError::new`로 초기화 오류를 만들 수 있다.

엔진 진단에는 `log`를 사용하며 전역 로거 설치는 애플리케이션에 맡긴다.

## 8. 예제와 검증

- `../../examples/hello-window`: 창, 시간, 생명주기와 종료 요청 예제.
- `../../crates/rpe/examples/keyboard.rs`: 방향키, Space 전이와 Escape 종료 예제.
- 코드에는 Builder 설정, 생명주기, 시간, 입력 전이, 키 변환과 오류를 다루는 단위 테스트가 있다.

이 문서 개정에서는 빌드, 테스트, 예제를 실행하지 않았다. 따라서 테스트 통과나 GUI 동작은 이번 개정에서 별도로 검증하지 않았다.

## 9. 후속 단계

Milestone 002에서는 CPU 픽셀 렌더러, 프레임버퍼, 색상, 기본 도형, 알파 블렌딩과 화면 출력 및 정수 배율을 다룬다. 고정 시간 간격 업데이트는 후속 마일스톤에서 다룬다.

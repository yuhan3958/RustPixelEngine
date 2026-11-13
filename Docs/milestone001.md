# RustPixelEngine
## Milestone 001 — Runtime Foundation

**프로젝트:** RustPixelEngine  
**마일스톤:** 001  
**버전:** v0.1.0-dev  
**상태:** Planned  
**개발 언어:** Rust 2024 Edition  
**대상 플랫폼:** Windows 우선, Linux/macOS 확장 고려  
**선행 조건:** Cargo Workspace 및 각 크레이트 Manifest 구성

---

# 1. 마일스톤 개요

## 1.1. 목표

Milestone 001의 목표는 RustPixelEngine의 최소 실행 환경을 구현하는 것이다.

사용자는 Rust 코드로 엔진을 초기화하고, 운영체제 창을 생성하며, 게임 루프를 실행하고, 정상적으로 종료할 수 있어야 한다.

이 단계에서는 렌더링 기능을 구현하지 않는다.

궁극적으로 다음 코드와 유사한 형태로 엔진을 실행할 수 있어야 한다.

```rust
use rust_pixel_engine::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn init(_ctx: &mut Context) -> Result<Self, GameError> {
        println!("Game initialized");
        Ok(Self)
    }

    fn update(&mut self, _ctx: &mut Context, dt: f32) {
        println!("Delta time: {dt}");
    }

    fn shutdown(&mut self, _ctx: &mut Context) {
        println!("Game shutdown");
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

위 코드는 Milestone 001의 **구현 목표 API**다.

렌더러가 아직 없기 때문에 논리 해상도는 설정값으로만 보관하며, 실제로 320×180의 이미지를 출력하지 않는다.

## 1.2. 완료 결과

- Cargo Workspace 정상 빌드
- 운영체제 창 생성
- 엔진 초기화 및 종료
- 이벤트 루프 실행
- 프레임 시간 계산
- 게임 업데이트 콜백 실행
- 창 닫기 이벤트 처리
- 오류의 명시적 전달
- 정상 종료 시 리소스 정리

---

# 2. 개발 범위

## 2.1. 구현 대상

| ID | 시스템 | 구현 내용 |
|---|---|---|
| RPE-001 | Engine | 공개 진입점 및 Builder |
| RPE-002 | Core | 게임 생명주기 및 Context |
| RPE-003 | Platform | 창 생성 및 이벤트 루프 |
| RPE-004 | Time | Delta Time 및 경과 시간 |
| RPE-005 | Input | 키보드 입력 상태 관리 |
| RPE-006 | Math | 기본 좌표 및 크기 자료형 |
| RPE-007 | Error | 초기화 및 실행 오류 처리 |
| RPE-008 | Logging | 엔진 진단 로그 |
| RPE-009 | Tests | 생명주기와 시간 계산 검증 |

## 2.2. 제외 범위

이번 마일스톤에서 다음 기능은 구현하지 않는다.

- CPU/GPU/Hybrid 렌더러
- 프레임버퍼
- 스프라이트
- 이미지 및 리소스 로딩
- 타일맵
- 물리 엔진
- 오디오
- 씬 관리
- ECS
- UI
- 게임패드
- 게임 프레임 제한 및 VSync
- 실제 픽셀 출력

**범위 원칙:** Milestone 001의 성공 기준은 게임을 그릴 수 있는지가 아니라, 게임을 실행할 수 있는지이다.

---

# 3. 대상 모듈

이번 마일스톤에서는 5개 크레이트의 실제 기능을 구현한다.

```text
crates/
├── rpe/
│   └── src/
│       ├── lib.rs
│       ├── engine.rs
│       ├── builder.rs
│       └── prelude.rs
│
├── rpe-core/
│   └── src/
│       ├── lib.rs
│       ├── game.rs
│       ├── context.rs
│       ├── lifecycle.rs
│       ├── time.rs
│       └── error.rs
│
├── rpe-platform/
│   └── src/
│       ├── lib.rs
│       ├── application.rs
│       ├── window.rs
│       └── event.rs
│
├── rpe-input/
│   └── src/
│       ├── lib.rs
│       ├── keyboard.rs
│       ├── key.rs
│       └── state.rs
│
└── rpe-math/
    └── src/
        ├── lib.rs
        └── size.rs
```

나머지 크레이트는 Workspace에 등록된 상태를 유지한다.

전체 Workspace를 빌드하려면 아직 구현하지 않은 크레이트에도 최소 `src/lib.rs`가 존재해야 한다.

## 3.1. 모듈 책임

### rpe

사용자에게 노출되는 엔진의 진입점.

- `Engine`
- `EngineBuilder`
- `EngineError` 재노출
- `Game` 재노출
- `prelude`

최상위 크레이트에서 각 시스템을 조립한다.

### rpe-core

플랫폼에 독립적인 엔진 실행 규약.

- `Game` trait
- `Context`
- `LifecycleState`
- `Time`
- `GameError`
- `EngineError`

운영체제 창 생성이나 이벤트 루프 구현은 포함하지 않는다.

### rpe-platform

운영체제별 창 및 이벤트 처리 추상화.

- `WindowConfig`
- `PlatformApplication`
- 창 생성과 종료
- `winit` 이벤트 수신
- 플랫폼 이벤트를 공통 이벤트로 변환

`rpe-platform`이 `rpe`를 의존하지 않도록 구성한다.

### rpe-input

키보드 상태 추적.

- `Key`
- `InputState`
- `pressed`
- `just_pressed`
- `just_released`

플랫폼 이벤트 타입을 직접 노출하지 않는 구조를 지향한다.

### rpe-math

기본 자료형 제공.

- `Size2D`
- `UVec2` 등 수학 타입 재노출
- 논리 해상도 검증용 자료형

기존 `glam` 의존성을 활용하며, 필요하지 않은 수학 기능은 추가하지 않는다.

---

# 4. Engine Builder

## 4.1. 역할

`EngineBuilder`는 엔진 실행에 필요한 설정을 수집하고 검증한다.

Builder는 설정 객체만 생성하며, 운영체제 창 생성은 실제 이벤트 루프 안에서 수행한다.

## 4.2. API

```rust
pub struct EngineBuilder {
    title: String,
    logical_width: u32,
    logical_height: u32,
    window_width: u32,
    window_height: u32,
}

impl EngineBuilder {
    pub fn title(self, title: impl Into<String>) -> Self;

    pub fn resolution(self, width: u32, height: u32) -> Self;

    pub fn window_size(self, width: u32, height: u32) -> Self;

    pub fn run<G: Game>(self) -> Result<(), EngineError>;
}
```

위 코드는 인터페이스 명세이며, 실제 함수 본문은 구현 과정에서 작성한다.

## 4.3. 기본값

| 설정 | 기본값 |
|---|---|
| Title | RustPixelEngine |
| Logical Width | 320 |
| Logical Height | 180 |
| Window Width | 1280 |
| Window Height | 720 |
| Resizable | true |
| Fixed Update | 비활성화 |
| Logging | 애플리케이션 설정에 따름 |

## 4.4. 설정 검증

다음 조건을 검사한다.

- 창 너비와 높이는 0보다 커야 한다.
- 논리 해상도의 너비와 높이는 0보다 커야 한다.
- 플랫폼에서 지원하지 않는 창 설정은 오류로 반환한다.
- 게임 또는 플랫폼 초기화 실패 시 명시적 오류를 반환한다.

---

# 5. Game Lifecycle

## 5.1. 상태 정의

```rust
pub enum LifecycleState {
    Created,
    Initializing,
    Running,
    Stopping,
    Stopped,
}
```

상태 전이는 다음과 같다.

```text
Created
   |
   v
Initializing
   |
   v
Running
   |
   v
Stopping
   |
   v
Stopped
```

초기화 실패 시 `Initializing`에서 종료 경로로 이동한다.

초기화에 성공하지 않은 게임 객체에는 `shutdown`을 호출하지 않는다.

## 5.2. Game Trait

```rust
pub trait Game: Sized + 'static {
    fn init(
        ctx: &mut Context,
    ) -> Result<Self, GameError>;

    fn update(
        &mut self,
        _ctx: &mut Context,
        _dt: f32,
    ) {}

    fn shutdown(
        &mut self,
        _ctx: &mut Context,
    ) {}
}
```

**Milestone 001에서는 세 메서드만 필수 실행 대상으로 구현한다.**

향후 `fixed_update`, `render` 등을 추가할 수 있도록 실행 루프를 구성한다.

렌더링 모듈이 아직 없으므로 `render` 콜백은 이번 단계의 Game trait에 포함하지 않는다.

## 5.3. Callback 호출 규칙

**init**

- 게임 인스턴스 생성 시 한 번 호출
- Context 접근 가능
- 초기화 오류 반환 가능
- 성공 시 Running 상태 전환

**update**

- 게임이 Running 상태일 때 호출
- 프레임 간 경과 시간을 초 단위로 전달
- 게임 상태 갱신에 사용
- 종료 요청 이후에는 호출하지 않음

**shutdown**

- 정상적으로 초기화한 게임에 대해 한 번 호출
- 정상적인 이벤트 루프 종료 경로에서 실행
- 게임 종료 시 리소스 정리 가능

강제 프로세스 종료나 패닉 등 비정상 종료 상황에서는 shutdown 호출을 보장하지 않는다.

---

# 6. Context

## 6.1. 목적

Context는 게임 실행에 필요한 공통 실행 상태에 대한 접근을 제공한다.

게임은 전역 싱글턴 대신 Context를 통해 엔진과 상호작용한다.

## 6.2. 구조

```rust
pub struct Context {
    time: Time,
    input: InputState,
    logical_size: Size2D,
    exit_requested: bool,
}
```

실제 필드 공개 여부와 내부 소유권 구조는 구현 시 조정할 수 있다.

## 6.3. 공개 API

```rust
impl Context {
    pub fn time(&self) -> &Time;

    pub fn input(&self) -> &InputState;

    pub fn logical_size(&self) -> Size2D;

    pub fn request_exit(&mut self);

    pub fn is_exit_requested(&self) -> bool;
}
```

Context가 운영체제 창이나 GPU 장치를 직접 소유하도록 설계하지 않는다.

---

# 7. Platform Application

## 7.1. 구현 기술

Windows를 우선 지원하며 창과 이벤트 처리는 `winit` 기반으로 구현한다.

winit 0.30 계열의 `ApplicationHandler` 구조에 맞춰 실행 루프를 설계한다.

주요 이벤트는 다음과 같다.

- `resumed`
- `window_event`
- `about_to_wait`
- `exiting`

## 7.2. 실행 원칙

운영체제 창은 플랫폼에서 허용하는 이벤트 루프 실행 시점에 생성한다.

최상위 엔진은 플랫폼 애플리케이션에 게임 생명주기 처리를 위임할 수 있도록 콜백 또는 Runtime Adapter를 연결한다.

단, 플랫폼 계층 자체가 구체적인 게임 타입을 알아야 하는 구조는 피한다.

## 7.3. 실행 루프

```text
Engine::run()
    |
    v
Validate Config
    |
    v
Create EventLoop
    |
    v
Start Platform Application
    |
    v
Platform Resumed
    |
    v
Create Window
    |
    v
Initialize Game
    |
    v
Process Events
    |
    v
Update Input State
    |
    v
Update Time
    |
    v
Game::update()
    |
    v
Check Exit Request
    |
    +---- Continue
    |
    v
Game::shutdown()
    |
    v
Exit EventLoop
```

실제 이벤트 루프의 업데이트는 `about_to_wait` 등 적절한 이벤트 지점에서 진행한다.

렌더러가 없는 이번 마일스톤에서는 GPU Present나 Redraw 처리가 필요하지 않다.

무제한 업데이트로 CPU 한 코어를 계속 점유하지 않도록 이벤트 대기 또는 임시 업데이트 주기 정책을 둔다. 이 정책은 게임의 공식 FPS 제한 기능과 구별한다.

---

# 8. Time System

## 8.1. 목적

프레임 간 경과 시간을 측정하고 게임 업데이트에 제공한다.

## 8.2. Time 구조

```rust
pub struct Time {
    delta_seconds: f32,
    elapsed_seconds: f64,
    frame_count: u64,
}
```

내부 시간 측정에는 `std::time::Instant`를 사용한다.

시각 계산에 시스템 벽시계 시간을 사용하지 않는다.

## 8.3. 시간 갱신 규칙

- 첫 업데이트에서는 Delta Time을 0으로 설정할 수 있다.
- 이후 업데이트에서는 직전 갱신 시점과 현재 시점의 간격을 측정한다.
- Delta Time은 음수가 될 수 없다.
- 프레임 카운트는 게임 업데이트마다 증가한다.
- 누적 시간은 `f64`로 관리하여 장시간 실행 시 정밀도 손실을 줄인다.

## 8.4. API

```rust
impl Time {
    pub fn delta_seconds(&self) -> f32;

    pub fn elapsed_seconds(&self) -> f64;

    pub fn frame_count(&self) -> u64;
}
```

고정 시간 간격 업데이트는 Milestone 003에서 추가한다.

---

# 9. Input System

## 9.1. 목적

키보드 입력의 현재 상태와 상태 변화를 추적한다.

## 9.2. 입력 상태

| 상태 | 의미 |
|---|---|
| pressed | 현재 키가 눌려 있음 |
| just_pressed | 이번 업데이트 구간에서 눌림 이벤트 발생 |
| just_released | 이번 업데이트 구간에서 해제 이벤트 발생 |

## 9.3. 처리 원칙

- 입력 이벤트는 플랫폼 계층에서 전달받는다.
- 입력 상태는 엔진의 논리적인 키 타입으로 저장한다.
- 키 눌림 및 해제 전이는 업데이트 경계에 맞춰 처리한다.
- `just_pressed`, `just_released`는 확인 전에 사라지지 않아야 한다.
- 창 포커스를 잃으면 눌림 상태를 정리하여 키가 계속 눌린 것으로 남지 않도록 한다.

이벤트가 한 번의 업데이트 구간에 여러 번 발생할 수 있으므로, 최종 키 상태와 전이 여부를 구분해서 저장한다.

## 9.4. 범위

이번 마일스톤에서는 기본 키보드 입력만 구현한다.

마우스, 게임패드, 입력 액션 매핑은 후속 마일스톤으로 이관한다.

---

# 10. Error Handling

## 10.1. 오류 유형

초기 오류 체계는 다음 범주를 지원한다.

```rust
pub enum EngineError {
    InvalidConfiguration(String),
    Platform(String),
    GameInitialization(GameError),
    Runtime(String),
}
```

구체적인 필드 타입은 구현 시 표준 Error trait과의 호환성을 고려한다.

## 10.2. 오류 정책

- 예상 가능한 실행 오류에는 `Result`를 사용한다.
- 사용자 입력에 대한 오류는 panic으로 처리하지 않는다.
- 오류 발생 시 가능한 한 정상 종료 경로로 이동한다.
- 불필요한 `unwrap()`을 피한다.
- 오류 메시지는 발생한 계층과 원인을 식별할 수 있도록 작성한다.

게임 초기화 실패는 GameError에서 EngineError로 변환한다.

---

# 11. Logging

## 11.1. 기본 원칙

엔진 내부 로그는 `log` 크레이트를 사용한다.

로그 출력기의 초기화는 기본적으로 최상위 애플리케이션의 책임으로 둔다.

엔진은 자체 전역 로거 설치를 강제하지 않는다.

## 11.2. 로그 이벤트

- 엔진 시작
- 설정 검증 완료
- 이벤트 루프 시작
- 창 생성 성공
- 게임 초기화 성공
- 종료 요청 수신
- 게임 종료 완료
- 이벤트 루프 종료
- 초기화 및 실행 오류

매 프레임 로그 출력은 기본적으로 수행하지 않는다.

필요한 경우 Debug 또는 Trace 레벨을 사용한다.

---

# 12. 개발 작업 순서

## Task 001 — Workspace Verification

**구현 작업**

1. 전체 Cargo Manifest 검증
2. 크레이트별 최소 lib.rs 구성
3. Workspace 의존 관계 확인
4. Rust 2024 Edition 설정 확인

**테스트**

```bash
cargo metadata --no-deps
cargo check --workspace
```

**완료 조건**

- Manifest 오류 없음
- 의존성 순환 없음
- 전체 Workspace 컴파일 성공

## Task 002 — Core Types

**구현 작업**

1. `Game` trait
2. `Context`
3. `LifecycleState`
4. `Time`
5. `EngineError`
6. `GameError`

**테스트**

- LifecycleState 상태 전이 검증
- 시간 값의 기본 초기화 검증
- 오류 타입 변환 검증

**완료 조건**

- rpe-core 독립 빌드 성공
- 플랫폼 관련 의존성 없음

## Task 003 — Platform Layer

**구현 작업**

1. winit 이벤트 루프
2. 플랫폼 애플리케이션
3. 창 설정
4. 창 생성
5. 창 닫기 이벤트

**테스트**

- Windows에서 창 생성
- 창 제목 검증
- 창 크기 검증
- 닫기 버튼을 통한 정상 종료

**완료 조건**

- 창이 생성되고 이벤트 루프가 정상 종료됨

## Task 004 — Engine Builder

**구현 작업**

1. Engine 구조체
2. EngineBuilder
3. 설정 검증
4. Platform 연결
5. Prelude

**테스트**

- Builder 기본값 검증
- 사용자 설정 반영
- 0 크기 해상도 거부
- 초기화 오류 전파

**완료 조건**

- 사용자 코드에서 Engine::builder() 사용 가능

## Task 005 — Runtime Lifecycle

**구현 작업**

1. Game::init 호출
2. Game::update 호출
3. Game::shutdown 호출
4. Context 생성
5. 종료 요청 처리

**테스트**

- init 호출 횟수 1회
- update 호출 확인
- 정상 종료 시 shutdown 1회
- init 실패 시 update 미호출
- init 실패 시 shutdown 미호출

**완료 조건**

- 게임 생명주기가 규약에 맞게 동작함

## Task 006 — Time System

**구현 작업**

1. Instant 기반 시간 측정
2. Delta Time
3. Elapsed Time
4. Frame Count

**테스트**

- Delta Time 비음수
- 누적 시간 단조 증가
- 프레임 카운트 갱신

**완료 조건**

- 게임 업데이트에 시간 정보가 정확하게 전달됨

## Task 007 — Input System

**구현 작업**

1. Key 타입
2. 키보드 입력 이벤트 변환
3. InputState
4. 눌림 및 해제 상태 추적
5. 포커스 상실 처리

**테스트**

- Press 이벤트
- Release 이벤트
- 상태 전이 검증
- 포커스 상실 시 키 상태 초기화

**완료 조건**

- 게임 업데이트에서 입력 상태를 확인할 수 있음

## Task 008 — Integration Testing

**구현 작업**

1. hello-window 예제
2. lifecycle-test 예제
3. 키보드 입력 예제
4. 전체 Workspace 테스트
5. Clippy 및 Formatting 검증

**테스트**

```bash
cargo check --workspace --all-targets

cargo test --workspace

cargo clippy --workspace --all-targets

cargo fmt --all -- --check
```

**완료 조건**

- 모든 자동 테스트 통과
- 창 생성 및 종료 수동 테스트 통과
- 주요 API 문서화 완료

---

# 13. 테스트 케이스

| ID | 테스트 내용 | 기대 결과 |
|---|---|---|
| M001-T01 | Cargo Workspace 검사 | 성공 |
| M001-T02 | 기본 Engine Builder | 정상 설정 생성 |
| M001-T03 | 논리 해상도 0 지정 | 오류 반환 |
| M001-T04 | 창 생성 | 정상 표시 |
| M001-T05 | 창 닫기 | 정상 종료 |
| M001-T06 | Game init | 정확히 1회 |
| M001-T07 | Game update | Running 상태에서 호출 |
| M001-T08 | Game shutdown | 정상 종료 시 정확히 1회 |
| M001-T09 | 초기화 실패 | 오류 반환 및 update 미호출 |
| M001-T10 | Delta Time | 음수 아님 |
| M001-T11 | Frame Count | 정상 증가 |
| M001-T12 | 키 눌림 | pressed 및 just_pressed 반영 |
| M001-T13 | 키 해제 | just_released 반영 |
| M001-T14 | 포커스 상실 | 눌림 상태 정리 |
| M001-T15 | Context 종료 요청 | 이벤트 루프 종료 |
| M001-T16 | 전체 Workspace 테스트 | 통과 |

---

# 14. 완료 판정

다음 조건을 모두 만족할 때 Milestone 001을 완료한 것으로 판정한다.

- [ ] 모든 대상 크레이트 컴파일 성공
- [ ] 전체 Workspace 검사 성공
- [ ] EngineBuilder 구현 완료
- [ ] 운영체제 창 생성 성공
- [ ] 게임 초기화 콜백 정상 작동
- [ ] 게임 업데이트 콜백 정상 작동
- [ ] 게임 종료 콜백 정상 작동
- [ ] Delta Time 정상 계산
- [ ] 키보드 입력 상태 관리 정상 작동
- [ ] 사용자 종료 요청 정상 처리
- [ ] 초기화 오류 정상 반환
- [ ] 통합 예제 실행 성공
- [ ] Clippy 및 Formatter 검사 통과
- [ ] 테스트 케이스 전체 통과

GUI 창이 필요한 테스트는 수동 또는 GUI가 제공되는 CI 환경에서 수행한다. 헤드리스 CI에서는 플랫폼 독립적인 단위 테스트를 우선 실행한다.

---

# 15. 후속 마일스톤 연결

Milestone 001 완료 후 진행할 다음 개발 단계는 다음과 같다.

**Milestone 002 — CPU Pixel Renderer**

예정 기능:

- Pixel Framebuffer
- Color
- Clear Command
- Pixel Command
- Rectangle Rasterization
- Alpha Blending
- 화면 출력
- Integer Scaling

Milestone 001에서 구현한 이벤트 루프와 게임 생명주기는 변경을 최소화하며, 렌더링 콜백과 CPU 렌더러를 추가하는 방식으로 확장한다.

---

# 16. 최종 목표

Milestone 001의 완료 상태는 다음과 같이 정의한다.

> RustPixelEngine을 이용해 Rust 코드만으로 운영체제 창을 만들고, 게임을 초기화하며, 업데이트 루프를 실행하고, 입력을 처리한 뒤, 안전하게 종료할 수 있다.

**이 마일스톤은 게임을 그리는 단계가 아니라 게임이 살아 움직이기 위한 첫 번째 기반을 만드는 단계이다.**

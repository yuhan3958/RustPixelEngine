# API 계약

현재 코드에 존재하는 계약과 후속 마일스톤의 설계안을 구분한다. 계획 API는 아직 사용 가능한 API가 아니다.

## 현재 공개 진입점

`rust_pixel_engine::prelude::*`는 `Engine`, `EngineBuilder`, `Context`, `Game`, `GameError`, `EngineError`, `Time`, `InputState`, `Key`를 재노출한다. 최상위 크레이트는 생명주기 관련 타입도 재노출한다.

### EngineBuilder

`Engine::builder()` 또는 `EngineBuilder::new()`에서 시작해 `title`, `resolution`, `window_size`를 연쇄 호출하고 `run::<G>()`으로 실행한다. 제목 기본값은 RustPixelEngine, 논리 해상도는 320×180, 창 크기는 1280×720이다. 모든 크기 차원은 양수여야 하며 잘못된 값은 이벤트 루프 생성 전에 `InvalidConfiguration`을 반환한다. 창 크기 설정은 winit 논리 크기다.

### Game

```rust
pub trait Game: Sized + 'static {
    fn init(ctx: &mut Context) -> Result<Self, GameError>;
    fn update(&mut self, _ctx: &mut Context, _dt: f32) {}
    fn shutdown(&mut self, _ctx: &mut Context) {}
}
```

`init` 성공 후에만 `update`와 `shutdown`이 가능하다. 정상 종료 경로에서 초기화된 게임의 `shutdown`은 한 번 호출된다. 초기화 실패, 강제 종료와 패닉 시 호출을 보장하지 않는다. `dt`는 초 단위이며 고정 간격이 아니다. 콜백은 Context의 독점 가변 참조를 임시로 받는다.

### Context

| 메서드 | 의미 |
|---|---|
| `time()`, `input()` | 시간과 입력 상태의 불변 참조 |
| `logical_size()` | 설정된 논리 해상도 `(u32, u32)` |
| `window_size()` | 마지막으로 보고된 물리 창 크기 |
| `is_focused()` | 마지막으로 보고된 키보드 포커스 상태 |
| `request_exit()`, `is_exit_requested()` | 종료 요청과 상태 조회 |

초기 창 크기는 보고 전까지 `(0, 0)`, 포커스는 false다. `input_mut`, 시간 갱신과 창 상태 설정 메서드도 현재 공개되어 있으나 런타임 이벤트 처리에 사용된다.

### 입력

`pressed`는 현재 상태, `just_pressed`와 `just_released`는 업데이트 경계까지 유지되는 전이다. 이미 눌린 키의 반복 press는 새 전이를 만들지 않는다. 같은 구간의 press와 release는 두 전이를 모두 유지한다. 포커스 상실과 suspend는 누른 키를 해제한다.

### 시간과 오류

`Time`은 단조 `Instant`를 사용한다. delta는 `f32` 초, 경과 시간은 생성 시점부터의 `f64` 초, frame count는 갱신 횟수다. 재개 시 delta 기준을 재설정하지만 경과 시간은 계속 흐른다.

`EngineError`는 `InvalidConfiguration`, `Platform`, `PlatformSource`, `GameInitialization`을 포함한다. 마지막 두 원인 보유 variant는 `Error::source`를 제공한다. `GameError::new`로 게임 초기화 오류를 생성한다.

## 계획된 확장

- **002:** 기본 빈 `Game::render`와 공통 프레임·명령·backend 계약. 명령은 제출 순서대로 처리한다.
- **003:** 기존 입력과 시간을 확장하고 기본 빈 `Game::fixed_update`를 추가한다. 고정 갱신과 가변 갱신은 각각 입력 전이 스냅샷을 갖는다.
- **004:** 이미지 저장소와 세대 핸들, 스프라이트 명령. 제거된 핸들은 오류다.
- **005–006:** 동일한 명령을 사용하는 GPU 및 Hybrid backend와 명시적 선택·fallback.
- **007:** 선택적 월드·장면·애니메이션·타일맵·물리·오디오·UI.

구체적인 계획 시그니처와 검증 조건은 해당 [마일스톤](roadmap.md)에 있으며 구현 과정에서 확정한다. 현재 `Game`에 render나 fixed_update는 존재하지 않는다.

## 변경과 오류 정책

0.x 단계에서는 공개 계약이 바뀔 수 있다. 변경 시 호출 순서, 소유권, 오류와 마이그레이션을 문서화한다. 플랫폼 또는 장치에 종속된 오류는 원인을 보존하고 지원되지 않는 기능을 조용히 성공 처리하지 않는다. 자동 fallback은 명시적으로 선택한 정책에서만 허용한다.

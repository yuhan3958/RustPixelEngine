# API 계약

현재 코드에 존재하는 계약과 후속 마일스톤의 설계안을 구분한다. 계획 API는 아직 사용 가능한 API가 아니다.

## 현재 공개 진입점

`rust_pixel_engine::prelude::*`는 엔진, 생명주기 콜백, 렌더 명령, `TimingConfig`, 입력 상태, 키·마우스 버튼 및 `InputEvent`를 재노출한다. 최상위 크레이트는 생명주기 관련 타입도 재노출한다.

### EngineBuilder

`Engine::builder()` 또는 `EngineBuilder::new()`에서 시작해 `title`, `resolution`, `window_size`, `timing`을 연쇄 호출하고 `run::<G>()`으로 실행한다. 제목 기본값은 RustPixelEngine, 논리 해상도는 320×180, 창 크기는 1280×720, 타이밍은 60Hz와 최대 5스텝이다. 크기 차원과 타이밍 값은 양수여야 하며 잘못된 값은 이벤트 루프 생성 전에 `InvalidConfiguration`을 반환한다. 창 크기 설정은 winit 논리 크기다.

### Game

```rust
pub trait Game: Sized + 'static {
    fn init(ctx: &mut Context) -> Result<Self, GameError>;
    fn fixed_update(&mut self, _ctx: &mut Context, _dt: f32) {}
    fn update(&mut self, _ctx: &mut Context, _dt: f32) {}
    fn render(&mut self, _ctx: &mut Context, _frame: &mut RenderFrame) {}
    fn shutdown(&mut self, _ctx: &mut Context) {}
}
```

`init` 성공 후에만 갱신과 `shutdown`이 가능하다. 정상 종료 경로에서 초기화된 게임의 `shutdown`은 한 번 호출된다. 초기화 실패, 강제 종료와 패닉 시 호출을 보장하지 않는다. `fixed_update`의 `dt`는 설정된 고정 간격이고 `update`의 `dt`는 실제 경과 초다. 각 표시 갱신은 고정 갱신 0회 이상, `update` 1회, `render` 1회 순서다. 콜백은 Context의 독점 가변 참조를 임시로 받는다.

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

`events()`는 키·마우스 버튼 전이와 커서 이동을 수집 순서로 반환한다. 고정 및 가변 갱신은 독립된 목록을 사용한다. 고정 갱신이 없으면 해당 목록은 유지되고, 여러 고정 스텝 중 첫 스텝에서만 누적 전이를 제공한다. 현재 상태는 최신 상태이며 이벤트별 커서 물리·논리 좌표는 이벤트에 보존된다. `bind_action`은 키와 마우스 버튼을 이름에 연결하고 `action_pressed`는 연결된 버튼들의 OR 상태를 반환한다.

### 시간과 오류

`Time`은 단조 `Instant`를 사용한다. delta는 `f32` 초, 경과 시간은 생성 시점부터의 `f64` 초, frame count는 갱신 횟수다. 재개 시 delta 기준을 재설정하지만 경과 시간은 계속 흐른다.

`EngineError`는 `InvalidConfiguration`, `Platform`, `PlatformSource`, `GameInitialization`을 포함한다. 마지막 두 원인 보유 variant는 `Error::source`를 제공한다. `GameError::new`로 게임 초기화 오류를 생성한다.

## 계획된 확장

- **004:** 이미지 저장소와 세대 핸들, 스프라이트 명령. 제거된 핸들은 오류다.
- **005–006:** 동일한 명령을 사용하는 GPU 및 Hybrid backend와 명시적 선택·fallback.
- **007:** 선택적 월드·장면·애니메이션·타일맵·물리·오디오·UI.

구체적인 계획 시그니처와 검증 조건은 해당 [마일스톤](roadmap.md)에 있으며 구현 과정에서 확정한다.

## 변경과 오류 정책

0.x 단계에서는 공개 계약이 바뀔 수 있다. 변경 시 호출 순서, 소유권, 오류와 마이그레이션을 문서화한다. 플랫폼 또는 장치에 종속된 오류는 원인을 보존하고 지원되지 않는 기능을 조용히 성공 처리하지 않는다. 자동 fallback은 명시적으로 선택한 정책에서만 허용한다.

# Milestone 003 — Input & Timing

## 1. Metadata

- Project: RustPixelEngine
- Milestone Number: 003
- Title: Input & Timing
- Target Version: 0.1.0 개발 단계 (릴리스 버전은 008에서 확정)
- Status: Planned
- Dependencies: 002
- Target Crates: rpe-input, rpe-core, rpe-platform, rpe

## 2. Overview

001의 키 상태와 Instant 기반 시간을 확장하여 가변 렌더와 고정 시뮬레이션을 분리한다.

## 3. Objectives

명시된 범위를 구현하고 오류 및 경계 동작을 검증해 다음 마일스톤에 안정된 인터페이스를 제공한다.

## 4. Scope

마우스, 액션 매핑, 입력 버퍼, fixed update, 갱신률과 진단을 추가한다. 게임패드와 리바인딩 UI는 제외한다.

## 5. Architecture

상위 rpe가 플랫폼, 공통 타입과 선택 구현체를 조립한다. 하위 크레이트는 rpe에 의존하지 않는다. 세부 의존 방향과 색상·좌표 규약은 [로드맵](../roadmap.md)을 따른다.

## 6. Module Responsibilities

대상 크레이트: rpe-input, rpe-core, rpe-platform, rpe. 각 크레이트는 자신의 데이터 및 동작을 소유하고 rpe가 생명주기와 사용자 API를 연결한다. 외부 리소스는 생성한 모듈이 해제한다.

## 7. Public API Design

다음은 아직 구현되지 않은 설계안이다. 필요한 타입은 같은 마일스톤 및 선행 문서에 정의한다.

```rust
pub struct TimingConfig { pub fixed_hz: u32, pub max_steps: u32 }
pub trait Game: Sized + 'static {
    fn init(ctx: &mut Context) -> Result<Self, GameError>;
    fn update(&mut self, _ctx: &mut Context, _dt: f32) {}
    fn fixed_update(&mut self, _ctx: &mut Context, _dt: f32) {}
    fn render(&mut self, _ctx: &mut Context, _frame: &mut RenderFrame) {}
    fn shutdown(&mut self, _ctx: &mut Context) {}
}
```

## 8. Internal Design

실제 통합은 별도 trait 강제가 아니라 Game에 기본 빈 fixed_update를 추가한다. TimingConfig는 0을 거부하고 기본 60Hz, max_steps 5다. 단조 시간 차이를 accumulator에 더하고 step 이상이면 최대 max_steps회 fixed_update를 호출한다. 초과 누적은 step 미만의 나머지만 보존하고 버린 시간을 진단한다. update는 표시 주기마다 한 번, render는 update 후 한 번이다. 입력 이벤트는 수집 순서를 보존하며 첫 fixed_update에 전이를 제공하고 소비 후 제거한다. fixed step이 없는 프레임에서도 전이는 유지한다. 가변 update에는 별도 프레임 스냅샷을 제공해 fixed 소비와 독립시킨다. pressed는 현재 상태다. 포커스 상실 시 키와 버튼을 해제하고 해제 이벤트를 버퍼에 기록한다. 마우스 물리 좌표는 002 viewport를 역변환하며 여백이나 배율 0에서는 논리 좌표 None이다. 액션은 사용자 정의 이름과 키/버튼 집합의 OR로 계산한다. 시간 초기화와 suspend 재개 기준을 001과 일치시킨다.

공개 오류는 원인을 보존하고 복구 가능한 입력 오류를 panic으로 처리하지 않는다. 이벤트 루프와 창은 플랫폼 스레드에서 소유하며 불필요한 Send/Sync 제약을 공개 API에 강제하지 않는다.

## 9. Implementation Tasks

### M003-T01 — 스케줄러

- 목적: 스케줄러 제공.
- 사전 조건: Milestone 002 완료.
- 수정 대상 모듈: core, rpe.
- 구현 사항: 설정 검증과 accumulator, 제한 및 진단을 구현.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 가짜 시계로 0·다중·초과 step 검증.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M003-T02 — 입력 버퍼

- 목적: 입력 버퍼 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: input, platform.
- 구현 사항: 기존 키 전이를 보존하며 고정/가변 스냅샷을 분리.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: step 없는 프레임과 다중 step 검증.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M003-T03 — 마우스와 액션

- 목적: 마우스와 액션 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: input, platform, rpe.
- 구현 사항: 버튼, 위치, viewport 역변환과 OR 매핑.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 여백·포커스 상실·복수 바인딩 검증.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

## 10. Test Specification

잘못된 Hz와 max_steps 거부, suspend 후 긴 delta 방지, 초과 시간 진단, 같은 구간 press/release, 0회 및 다중 fixed step, 여백 좌표를 검증한다.

## 11. Performance Considerations

이벤트 수와 스냅샷 복사, step 수 및 프레임 간격 분포를 측정한다. 측정 환경과 표본을 기록하며 측정 전 성능 수치를 보장하지 않는다.

## 12. Risks

고정과 가변 소비자가 같은 전이를 중복 처리할 수 있으므로 각각의 의미를 API 문서에 명시한다.

## 13. Acceptance Criteria

- [ ] 범위 내 기능과 오류 처리가 구현되고 공개 API가 문서화됨.
- [ ] 각 Task의 테스트와 이 문서의 경계 사례가 통과함.
- [ ] 예제와 지원 feature 조합이 대상 환경에서 검증됨.
- [ ] 미지원 환경과 수동 검증 결과가 기록됨.

## 14. Deliverables

대상 크레이트 코드, 단위·통합 테스트, 실행 예제, API 및 사용 문서와 검증 결과.

## 15. Transition

004 이후 애니메이션과 게임 로직은 fixed_update의 시간을 사용할 수 있다.


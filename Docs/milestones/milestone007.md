# Milestone 007 — Game Framework

## 1. Metadata

- Project: RustPixelEngine
- Milestone Number: 007
- Title: Game Framework
- Target Version: 0.1.0 개발 단계 (릴리스 버전은 008에서 확정)
- Status: Planned
- Dependencies: 006
- Target Crates: rpe-world, rpe-animation, rpe-tilemap, rpe-physics, rpe-audio, rpe-ui

## 2. Overview

코드 기반 객체 모델로 작은 2D 게임을 구성한다. ECS는 필수가 아니다.

## 3. Objectives

명시된 범위를 구현하고 오류 및 경계 동작을 검증해 다음 마일스톤에 안정된 인터페이스를 제공한다.

## 4. Scope

world/entity/scene, camera, animation, tilemap, AABB 충돌, 선택 물리, 오디오 및 기본 UI. 에디터와 완전한 물리 엔진은 제외한다.

## 5. Architecture

상위 rpe가 플랫폼, 공통 타입과 선택 구현체를 조립한다. 하위 크레이트는 rpe에 의존하지 않는다. 세부 의존 방향과 색상·좌표 규약은 [로드맵](../roadmap.md)을 따른다.

## 6. Module Responsibilities

대상 크레이트: rpe-world, rpe-animation, rpe-tilemap, rpe-physics, rpe-audio, rpe-ui. 각 크레이트는 자신의 데이터 및 동작을 소유하고 rpe가 생명주기와 사용자 API를 연결한다. 외부 리소스는 생성한 모듈이 해제한다.

## 7. Public API Design

다음은 아직 구현되지 않은 설계안이다. 필요한 타입은 같은 마일스톤 및 선행 문서에 정의한다.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId { pub index: u32, pub generation: u32 }
pub trait Scene {
    fn update(&mut self, dt: f32);
    fn render(&mut self, frame: &mut RenderFrame);
}
```

## 8. Internal Design

World는 세대 ID 슬롯과 사용자 객체를 소유하고 제거한 ID를 거부한다. Scene 전환은 콜백 중 큐에 넣고 업데이트 경계에서 현재 장면 종료 후 새 장면 초기화 순서로 처리한다. 실패 시 기존 장면을 유지한다. 카메라는 정수 논리 좌표 오프셋으로 월드 좌표를 변환한다. 애니메이션은 양수 duration 프레임 목록과 누적 시간으로 큰 dt에서도 여러 프레임을 진행하고 loop/once를 명시한다. 타일은 row-major 인덱스, 레이어 제출 순서, 빈 셀과 검증된 sprite source를 사용한다. 충돌은 반열린 AABB 겹침을 기본 제공하며 역학은 별도 feature다. 오디오 장치 생성 실패를 반환하고 장치 없는 환경을 지원한다. UI는 논리 좌표 hit-test, 포커스와 클릭 전이를 사용한다. 장면이 핸들을 보유하며 assets 소유권과 만료 정책은 004를 따른다.

공개 오류는 원인을 보존하고 복구 가능한 입력 오류를 panic으로 처리하지 않는다. 이벤트 루프와 창은 플랫폼 스레드에서 소유하며 불필요한 Send/Sync 제약을 공개 API에 강제하지 않는다.

## 9. Implementation Tasks

### M007-T01 — 월드와 장면

- 목적: 월드와 장면 제공.
- 사전 조건: Milestone 006 완료.
- 수정 대상 모듈: world.
- 구현 사항: 세대 ID, 객체 수명 및 지연 전환.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: stale ID와 전환 실패 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M007-T02 — 카메라와 애니메이션

- 목적: 카메라와 애니메이션 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: world, animation.
- 구현 사항: 정수 변환과 시간 기반 프레임 진행.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 음수 좌표, 큰 dt 및 종료 검증.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M007-T03 — 타일맵

- 목적: 타일맵 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: tilemap.
- 구현 사항: 검증된 레이어 저장과 표시.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 경계 및 레이어 순서 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M007-T04 — 충돌과 선택 물리

- 목적: 충돌과 선택 물리 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: physics.
- 구현 사항: AABB와 선택 dynamics 경계.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 맞닿음, 중첩 및 feature 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M007-T05 — 오디오

- 목적: 오디오 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: audio.
- 구현 사항: 장치 및 음원 수명, 오류 처리.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 장치 없음과 정리 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M007-T06 — UI와 게임 예제

- 목적: UI와 게임 예제 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: ui, rpe.
- 구현 사항: 입력 hit-test와 게임 구성.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 포커스·클릭 및 장면 종료 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

## 10. Test Specification

세대 재사용, 장면 전환 실패, 큰 delta, 빈/경계 타일, 반열린 충돌, 장치 부재와 UI 포커스를 검증한다. GUI/오디오 수동 검증 환경을 기록한다.

## 11. Performance Considerations

객체 수, 타일 가시 영역, 충돌 후보와 오디오 메모리별 비용을 측정한다. 측정 환경과 표본을 기록하며 측정 전 성능 수치를 보장하지 않는다.

## 12. Risks

오디오 라이브러리와 선택적 물리 범위가 미확정이다. 라이선스와 플랫폼 지원을 선정 전에 검토한다.

## 13. Acceptance Criteria

- [ ] 범위 내 기능과 오류 처리가 구현되고 공개 API가 문서화됨.
- [ ] 각 Task의 테스트와 이 문서의 경계 사례가 통과함.
- [ ] 예제와 지원 feature 조합이 대상 환경에서 검증됨.
- [ ] 미지원 환경과 수동 검증 결과가 기록됨.

## 14. Deliverables

대상 크레이트 코드, 단위·통합 테스트, 실행 예제, API 및 사용 문서와 검증 결과.

## 15. Transition

008은 각 선택 feature와 전체 예제의 호환성을 검증한다.


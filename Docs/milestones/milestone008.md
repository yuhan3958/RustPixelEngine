# Milestone 008 — Stabilization & Release

## 1. Metadata

- Project: RustPixelEngine
- Milestone Number: 008
- Title: Stabilization & Release
- Target Version: 0.1.0 개발 단계 (릴리스 버전은 008에서 확정)
- Status: Planned
- Dependencies: 001–007
- Target Crates: 전체 Workspace

## 2. Overview

기능 존재를 공개 가능한 품질과 지원 범위로 검증한다.

## 3. Objectives

명시된 범위를 구현하고 오류 및 경계 동작을 검증해 다음 마일스톤에 안정된 인터페이스를 제공한다.

## 4. Scope

API, 문서, 예제, 회귀, benchmark, 플랫폼, feature, 의존성, 패키징 및 릴리스. 새 기능 확장은 제외한다.

## 5. Architecture

상위 rpe가 플랫폼, 공통 타입과 선택 구현체를 조립한다. 하위 크레이트는 rpe에 의존하지 않는다. 세부 의존 방향과 색상·좌표 규약은 [로드맵](../roadmap.md)을 따른다.

## 6. Module Responsibilities

대상 크레이트: 전체 Workspace. 각 크레이트는 자신의 데이터 및 동작을 소유하고 rpe가 생명주기와 사용자 API를 연결한다. 외부 리소스는 생성한 모듈이 해제한다.

## 7. Public API Design

다음은 아직 구현되지 않은 설계안이다. 필요한 타입은 같은 마일스톤 및 선행 문서에 정의한다.

```rust
pub struct ReleaseSupport {
    pub windows_verified: bool,
    pub linux_verified: bool,
    pub macos_verified: bool,
}
```

## 8. Internal Design

위 타입은 지원 상태를 설명하는 문서 예시이며 제품 API로 추가하지 않는다. Windows에서 실제 실행과 표준 검사를 수행하고 Linux/macOS는 검증 결과를 별도 기록한다. CPU 단독, GPU, hybrid, 선택 시스템과 full 및 no-default 조합을 매트릭스로 정의한다. platform의 window off 조건부 컴파일과 assets 기본 의존성, Workspace 중복 선언을 해결한다. GUI, GPU, 오디오가 없는 CI에서 skip은 통과가 아니라 미검증으로 기록한다. 현재 publish=false와 실제 LICENSE 파일 유무를 검토해 배포 결정에 맞게 명시적으로 수정한다. 0.x 호환성 정책, MSRV, 변경 이력, 보안 및 의존 라이선스 검토를 릴리스 문서에 기록한다.

공개 오류는 원인을 보존하고 복구 가능한 입력 오류를 panic으로 처리하지 않는다. 이벤트 루프와 창은 플랫폼 스레드에서 소유하며 불필요한 Send/Sync 제약을 공개 API에 강제하지 않는다.

## 9. Implementation Tasks

### M008-T01 — API와 문서

- 목적: API와 문서 제공.
- 사전 조건: Milestone 001–007 완료.
- 수정 대상 모듈: 전체.
- 구현 사항: 공개 API, 오류, 예제, 호환 정책과 마이그레이션 검토.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 문서 예제 및 공개 항목 대조.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M008-T02 — 회귀와 성능

- 목적: 회귀와 성능 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: 전체.
- 구현 사항: 단위·통합·이미지 회귀 및 재현 가능한 benchmark.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 참조 이미지와 결과 기록.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M008-T03 — 플랫폼과 feature

- 목적: 플랫폼과 feature 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: 전체.
- 구현 사항: Windows 우선 매트릭스와 나머지 플랫폼 상태 기록.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 각 조합 빌드 및 실제 실행 기록.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M008-T04 — 공급망과 배포

- 목적: 공급망과 배포 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: Cargo 및 릴리스 문서.
- 구현 사항: 라이선스, 의존성, 패키징, 버전과 배포물 검토.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 패키지 내용 및 릴리스 후보 확인.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

## 10. Test Specification

공개 예제 컴파일, 지원 feature 조합, 오류 경로, 렌더 기준 이미지, Windows 창/입력/출력/종료와 사용 가능한 Linux/macOS 환경을 확인한다. 미실행 항목은 미검증으로 남긴다.

## 11. Performance Considerations

벤치 장비, OS, 빌드 설정, 장면, 표본 수와 분포를 기록하고 기준선과 비교한다. 측정 환경과 표본을 기록하며 측정 전 성능 수치를 보장하지 않는다.

## 12. Risks

지원하지 못한 OS와 GPU 드라이버를 지원으로 오인하지 않도록 릴리스 노트에 분리한다.

## 13. Acceptance Criteria

- [ ] 범위 내 기능과 오류 처리가 구현되고 공개 API가 문서화됨.
- [ ] 각 Task의 테스트와 이 문서의 경계 사례가 통과함.
- [ ] 예제와 지원 feature 조합이 대상 환경에서 검증됨.
- [ ] 미지원 환경과 수동 검증 결과가 기록됨.

## 14. Deliverables

대상 크레이트 코드, 단위·통합 테스트, 실행 예제, API 및 사용 문서와 검증 결과.

## 15. Transition

승인된 배포 방식과 검증된 지원 매트릭스를 바탕으로 초기 공개 릴리스를 진행한다.


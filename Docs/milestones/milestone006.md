# Milestone 006 — Hybrid Renderer

## 1. Metadata

- Project: RustPixelEngine
- Milestone Number: 006
- Title: Hybrid Renderer
- Target Version: 0.1.0 개발 단계 (릴리스 버전은 008에서 확정)
- Status: Planned
- Dependencies: 005
- Target Crates: rpe-render-hybrid, rpe-render-cpu, rpe-render-gpu, rpe-render, rpe

## 2. Overview

CPU 결과를 GPU 텍스처로 업로드하고 정수 배율로 표시한다.

## 3. Objectives

명시된 범위를 구현하고 오류 및 경계 동작을 검증해 다음 마일스톤에 안정된 인터페이스를 제공한다.

## 4. Scope

CPU 프레임버퍼, 업로드, compositing, 백엔드 선택과 명시적 fallback. GPU 명령과 CPU 명령의 프레임 내 혼합은 제외한다.

## 5. Architecture

상위 rpe가 플랫폼, 공통 타입과 선택 구현체를 조립한다. 하위 크레이트는 rpe에 의존하지 않는다. 세부 의존 방향과 색상·좌표 규약은 [로드맵](../roadmap.md)을 따른다.

## 6. Module Responsibilities

대상 크레이트: rpe-render-hybrid, rpe-render-cpu, rpe-render-gpu, rpe-render, rpe. 각 크레이트는 자신의 데이터 및 동작을 소유하고 rpe가 생명주기와 사용자 API를 연결한다. 외부 리소스는 생성한 모듈이 해제한다.

## 7. Public API Design

다음은 아직 구현되지 않은 설계안이다. 필요한 타입은 같은 마일스톤 및 선행 문서에 정의한다.

```rust
#[derive(Clone, Copy, Debug)]
pub enum BackendKind { Cpu, Gpu, Hybrid }
#[derive(Clone, Copy, Debug)]
pub enum FallbackPolicy { Disabled, ToCpu }
```

## 8. Internal Design

Hybrid는 002 CPU 래스터라이저를 호출한 뒤 RGBA8 결과를 GPU 텍스처에 업로드하고 nearest로 표시한다. 행 pitch와 정렬은 wgpu 업로드 경로의 실제 제약을 확인하고 checked arithmetic으로 계산한다. 텍스처 크기는 논리 해상도가 변할 때만 재생성한다. 선택한 백엔드가 feature에서 빠졌거나 capability가 부족하면 오류를 반환한다. ToCpu는 초기화 실패 시에만 시도하고 원래 실패와 fallback 결과를 보존·로그한다. 런타임 중 자동 전환은 지원하지 않는다. hybrid feature는 cpu와 gpu를 활성화한다. GPU resource와 창 수명은 005 규약을 따른다.

공개 오류는 원인을 보존하고 복구 가능한 입력 오류를 panic으로 처리하지 않는다. 이벤트 루프와 창은 플랫폼 스레드에서 소유하며 불필요한 Send/Sync 제약을 공개 API에 강제하지 않는다.

## 9. Implementation Tasks

### M006-T01 — 업로드와 표시

- 목적: 업로드와 표시 제공.
- 사전 조건: Milestone 005 완료.
- 수정 대상 모듈: render-hybrid, render-gpu.
- 구현 사항: CPU 결과 업로드, nearest 출력 및 재생성.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 비정렬 너비와 resize 이미지 비교.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M006-T02 — 선택과 fallback

- 목적: 선택과 fallback 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: rpe.
- 구현 사항: feature와 capability 검증, 초기화 전용 fallback.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 미컴파일 백엔드·실패·fallback 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M006-T03 — 측정 예제

- 목적: 측정 예제 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: rpe.
- 구현 사항: 동일 장면에서 세 백엔드를 선택하는 예제.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 논리 픽셀과 초기화 결과 비교.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

## 10. Test Specification

CPU와 hybrid 논리 픽셀 일치, 크기 0, 장치 실패, fallback 비활성/활성, feature 미포함을 검사한다.

## 11. Performance Considerations

동일 해상도와 장면에서 래스터, 업로드, 표시 및 전체 프레임 시간을 별도 측정한다. 측정 환경과 표본을 기록하며 측정 전 성능 수치를 보장하지 않는다.

## 12. Risks

전체 프레임 업로드 대역폭과 GPU 의존성이 부담이다. 부분 업로드는 측정 후 별도 최적화한다.

## 13. Acceptance Criteria

- [ ] 범위 내 기능과 오류 처리가 구현되고 공개 API가 문서화됨.
- [ ] 각 Task의 테스트와 이 문서의 경계 사례가 통과함.
- [ ] 예제와 지원 feature 조합이 대상 환경에서 검증됨.
- [ ] 미지원 환경과 수동 검증 결과가 기록됨.

## 14. Deliverables

대상 크레이트 코드, 단위·통합 테스트, 실행 예제, API 및 사용 문서와 검증 결과.

## 15. Transition

007은 백엔드 선택과 무관하게 동일한 RenderFrame을 제출한다.


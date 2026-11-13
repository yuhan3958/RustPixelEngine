# Milestone 005 — GPU Renderer

## 1. Metadata

- Project: RustPixelEngine
- Milestone Number: 005
- Title: GPU Renderer
- Target Version: 0.1.0 개발 단계 (릴리스 버전은 008에서 확정)
- Status: Planned
- Dependencies: 004
- Target Crates: rpe-render-gpu, rpe-render, rpe-assets, rpe

## 2. Overview

동일 프레임 명령을 wgpu로 실행하는 선택적 GPU 백엔드다.

## 3. Objectives

명시된 범위를 구현하고 오류 및 경계 동작을 검증해 다음 마일스톤에 안정된 인터페이스를 제공한다.

## 4. Scope

adapter/device/queue, surface, pipeline, texture upload, 순서 보존 batching, nearest, 리사이즈 및 capability. 셰이더 확장 API는 제외한다.

## 5. Architecture

상위 rpe가 플랫폼, 공통 타입과 선택 구현체를 조립한다. 하위 크레이트는 rpe에 의존하지 않는다. 세부 의존 방향과 색상·좌표 규약은 [로드맵](../roadmap.md)을 따른다.

## 6. Module Responsibilities

대상 크레이트: rpe-render-gpu, rpe-render, rpe-assets, rpe. 각 크레이트는 자신의 데이터 및 동작을 소유하고 rpe가 생명주기와 사용자 API를 연결한다. 외부 리소스는 생성한 모듈이 해제한다.

## 7. Public API Design

다음은 아직 구현되지 않은 설계안이다. 필요한 타입은 같은 마일스톤 및 선행 문서에 정의한다.

```rust
#[derive(Clone, Copy, Debug)]
pub struct BackendCapabilities { pub max_texture_dimension: u32, pub supports_sprites: bool }
```

## 8. Internal Design

rpe가 창을 소유하고 GPU backend는 안전하게 공유 소유한 창으로 surface 수명을 유지한다. wgpu 30 실제 API와 대상 플랫폼 지원은 구현 시 공식 문서로 확인한다. 002 합성은 straight alpha 정수 sRGB 규약이므로 기본 하드웨어 sRGB blending만으로 동일하다고 가정하지 않는다. 논리 해상도 RGBA8 정수 텍스처에 순서대로 실행하고 명시적 정수 연산 또는 검증된 동등한 경로로 합성한다. 표시는 nearest와 002 viewport를 사용한다. sprite texture는 핸들 generation별 캐시하고 삭제 시 무효화한다. 겹침 순서를 바꾸는 정렬은 금지하고 연속 호환 명령만 batch한다. 0 크기 surface는 중단; lost/outdated는 재설정 후 다음 프레임 재시도, timeout은 프레임 생략, 메모리 부족은 오류 종료한다. GPU 제한 초과와 생성 실패는 원인을 전달한다.

공개 오류는 원인을 보존하고 복구 가능한 입력 오류를 panic으로 처리하지 않는다. 이벤트 루프와 창은 플랫폼 스레드에서 소유하며 불필요한 Send/Sync 제약을 공개 API에 강제하지 않는다.

## 9. Implementation Tasks

### M005-T01 — GPU 수명과 surface

- 목적: GPU 수명과 surface 제공.
- 사전 조건: Milestone 004 완료.
- 수정 대상 모듈: render-gpu, rpe.
- 구현 사항: 초기화, capability 및 재생성.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 실패 주입과 resize 수동 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M005-T02 — 명령 파이프라인

- 목적: 명령 파이프라인 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: render-gpu.
- 구현 사항: 기본 도형과 이미지 업로드, 순서 보존 batch.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: CPU 기준 이미지와 오프스크린 비교.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M005-T03 — 복구와 통합

- 목적: 복구와 통합 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: render-gpu, rpe.
- 구현 사항: 오류 분류 및 feature 선택.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: surface 손실·0 크기·GPU 미지원 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

## 10. Test Specification

CPU 기준 장면의 논리 RGBA 결과를 완전 일치 비교하고 GPU 지원 환경이 없으면 skip을 명시한다. 중첩 투명, 경계, texture 제한과 복구를 검증한다.

## 11. Performance Considerations

업로드 바이트, draw 수, CPU 제출 시간과 GPU 완료 시간을 별도로 측정한다. 측정 환경과 표본을 기록하며 측정 전 성능 수치를 보장하지 않는다.

## 12. Risks

정수 합성과 surface 색 관리의 플랫폼 차이가 크다. 동등성을 증명할 수 없으면 기능을 완료로 표시하지 않는다.

## 13. Acceptance Criteria

- [ ] 범위 내 기능과 오류 처리가 구현되고 공개 API가 문서화됨.
- [ ] 각 Task의 테스트와 이 문서의 경계 사례가 통과함.
- [ ] 예제와 지원 feature 조합이 대상 환경에서 검증됨.
- [ ] 미지원 환경과 수동 검증 결과가 기록됨.

## 14. Deliverables

대상 크레이트 코드, 단위·통합 테스트, 실행 예제, API 및 사용 문서와 검증 결과.

## 15. Transition

006은 GPU 표시 자원과 surface 복구 코드를 재사용한다.


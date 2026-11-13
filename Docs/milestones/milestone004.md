# Milestone 004 — Sprite & Asset

## 1. Metadata

- Project: RustPixelEngine
- Milestone Number: 004
- Title: Sprite & Asset
- Target Version: 0.1.0 개발 단계 (릴리스 버전은 008에서 확정)
- Status: Planned
- Dependencies: 003
- Target Crates: rpe-assets, rpe-render, rpe-render-cpu, rpe

## 2. Overview

PNG를 디코딩해 캐시하고 공통 프레임 명령으로 스프라이트를 그린다.

## 3. Objectives

명시된 범위를 구현하고 오류 및 경계 동작을 검증해 다음 마일스톤에 안정된 인터페이스를 제공한다.

## 4. Scope

동기 PNG 로드, 세대 핸들, 이미지 캐시, 시트와 소스 사각형, 정수 이동·배율·뒤집기. 비동기 로드와 임의 각도 회전은 제외한다.

## 5. Architecture

상위 rpe가 플랫폼, 공통 타입과 선택 구현체를 조립한다. 하위 크레이트는 rpe에 의존하지 않는다. 세부 의존 방향과 색상·좌표 규약은 [로드맵](../roadmap.md)을 따른다.

## 6. Module Responsibilities

대상 크레이트: rpe-assets, rpe-render, rpe-render-cpu, rpe. 각 크레이트는 자신의 데이터 및 동작을 소유하고 rpe가 생명주기와 사용자 API를 연결한다. 외부 리소스는 생성한 모듈이 해제한다.

## 7. Public API Design

다음은 아직 구현되지 않은 설계안이다. 필요한 타입은 같은 마일스톤 및 선행 문서에 정의한다.

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ImageHandle { pub index: u32, pub generation: u32 }
pub struct Sprite { pub image: ImageHandle, pub source: Rect, pub x: i32, pub y: i32, pub scale: u32, pub flip_x: bool, pub flip_y: bool }
```

## 8. Internal Design

assets가 정규화한 RGBA8 straight-alpha 이미지를 소유한다. PNG 크기와 바이트 길이는 checked arithmetic과 설정 가능한 디코딩 한도로 검증한다. 경로 캐시는 정규화 경로를 키로 사용하며 로드 실패는 캐시하지 않는다. 슬롯 재사용 시 generation을 증가시키고 만료 핸들은 오류다. 명시적 제거가 소유 리소스를 해제한다. 프레임 제출 시 핸들을 이미지 저장소에서 확인하고 프레임 실행 동안 불변 대여하여 제거와 경합하지 않는다. SpriteSheet는 이미지와 양수 셀 크기·행열을 검증하여 반열린 source rect를 반환한다. 영역 밖 source, scale 0과 오버플로는 오류다. 정수 nearest 복사, 뒤집기와 002 합성 규칙을 사용한다. assets는 render에 의존하지 않는다. rpe의 assets/PNG를 선택 feature로 변경하고 기존 기본 CPU 경로는 이미지 디코더 없이 동작하게 한다.

공개 오류는 원인을 보존하고 복구 가능한 입력 오류를 panic으로 처리하지 않는다. 이벤트 루프와 창은 플랫폼 스레드에서 소유하며 불필요한 Send/Sync 제약을 공개 API에 강제하지 않는다.

## 9. Implementation Tasks

### M004-T01 — 이미지 저장소

- 목적: 이미지 저장소 제공.
- 사전 조건: Milestone 003 완료.
- 수정 대상 모듈: assets.
- 구현 사항: PNG 변환, 제한, 세대 핸들과 경로 캐시.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 손상 PNG, 중복 로드, stale 핸들 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M004-T02 — 스프라이트 명령

- 목적: 스프라이트 명령 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: render, render-cpu.
- 구현 사항: 명령과 이미지 조회 경계, 클리핑과 변환 구현.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 시트 경계·뒤집기·투명 픽셀 참조 비교.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M004-T03 — 공개 통합

- 목적: 공개 통합 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: rpe.
- 구현 사항: 선택 feature와 로드·표시 예제를 연결.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: feature별 의존 및 이미지 표시 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

## 10. Test Specification

잘못된 경로/PNG, 0·초과 크기, 만료 핸들, 시트 범위, 정수 확대와 가장자리 클리핑, 반투명 합성을 검사한다.

## 11. Performance Considerations

디코딩 시간, 캐시 적중률과 스프라이트 복사 비용을 측정한다. 측정 환경과 표본을 기록하며 측정 전 성능 수치를 보장하지 않는다.

## 12. Risks

이미지 메모리 상한과 symlink 경로 정규화 실패를 오류로 처리한다.

## 13. Acceptance Criteria

- [ ] 범위 내 기능과 오류 처리가 구현되고 공개 API가 문서화됨.
- [ ] 각 Task의 테스트와 이 문서의 경계 사례가 통과함.
- [ ] 예제와 지원 feature 조합이 대상 환경에서 검증됨.
- [ ] 미지원 환경과 수동 검증 결과가 기록됨.

## 14. Deliverables

대상 크레이트 코드, 단위·통합 테스트, 실행 예제, API 및 사용 문서와 검증 결과.

## 15. Transition

005는 동일한 이미지 핸들과 스프라이트 명령을 사용한다.


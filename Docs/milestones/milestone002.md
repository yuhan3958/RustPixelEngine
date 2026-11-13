# Milestone 002 — CPU Pixel Renderer

## 1. Metadata

- Project: RustPixelEngine
- Milestone Number: 002
- Title: CPU Pixel Renderer
- Target Version: 0.1.0 개발 단계 (릴리스 버전은 008에서 확정)
- Status: Planned
- Dependencies: 001
- Target Crates: rpe-render, rpe-render-cpu, rpe-platform, rpe

## 2. Overview

공통 명령과 CPU 전용 화면 표시 경로를 도입한다. GPU 의존성 없이 논리 프레임을 창에 표시한다.

## 3. Objectives

명시된 범위를 구현하고 오류 및 경계 동작을 검증해 다음 마일스톤에 안정된 인터페이스를 제공한다.

## 4. Scope

Color, Framebuffer, DrawCommand, RenderFrame, RenderBackend, 픽셀·사각형 래스터화, 클리핑, alpha, 정수 배율, 레터박스 및 Game::render. 스프라이트와 GPU는 제외한다.

## 5. Architecture

상위 rpe가 플랫폼, 공통 타입과 선택 구현체를 조립한다. 하위 크레이트는 rpe에 의존하지 않는다. 세부 의존 방향과 색상·좌표 규약은 [로드맵](../roadmap.md)을 따른다.

## 6. Module Responsibilities

대상 크레이트: rpe-render, rpe-render-cpu, rpe-platform, rpe. 각 크레이트는 자신의 데이터 및 동작을 소유하고 rpe가 생명주기와 사용자 API를 연결한다. 외부 리소스는 생성한 모듈이 해제한다.

## 7. Public API Design

다음은 아직 구현되지 않은 설계안이다. 필요한 타입은 같은 마일스톤 및 선행 문서에 정의한다.

```rust
pub struct Color { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }
pub struct Rect { pub x: i32, pub y: i32, pub width: u32, pub height: u32 }
pub enum DrawCommand { Clear(Color), Pixel { x: i32, y: i32, color: Color }, Rect { rect: Rect, color: Color } }
pub struct RenderFrame { pub commands: Vec<DrawCommand> }
pub trait RenderBackend {
    type Error: std::error::Error;
    fn render(&mut self, frame: &RenderFrame) -> Result<(), Self::Error>;
}
```

## 8. Internal Design

Framebuffer는 nonzero width/height와 길이 width×height×4의 row-major RGBA8 Vec<u8>을 소유한다. checked_mul과 할당 오류로 생성 실패를 반환한다. (x,y)의 바이트 시작은 4×(y×width+x), 범위 밖 읽기는 None, 쓰기는 무시한다. clear는 모든 픽셀을 교체한다. 사각형은 i64로 끝점을 계산하고 프레임과 교차시켜 오버플로를 피한다. source-over는 straight alpha: A = As×255 + Ad×(255−As), Ao = round(A/255); 채널은 A=0이면 0, 아니면 round((Cs×As×255 + Cd×Ad×(255−As))/A). round는 양의 정수에 분모/2를 더한 정수 나눗셈이다. 중간값은 u64. Clear는 합성하지 않는다. 창 물리 크기 / 논리 크기의 작은 몫을 정수 배율로 사용하고 중앙 정렬한다. 나머지 픽셀은 검정; 배율 0이면 표시를 생략한다. 창 크기 0에서는 surface를 갱신하지 않는다. CPU 표시 수단은 GPU를 필요로 하지 않는 플랫폼별 software surface 라이브러리를 선정하고 라이선스, 수명, 오류 및 지원 플랫폼을 확인한다. Game::render(&mut self, ctx: &mut Context, frame: &mut RenderFrame)는 기본 빈 구현이며 update 이후 호출한다. 창 이벤트 스레드에서만 표시하고 프레임은 호출 종료 후 비운다.

공개 오류는 원인을 보존하고 복구 가능한 입력 오류를 panic으로 처리하지 않는다. 이벤트 루프와 창은 플랫폼 스레드에서 소유하며 불필요한 Send/Sync 제약을 공개 API에 강제하지 않는다.

## 9. Implementation Tasks

### M002-T01 — 공통 자료형

- 목적: 공통 자료형 제공.
- 사전 조건: Milestone 001 완료.
- 수정 대상 모듈: render.
- 구현 사항: 타입과 검증, 순서 및 오류 모델을 정의.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 좌표·색상·순서 단위 테스트.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M002-T02 — 프레임버퍼

- 목적: 프레임버퍼 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: render-cpu.
- 구현 사항: checked allocation, clear와 픽셀 접근을 구현.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 0 크기와 오버플로 및 마지막 픽셀 검사.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M002-T03 — 래스터화

- 목적: 래스터화 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: render-cpu.
- 구현 사항: 클리핑과 source-over 픽셀 및 사각형을 순서대로 실행.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: 경계 교차, 투명·반투명 합성 정확값 비교.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

### M002-T04 — 창 표시와 연결

- 목적: 창 표시와 연결 제공.
- 사전 조건: 이전 Task 완료.
- 수정 대상 모듈: platform, rpe.
- 구현 사항: 소프트웨어 surface를 선택하고 리사이즈 및 render 콜백을 연결.
- 구현 순서: 요구사항 검증 → 구현 → 오류 전파 → 예제 및 테스트 추가.
- 테스트 방법: Windows 창 표시, 최소화·복구·종료 수동 확인.
- 실패 조건: 오류가 누락되거나 기대 결과와 다름.
- 완료 기준: 해당 테스트와 관련 문서 및 예제가 통과·갱신됨.

## 10. Test Specification

1×1, 빈 사각형, 음수 좌표, 최대 크기 실패, 투명/불투명 및 반투명 합성, 순서 의존, 배율 0/1/2와 홀수 여백을 참조 픽셀과 비교한다. surface 오류는 EngineError로 전달한다.

## 11. Performance Considerations

프레임당 할당과 전체 버퍼 복사량, clear 및 채운 사각형 시간을 측정한다. 측정 환경과 표본을 기록하며 측정 전 성능 수치를 보장하지 않는다.

## 12. Risks

선택한 소프트웨어 surface의 플랫폼 지원과 안전한 창 수명을 확인해야 한다.

## 13. Acceptance Criteria

- [ ] 범위 내 기능과 오류 처리가 구현되고 공개 API가 문서화됨.
- [ ] 각 Task의 테스트와 이 문서의 경계 사례가 통과함.
- [ ] 예제와 지원 feature 조합이 대상 환경에서 검증됨.
- [ ] 미지원 환경과 수동 검증 결과가 기록됨.

## 14. Deliverables

대상 크레이트 코드, 단위·통합 테스트, 실행 예제, API 및 사용 문서와 검증 결과.

## 15. Transition

003은 이 render 콜백과 논리 좌표를 유지한다.


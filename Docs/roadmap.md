# RustPixelEngine 개발 로드맵

## 비전과 원칙

Rust 코드만으로 도트 게임을 만드는 경량 2D 엔진. 논리 픽셀의 일관성, 모듈 분리, 선택 가능한 CPU/GPU/Hybrid 백엔드, 선택적 게임 시스템과 플랫폼 이식성을 지향한다. 에디터와 ECS는 필수가 아니다.

## 현재 상태와 순서

| 단계 | 상태 | 결과물 | 선행 |
|---|---|---|---|
| 001 Runtime Foundation | Implemented | 창, 생명주기, 키보드, 시간 | 없음 |
| 002 CPU Pixel Renderer | Planned | 공통 명령, CPU 래스터화 및 화면 출력 | 001 |
| 003 Input & Timing | Planned | 입력 확장, 고정 갱신 | 002 |
| 004 Sprite & Asset | Planned | PNG, 핸들, 스프라이트 | 002, 003 |
| 005 GPU Renderer | Planned | 공통 명령 GPU 실행 | 004 |
| 006 Hybrid Renderer | Planned | CPU 래스터화 및 GPU 표시 | 005 |
| 007 Game Framework | Planned | 월드, 장면, 타일맵, 애니메이션, 충돌, 오디오, UI | 004, 006 |
| 008 Stabilization & Release | Planned | 호환성 검증 및 배포 | 001–007 |

001은 코드와 단위 테스트 및 창/키보드 예제가 존재하지만 이번 조사에서 빌드, 테스트, GUI 실행을 하지 않았으므로 Verified가 아니다. `Docs/milestone001.md`의 완료 표시는 사용자 제공 맥락이며 독립 검증 결과가 아니다. 렌더링 및 게임 프레임워크 크레이트의 lib.rs는 현재 비어 있다.

## 크레이트와 의존 방향

001: rpe, core, platform, input, math. 002: render, render-cpu. 004: assets. 005: render-gpu. 006: render-hybrid. 007: world, animation, tilemap, physics, audio, ui. 상위 rpe가 조립하며 하위 크레이트가 rpe에 의존하지 않는다. render는 공통 타입을 소유하고 backend 구현체가 이를 의존한다. assets는 render에 의존하지 않는다.

현재 assets는 rpe의 필수 의존성이며 PNG 기본 기능이 켜져 있다. 경량 기본 설치를 위해 004에서 선택적 의존성으로 변경한다. platform의 window feature는 winit을 선택적으로 선언하지만 현재 소스가 무조건 참조하므로 008에서 지원하는 feature 조합을 명시하고 수정한다. Workspace members에 중복 항목이 있어 실제 Cargo 검증 시 정리 여부를 판단한다.

## 공통 규약

논리 좌표는 좌상단 원점, x 오른쪽, y 아래쪽, 사각형은 반열린 범위. 픽셀은 straight-alpha RGBA8 sRGB 바이트로 저장하고 합성은 정수 sRGB 채널 공간의 source-over로 정의한다. CPU를 기준 결과로 삼되 GPU의 정수 정밀도와 표시 surface 색상 변환 차이는 005에서 명시적으로 검증한다. 그리기 순서는 제출 순서다. 정수 배율이 1 미만이면 잘라내지 않고 렌더링을 건너뛰며 검은 화면을 유지한다.

초기 API는 0.x에서 변경 가능하며 변경 사항과 마이그레이션을 기록한다. 008에서 공개 API와 feature 조합을 동결하고 버전 정책을 확정한다. Windows를 우선 검증하고 Linux/macOS는 실제 검증 여부를 별도 표기한다.

## 위험과 릴리스 기준

GPU와 CPU의 색상/반올림 차이, surface 수명과 재생성, 플랫폼 표시 라이브러리, 입력 전이의 고정 갱신 전달, 선택 feature의 의존성 누수, 라이선스 및 배포 가능성이 핵심 위험이다. 각 단계의 테스트와 008의 플랫폼·feature 매트릭스, 문서, 예제, 라이선스 및 패키징 검토를 통과해야 릴리스한다. 수치 성능 목표는 기준 장비에서 측정한 뒤 설정한다.

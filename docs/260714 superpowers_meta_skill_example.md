# Superpowers를 이용한 메타 Skill 예시

## 목적

이 문서는 프로젝트 Recipe가 Superpowers의 범용 Skill을 재사용하면서, 프로젝트 고유 Memory와 판단 기준을 결합하는 방식을 보인다. 예시는 `feature-development` Recipe이지만, 버그 수정·리뷰·배포 Recipe에도 같은 구조를 적용할 수 있다.

이 Recipe는 Superpowers Skill을 복사하거나 수정하지 않는다. Recipe는 작업의 진입점으로서 현재 요청에 필요한 Memory를 고르고, 적용할 Superpowers 절차와 프로젝트 고유 Skill을 판단해 조합한다.

## 책임 분리

```text
사용자 기능 요청
    ↓
feature-development Recipe (메타 Skill)
    ├─ Memory Index 조회 및 관련 지식 선택
    ├─ 단순/심화 경로 판단
    ├─ Superpowers Skill 호출 조건 결정
    ├─ 프로젝트 고유 Skill·도구 선택
    └─ 회고 및 Memory 후보 제안
    ↓
Superpowers Skill
    ├─ brainstorming: 설계 대화와 사용자 승인
    ├─ writing-plans: 상세 구현 계획
    ├─ test-driven-development: 구현 전 테스트 설계
    └─ verification-before-completion: 완료 주장 전 검증
```

Recipe는 **무엇을, 언제, 어떤 프로젝트 맥락에서 사용할지**를 책임진다. Superpowers는 **선택된 일반 절차를 어떻게 품질 있게 수행할지**를 책임진다. Memory는 절차를 명령하지 않고, 저장소의 사실·결정·사례를 제공한다.

## 동작 예시

```text
1. 기능 요청 수신
2. Memory Index에서 관련 도메인·설정·아키텍처 지식의 압축본을 조회
3. 압축본으로 충분한지 판단하고, 필요한 경우에만 정본 확인
4. 영향 범위와 복잡도를 분류
   - 단순: 기존 경계 안의 작은 변경
   - 심화: 새 도메인 규칙, 외부 의존성, 다중 진입점, 장기 확장 부담
5. 의미 있는 변경이면 Superpowers brainstorming으로 설계와 사용자 승인 진행
6. 승인 후 Superpowers writing-plans로 세부 계획 작성
7. 구현 시 Superpowers test-driven-development 및 프로젝트 고유 Skill 적용
8. 완료 전 Superpowers verification-before-completion으로 검증
9. 선택한 지식·절차의 적절성을 회고하고, 영속 Memory 후보를 사람에게 제안
```

단순 경로도 검증을 생략하지 않는다. 다만 현재 코드와 Memory가 충분히 뒷받침하는 작은 변경에 불필요한 아키텍처 설계 단계를 강제하지 않는다.

## Recipe 템플릿

아래는 프로젝트에 둘 `feature-development` Recipe의 내용 예시다. 실제 파일 위치와 호출 방식은 사용 중인 하네스의 Skill 규약에 맞춘다.

```md
---
name: feature-development
description: 프로젝트 지식과 Superpowers 절차를 조합해 기능 개발을 수행하는 진입점 Recipe
---

# Feature Development Recipe

## 입력

- 사용자의 기능 요청과 완료 기준
- 현재 저장소 코드와 관련 변경 사항

## 컨텍스트 수집

1. Memory Index에서 요청의 도메인·기술·설정 태그로 압축본을 찾는다.
2. 압축본으로 판단할 수 있으면 정본을 읽지 않는다.
3. 정보 부족, 고위험 변경, 문서·코드 충돌, 오래된 요약, 사용자의 원문 요청일 때만 정본을 확인한다.

## 경로 판단

다음 중 하나가 있으면 심화 경로를 선택한다.

- 새 도메인 규칙 또는 중요한 상태 전이
- 외부 서비스·영속성·네트워크 경계 추가 또는 변경
- 여러 진입점 또는 모듈에 걸친 영향
- 장기적인 확장·호환성·보안 부담

그 밖에는 기존 모듈 경계 안의 단순 경로를 우선 고려한다. 분류가 불명확하면 사용자에게 확인한다.

## 절차 조합

- 의미 있는 변경: Superpowers `brainstorming`으로 설계를 제시하고 사용자 승인을 받는다.
- 승인된 설계: Superpowers `writing-plans`로 구현 계획을 만든다.
- 구현: Superpowers `test-driven-development`와 필요한 프로젝트 고유 Skill을 적용한다.
- 완료 전: Superpowers `verification-before-completion`으로 실제 검증 결과를 확인한다.

## 종료

- 결과, 검증 근거, 잔여 위험을 사용자에게 보고한다.
- 새 결정 또는 재사용 가능한 사례가 있으면 Memory 후보로만 제안한다.
- 사람의 승인 전에는 영속 Memory나 Memory Index를 변경하지 않는다.
```

## 판단 예시

| 요청 | 선택 경로 | Memory와 Superpowers의 역할 |
|---|---|---|
| 기존 API 응답에 선택 필드 하나 추가 | 단순 | 관련 구현·API 관례 압축본을 읽고, 필요한 테스트와 검증을 적용한다. |
| 결제 수단과 외부 결제사를 추가 | 심화 | 결제·보안·설정·아키텍처 Memory를 조회하고, `brainstorming`과 계획 수립 뒤 구현한다. |
| 설정 파일 위치를 찾고 설명 | 직접 응답 | 설정 `implementation` Memory 압축본을 읽고 답한다. Recipe 전체를 시작하지 않는다. |

## 경계와 주의점

- Recipe는 Superpowers의 절차를 대체하거나 그 내부 내용을 복제하지 않는다.
- 직접 사용자 지시와 `AGENTS.md`가 Recipe보다 우선한다.
- Memory는 지시문이 아니다. Recipe가 판단에 사용할 근거를 제공할 뿐이다.
- Superpowers Skill의 실제 이름·사용 가능 여부는 실행 환경에서 확인한다. 사용할 수 없으면 Recipe는 대체 절차를 명시하거나 사용자에게 알린다.
- Memory 영속화는 자동화하지 않는다. LLM은 후보를 제안하고, 사람은 승인 여부를 결정한다.

## 이 예시를 정련하는 방법

실제 기능 개발을 이 Recipe로 몇 번 수행한 뒤, 자주 반복되고 독립적으로 이해·검증 가능한 단계만 프로젝트 고유 Skill로 추출한다. 예를 들어 API 계약 검토, 설정 변경 검증, 특정 모듈 테스트 준비가 반복된다면 그때 독립 Skill 후보가 된다.

이 순서는 Recipe를 먼저 실사용으로 정련하고, 검증된 작업 단위만 Skill로 승격한다는 하네스 원칙을 따른다.

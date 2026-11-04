수정 폼의 **커뮤니티 가이드라인(Community guidelines)** 필드는 이 에이전트가 실행될 때마다 사용자 역할 컨텍스트 메시지에 포함되는 선택적 정책 텍스트 블록입니다. 이 텍스트는 신뢰할 수 없는 텍스트로 펜싱됩니다(플랫폼이 댓글 본문 및 기타 사용자 제공 콘텐츠에 적용하는 것과 동일한 펜싱 방식). 따라서 모델은 이를 시스템 지시가 아닌 정책 참조 자료로 취급합니다. "이 사이트에서 어떤 행동이 허용되고 허용되지 않는가"를 기록하여 에이전트가 일관되게 적용하도록 하는 표준적인 장소입니다.

### 초기 프롬프트와의 차이점

- **초기 프롬프트(Initial prompt)** - 에이전트의 역할과 의사 결정 스타일. "당신은 운영자입니다. 차단보다 경고를 선호하세요."
- **커뮤니티 가이드라인(Community guidelines)** - 정책 언어로 작성된 커뮤니티의 규칙. "인신공격 금지. 24시간 미만 계정의 홍보 링크 금지. 스레드가 과열된 경우 주제에서 벗어난 댓글은 삭제할 수 있습니다."

두 항목 모두 동일한 컨텍스트 창으로 흘러 들어가지만 서로 다른 계층에서 들어옵니다. 초기 프롬프트는 시스템 역할의 일부이고, 가이드라인 문서는 사용자 역할 컨텍스트 메시지 안에 펜싱된 텍스트입니다. 이러한 분리 덕분에 하나를 업데이트할 때 다른 하나를 다시 읽지 않고도 수정하기가 쉬워집니다.

### 좋은 가이드라인 문서란

짧고, 구체적이며, 사람이 직접 작성한 문서입니다. 산문보다 목록 형식이 더 효과적입니다:

[inline-code-attrs-start title = '커뮤니티 가이드라인 예시'; type='text' inline-code-attrs-end]
[inline-code-start]
Allowed:
- Substantive disagreement, even strongly worded.
- Links to original sources, even from new accounts.
- Off-topic asides if the parent thread permits them.

Not allowed:
- Personal attacks against specific named users.
- Doxxing or sharing of private information.
- Coordinated promotional activity (multiple comments pushing the same external link).
- Comments that exist only to derail discussion.

Borderline:
- Strong language without a target. Allowed if not directed at a person.
- Political topics outside the page subject. Off-topic; warn first, don't remove unless persistent.
[inline-code-end]

에이전트는 매 실행마다 이를 적용합니다. 가이드라인을 변경하면 다음 트리거부터 변경 사항이 적용됩니다. 과거 실행에 대해 소급하여 재평가되지는 않습니다.

### 여기에 넣으면 안 되는 것

- **출력 형식 지시** ("HTML로 답변", "이모지 사용"). 이러한 내용은 [초기 프롬프트](#personality-prompt)에 속합니다.
- **현지화된 텍스트.** 가이드라인 문서는 프롬프트와 마찬가지로 동일한 이유로 **영어로만** 작성해야 합니다. 기계 번역은 에이전트의 동작을 조용히 바꿀 수 있습니다. 로캘별로 다른 정책이 있다면 모두 영어로 이 하나의 문서에 작성하고 "독일어 페이지의 경우: ..."와 같은 구조로 문서를 구성하세요.
- **외부 정책의 긴 인용.** 의역하세요. 긴 컨텍스트는 매 실행마다 토큰 비용을 소모합니다.
- **PII 또는 비밀 정보.** 이 텍스트는 매 실행마다 LLM 제공업체로 전송됩니다.

### 길이

이 필드는 **4000자**로 제한됩니다(폼과 저장 라우트 양쪽에서 강제됨). 매 실행의 토큰 비용은 길이에 비례하므로, 제한 내에서도 수백 단어면 보통 충분합니다. 커뮤니티 정책이 여러 페이지에 걸친다면 에이전트에 필요한 부분을 요약하여 이 필드 전용 다이제스트로 만드세요.

### 버전 관리

가이드라인 문서에는 내장된 버전 기록이 없습니다. 에이전트는 가장 최근에 저장된 값을 사용합니다. 기록을 원한다면 주요 수정 전에 문서를 자체 추적 시스템에 복사하세요. [프롬프트 다듬기(Refine Prompts)](#refining-prompts) 흐름은 *초기 프롬프트*의 변경 사항을 기록할 수 있지만 가이드라인 문서는 버전 관리하지 않습니다.
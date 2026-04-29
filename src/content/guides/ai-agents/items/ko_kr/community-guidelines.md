---
편집 폼의 **Community guidelines** 필드는 이 에이전트가 실행될 때마다 사용자 역할 컨텍스트 메시지에 포함되는 선택적 정책 텍스트 블록입니다. 이 필드는 신뢰할 수 없는 텍스트로 펜스 처리됩니다(플랫폼이 댓글 본문 및 기타 사용자 제공 콘텐츠에 적용하는 동일한 펜싱), 따라서 모델은 이를 시스템 지시가 아닌 정책 참조로 취급합니다. 이 필드는 "이 사이트에서 어떤 행동이 허용되며 허용되지 않는지"를 기록하는 표준화된 장소이므로 에이전트가 일관되게 적용할 수 있습니다.

### How it differs from the initial prompt

- **Initial prompt** - 에이전트의 역할과 의사결정 방식. "You are a moderator. Prefer warning over banning."
- **Community guidelines** - 정책 언어로 작성된 커뮤니티의 규칙들. "No personal attacks. No promotional links from accounts under 24 hours old. Off-topic comments may be removed if a thread is heated."

두 요소는 동일한 컨텍스트 창으로 흘러들어가지만 서로 다른 레이어로 들어갑니다 - 초기 프롬프트는 시스템 역할의 일부인 반면, 가이드라인 문서는 사용자 역할 컨텍스트 메시지 안의 펜스된 텍스트입니다. 이 분리는 한 쪽을 수정할 때 다른 쪽을 다시 읽지 않아도 되어 편집을 더 쉽게 만듭니다.

### What's a good guidelines doc

짧고 구체적이며 사람이 작성한 문서. 산문보다 목록 형식이 더 효과적입니다:

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

에이전트는 이를 매 실행마다 적용합니다. 가이드라인을 변경하면 변경 사항은 다음 트리거에서 적용되며, 이전 실행에는 소급하여 재평가되지 않습니다.

### What not to put here

- **Output formatting instructions** ("reply in HTML", "use emoji"). Those belong in the [initial prompt](#personality-prompt).
- **Localized text.** The guidelines doc, like the prompt, is **English-only** for the same reason - machine translation can change agent behavior silently. If you have policies that vary by locale, write them all in English in this one document and structure the doc as "for German-language pages: ..."
- **Long quotations of external policies.** Paraphrase. Long context costs tokens on every run.
- **PII or secrets.** This text is sent to the LLM provider on every run.

### Length

이 필드는 **4000 characters**로 제한됩니다(양식과 저장 경로 모두에서 강제됩니다). 매 실행의 토큰 비용은 길이에 비례하므로, 허용치 내라도 보통 몇백 단어면 충분합니다. 커뮤니티 정책이 여러 페이지에 걸친다면 에이전트가 필요로 하는 부분을 이 필드에 맞춘 간략한 요약으로 정리하세요.

### Versioning

가이드라인 문서에는 내장된 버전 기록이 없으며 - 에이전트는 가장 최근에 저장된 값을 사용합니다. 기록을 남기고 싶다면 주요 편집 전마다 문서를 자체 추적 시스템에 복사하세요. The [Refine Prompts](#refining-prompts) flow can record changes to the *initial prompt* but does not version the guidelines doc.
---
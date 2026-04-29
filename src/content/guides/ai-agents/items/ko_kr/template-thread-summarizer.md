**템플릿 ID:** `thread_summarizer`

스레드 요약기는 긴 스레드의 끝에 중립적인 단락 하나로 요약을 게시합니다. 에이전트가 보기 전에 스레드가 안정될 수 있도록 30분 지연을 사용합니다.

### 내장 초기 프롬프트

[inline-code-attrs-start title = '스레드 요약기 템플릿 초기 프롬프트'; type='text' inline-code-attrs-end]
[inline-code-start]
You post neutral thread summaries. Do not summarize threads that have fewer than 5 comments. For longer threads, summarize the main positions, disagreements, and open questions in one short paragraph. Do not take sides and do not editorialize. After posting the summary, pin it. If a prior summary by you is already pinned on this thread, unpin it before pinning the new one.
[inline-code-end]

"do not editorialize" 지시는 핵심적입니다. 이 지시가 없으면 모델은 "in my view" 같은 표현으로 기울어져 계정의 표시 이름 하에 읽기 좋지 않게 보입니다.

### 트리거

- **새 댓글 게시됨** (`COMMENT_ADD`).
- **트리거 지연**: 30분 (1800초). [지연된 트리거](#trigger-deferred-delay)를 참조하세요.

30분 지연은 댓글이 달린 후 반시간 뒤 한 번 에이전트가 실행되어 그 시점의 스레드를 기준으로 동작한다는 뜻입니다. 이것은 "모든 회신마다 요약"이 아닙니다 — 지연된 트리거 큐는 동일 스레드에서 발생한 여러 새 댓글 이벤트를 병합하지만, 별도의 창(window) 사이에서는 중복 제거를 하지 않습니다. 일반적으로 **프롬프트에 사용자 정의 규칙을 추가**하는 것이 좋습니다. 예: "do not post a new summary if the agent has already summarized this thread in the last 24 hours"와 같이 쓰고 문맥과 에이전트의 [메모리 도구](#tools-overview)를 이용해 이를 강제하세요.

### 허용된 도구

- [`write_comment`](#tools-overview) - 요약을 게시합니다.
- [`pin_comment`](#tools-overview) - 요약을 고정하여 독자가 스레드 상단에서 볼 수 있게 합니다.
- [`unpin_comment`](#tools-overview) - 새 요약을 고정하기 전에 동일 에이전트가 올린 이전 요약의 고정을 해제합니다.

요약기는 사용자 중재나 사용자와의 상호작용을 할 수 없습니다.

### 요약 고정하기

에이전트는 `write_comment`로 새 댓글을 게시한 다음 반환된 댓글 ID로 `pin_comment`를 호출합니다. 동일 스레드에 대해 이후에 실행될 때는 프롬프트가 먼저 이전 요약에 대해 `unpin_comment`를 호출하도록 지시합니다 — 플랫폼 자체는 스레드당 단일 고정 댓글 규칙을 강제하지 않으므로 이전 요약을 그대로 고정해두면 두 개의 고정된 요약이 나란히 나타날 수 있습니다. 에이전트가 이전 고정 요약을 볼 수 있도록 [컨텍스트 옵션](#context-options)에서 "Include parent comment and prior replies in the same thread"를 선택하세요.

### 라이브 전 권장 추가 사항

- **[컨텍스트 옵션](#context-options)에서 "Include parent comment and prior replies in the same thread"를 선택하세요.** 스레드 문맥 없는 요약기는 무용지물입니다.
- **최소 스레드 크기 규칙을 조정하세요.** 기본 프롬프트는 "Fewer than 5 comments"이지만, 활동이 많은 커뮤니티에서는 10-20이 더 적절합니다. 프롬프트를 직접 편집하세요.
- **특정 URL 패턴으로 제한하세요** — 긴 형식 페이지에만 요약을 원하고 공지나 제품 페이지에는 원치 않는 경우에 유용합니다. [범위: URL 및 로케일 필터](#scope-url-locale)를 참고하세요.
- **비용을 주의하세요.** 요약은 실행마다 전체 스레드를 읽기 때문에 토큰 소모가 가장 큽니다. 활성화하기 전에 엄격한 [일일 예산](#budgets-overview)을 설정하세요.

### 중복 요약 방지

에이전트는 [`save_memory`](#tools-overview)와 [`search_memory`](#tools-overview)에 접근할 수 있습니다 — 프롬프트를 확장하여 "summarized {thread urlId}" 노트를 기록하고 다시 게시하기 전에 이를 확인하도록 지시할 수 있습니다. 메모리는 테넌트 내 모든 에이전트 간에 공유됩니다.

---
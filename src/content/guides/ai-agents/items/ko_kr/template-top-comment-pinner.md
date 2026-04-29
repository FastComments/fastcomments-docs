**템플릿 ID:** `top_comment_pinner`

상단 댓글 고정기는 투표 임계값을 넘는 최상위 댓글을 감시하고, 고정하여 동일한 스레드에서 이전에 고정된 항목을 대체합니다.

### 내장 초기 프롬프트

[inline-code-attrs-start title = '상단 댓글 고정기 템플릿 초기 프롬프트'; type='text' inline-code-attrs-end]
[inline-code-start]
You pin the best top-level comments on a thread. When a comment reaches the vote threshold, pin it if the content is substantive and non-promotional. Unpin any previously pinned comment on the same thread first. Do not pin replies, only top-level comments.
[inline-code-end]

"답글을 고정하지 말라"는 지침은 중요합니다: 고정은 스레드 단위로 작동하므로 답글을 고정하는 것은 거의 유용하지 않습니다. "비홍보성" 필터는 에이전트가 인기 있는 링크 스팸 댓글을 증폭시키지 않도록 방지합니다.

### 트리거

- **댓글이 투표 임계값을 넘을 때** (`COMMENT_VOTE_THRESHOLD`, 기본 투표 임계값: 10).

트리거는 댓글의 순투표수(`up - down`)가 설정된 임계값에 도달하면 작동합니다. 스레드 활동 수준에 따라 편집 폼에서 수치를 조정하세요 - 활동이 중간 정도인 사이트에는 10이 합리적인 기본값입니다.

### 허용된 도구

- [`pin_comment`](#tools-overview)
- [`unpin_comment`](#tools-overview)

고정은 비파괴적이므로 즉시 되돌릴 수 있습니다 - 따라서 이 템플릿은 보통 승인 없이 실행됩니다.

### 라이브 적용 전에 권장 사항

- [Context Options](#context-options)에서 **"Include parent comment and prior replies in the same thread"** 옵션을 선택하세요. 스레드 컨텍스트가 없으면 에이전트가 이미 고정된 댓글을 고정 해제해야 하는지를 신뢰성 있게 판단할 수 없습니다.
- **투표 임계값을 사이트에 맞게 조정**하세요. 바쁜 스레드에서는 10이 너무 자주 발생할 수 있고, 조용한 스레드에서는 10이 거의 일어나지 않을 수 있습니다.
- **URL별로 범위를 제한하는 것을 고려**하세요. 특정 섹션(예: 뉴스 스레드)에서만 고정 댓글을 허용하고 공지 스레드에서는 허용하지 않으려는 경우 유용합니다.

### 중복 고정에 대한 참고

에이전트의 프롬프트는 고정하기 전에 먼저 고정 해제를 지시하지만, 모델이 해당 단계를 놓치면 플랫폼 자체는 스레드당 하나의 고정만 허용하는 규칙을 강제하지 않습니다(여러 개가 존재할 수 있음). 사이트에서 중복 고정이 문제가 된다면 `pin_comment`를 승인 뒤에 두고 각 항목을 검토하거나 더 엄격한 프롬프트를 작성하세요.

---
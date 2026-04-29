**템플릿 ID:** `welcome_greeter`

The Welcome Greeter replies warmly to first-time commenters. It is the lowest-risk template (no destructive tools) and a good first agent to ship live.

### 내장 초기 프롬프트

[inline-code-attrs-start title = '환영 인사 템플릿 초기 프롬프트'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### 트리거

- **New user posts their first comment on this site** (`NEW_USER_FIRST_COMMENT`).

This event fires exactly once per user, so the agent cannot loop. See [트리거: 새 사용자 첫 댓글](#trigger-new-user-first-comment).

### 허용된 도구

- [`write_comment`](#tools-overview)

That is the only tool - the agent literally cannot moderate, vote, ban, or DM.

### 라이브 전 권장 추가 사항

- **표시 이름을 설정**하여 친근한 이름으로 설정하세요 — "Community Bot", 사이트 마스코트, 또는 브랜드 이름 등. 표시 이름은 독자들이 환영 답글 옆에서 보게 되는 이름입니다.
- **[Context Options](#context-options)에서 "페이지 제목, 부제, 설명 및 메타 태그 포함"을 선택**하세요. 그리터가 페이지의 실제 내용을 참조할 수 있을 때 응답이 눈에 띄게 좋아집니다.
- 여러 언어로 운영하는 경우 **로케일 제한을 고려**하세요. 잘못된 언어로 된 환영 답글은 놓친 답글보다 더 어색할 수 있습니다. 자세한 내용은 [Scope: URL and Locale Filters](#scope-url-locale)를 참조하세요.

### 승인이 필요 없는 이유

The agent only writes new comments and only on a one-shot trigger. Worst case: an awkward greeting. There is no destructive action to gate. Most operators run this one with no approvals at all once dry-run looks clean.

---
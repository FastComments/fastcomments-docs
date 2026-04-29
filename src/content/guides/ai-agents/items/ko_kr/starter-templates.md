---
FastComments는 바로 사용할 수 있는 네 가지 시작 템플릿을 제공하므로 작동하는 에이전트를 처음부터 작성할 필요가 없습니다. 템플릿은 [AI 에이전트 페이지](https://fastcomments.com/auth/my-account/ai-agents)에서 **템플릿 둘러보기**를 클릭하여 접근할 수 있습니다.

템플릿을 선택하면:

1. 에이전트가 **상태: Dry Run**으로 생성되며 템플릿 기반의 내부 이름이 부여됩니다 (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). 해당 이름이 귀하의 테넌트에 이미 존재하면 숫자 접미사가 추가됩니다.
2. 프롬프트, 트리거, 허용된 액션 및 임계값 등 모든 항목이 미리 채워진 편집 폼으로 바로 이동합니다. 상단 배너에는 "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready." 라고 표시됩니다.
3. 아직 아무것도 활성화되어 있지 않습니다. 에이전트는 저장하고 관찰을 위해 dry-run을 유지하거나 Enabled로 전환할 때까지 동작하지 않습니다.

### 네 가지 템플릿

- **[Moderator](#template-moderator)** - 새 댓글과 신고된 댓글을 검토하고, 초범에게 경고를 주며 경고 이후에만 차단으로 단계적으로 전환합니다. 새 댓글 및 신고 임계값 도달 시 트리거됩니다(기본 신고 임계값: 3). 허용된 도구: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - 첫 댓글 작성자에게 짧고 개인적인 환영 메시지로 따뜻하게 응답합니다. 트리거: new-user-first-comment. 허용된 도구: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - 상위 수준의 실질적인 댓글이 투표 임계값을 넘으면(기본값: 10) 해당 댓글을 고정하고, 먼저 이전에 고정된 댓글을 고정 해제합니다. 트리거: 투표 임계값 도달. 허용된 도구: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - 긴 스레드에 대해 지연 후 중립적이고 한 문단짜리 요약을 게시한 다음 이를 고정합니다. 스레드가 안정될 수 있도록 30분 지연을 둔 새 댓글에 대해 트리거됩니다. 허용된 도구: `write_comment`, `pin_comment`, `unpin_comment`.

### 템플릿 맞춤 설정

템플릿은 출발점일 뿐이며 변경 불변의 약속이 아닙니다. 다음을 수행하는 것이 기대됩니다:

- **Initial prompt**을 커뮤니티의 어조에 맞게 조정합니다.
- 에이전트가 얼마나 자주 실행되어야 하는지에 맞추어 **Triggers**를 추가하거나 제거합니다.
- 민감한 액션에는 **Approvals**를 추가하세요 — 모더레이터 스타일 템플릿의 경우 `ban_user`는 승인 뒤에 두는 것을 강력히 권장합니다.
- 에이전트가 작성된 정책을 일관되게 적용할 수 있도록 **Community guidelines**를 추가합니다. 자세한 내용은 [커뮤니티 지침](#community-guidelines)을 참조하세요.
- 예상 트리거 수에 적합한 에이전트별 **Budgets**를 설정하세요.

템플릿은 합리적인 기본값을 미리 채워주는 수단일 뿐이며; 저장하면 에이전트는 귀하의 것입니다.

---
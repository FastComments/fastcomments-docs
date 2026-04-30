FastComments는 작동하는 에이전트를 처음부터 작성할 필요가 없도록 다섯 가지 시작 템플릿을 제공합니다. 템플릿은 [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents)에서 **Browse templates**를 클릭하면 접근할 수 있습니다.

템플릿을 선택하면:

1. 에이전트는 **Status: Dry Run** 상태로 생성되며 템플릿 기반의 내부 이름이 지정됩니다 (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). 해당 이름이 테넌트에 이미 존재하면 숫자 접미사가 추가됩니다.
2. 프롬프트, 트리거, 허용된 동작 및 임계값 등 모든 항목이 미리 채워진 편집 폼으로 바로 이동합니다. 상단 배너에는 "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready." 라고 표시됩니다.
3. 아직 아무 것도 활성화되지 않습니다. 저장하고 Dry Run을 유지(관찰용)하거나 Enabled로 전환할 때까지 에이전트는 작동하지 않습니다.

### 다섯 가지 템플릿

- **[Moderator](#template-moderator)** - 새 댓글과 플래그된 댓글을 검토하고, 초범 사용자에게 경고를 주며 경고 후에만 차단으로 에스컬레이션합니다. 신규 댓글 및 플래그 임계값 초과 시 트리거됩니다(기본 플래그 임계값: 3). 허용 도구: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - 처음 댓글을 다는 사용자에게 짧고 개인적인 환영 인사를 따뜻하게 답글로 보냅니다. 트리거: new-user-first-comment. 허용 도구: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - 실질적인 최상위 댓글이 투표 임계값을 넘으면(기본: 10) 이전에 고정된 댓글을 먼저 해제한 후 해당 댓글을 고정합니다. 트리거: vote-threshold crossings. 허용 도구: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - 긴 스레드에 대해 지연 후 중립적이고 한 문단짜리 요약을 게시한 다음 이를 고정합니다. 스레드가 정착될 수 있도록 30분 지연을 둔 신규 댓글에서 트리거됩니다. 허용 도구: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - 스레드 중간에서 댓글 수정을 감시하여 답글의 의미를 왜곡시키는 재작성 행위를 감지하고 원본 텍스트를 복원하며 작성자에게 DM을 보냅니다. 트리거: comment edits. 허용 도구: `edit_comment`, `warn_user`, `send_dm`.

### 템플릿 사용자화

템플릿은 출발점일 뿐이며 변경 불가의 약속이 아닙니다. 다음을 수행하는 것이 기대됩니다:

- 커뮤니티 목소리에 맞게 **Initial prompt**를 조정하세요.
- 에이전트가 실행되는 빈도에 맞게 **Triggers**를 추가하거나 제거하세요.
- 민감한 동작에 대해서는 **Approvals**를 추가하세요 - 모더레이터 스타일 템플릿의 경우 `ban_user`는 승인 뒤에 두는 것을 강력히 권장합니다.
- 에이전트가 문서화된 정책을 일관되게 적용할 수 있도록 **Community guidelines**를 추가하세요. 자세한 내용은 [Community Guidelines](#community-guidelines)를 참조하세요.
- 예상되는 트리거 수에 적절한 에이전트별 **Budgets**를 설정하세요.

템플릿은 합리적인 기본값을 미리 채워주는 수단일 뿐이며, 저장하면 그 에이전트는 귀하의 것입니다.
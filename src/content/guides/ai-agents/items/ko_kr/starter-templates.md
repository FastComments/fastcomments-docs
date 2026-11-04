FastComments는 처음부터 작동하는 에이전트를 직접 작성하지 않아도 되도록 다섯 가지 스타터 템플릿을 제공합니다. [AI Agents 페이지](https://fastcomments.com/auth/my-account/ai-agents)에서 **Browse templates**를 클릭하면 이 템플릿들에 접근할 수 있습니다.

템플릿을 선택하면:

1. 에이전트가 **Status: Dry Run** 상태로 생성되며, 템플릿을 기반으로 한 내부 이름(`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`)이 부여됩니다. 해당 이름이 이미 테넌트에 존재하는 경우 숫자 접미사가 추가됩니다.
2. 프롬프트, 트리거, 허용된 작업 및 각종 임계값이 모두 미리 채워진 편집 폼으로 바로 이동합니다. 상단 배너에는 "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."라는 문구가 표시됩니다.
3. 아직 아무것도 활성화되지 않습니다. 저장한 후 드라이런을 유지하며(관찰용) 상태를 Enabled로 변경하기 전까지는 에이전트가 작동하지 않습니다.

### 다섯 가지 템플릿

- **[Moderator](#template-moderator)** - 새 댓글과 신고된 댓글을 검토하고, 초반 위반자에게 경고를 보내며, 경고 후에만 차단으로 에스컬레이션합니다. 새 댓글 및 신고 임계값 초과 시 트리거됩니다(기본 신고 임계값: 3). 허용된 도구: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - 처음 댓글을 작성한 사용자에게 짧고 따뜻한 개인화 환영 답글을 남깁니다. 신규 사용자의 첫 댓글 시 트리거됩니다. 허용된 도구: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - 유의미한 최상위 댓글이 투표 임계값(기본값: 10)을 넘으면 고정하고, 먼저 이전에 고정된 댓글의 고정을 해제합니다. 투표 임계값 초과 시 트리거됩니다. 허용된 도구: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - 긴 스레드에 지연 시간 후 중립적인 단일 문단 요약을 게시한 뒤 고정합니다. 새 댓글 시 트리거되며, 스레드가 정리될 수 있도록 30분 지연을 둡니다. 허용된 도구: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - 스레드 도중 답글의 맥락을 왜곡하는 댓글 수정을 감시하고, 원본 텍스트를 복원하며 작성자에게 DM을 보냅니다. 댓글 수정 시 트리거됩니다. 허용된 도구: `edit_comment`, `warn_user`, `send_dm`.

### 템플릿 커스터마이징

템플릿은 시작점이지 계약이 아닙니다. 다음을 수행하는 것이 좋습니다:

- 커뮤니티의 톤에 맞게 **Initial prompt**를 조정합니다.
- 에이전트가 실행되는 빈도에 맞게 **Triggers**를 추가하거나 제거합니다.
- 민감한 작업에 대해 **Approvals**를 추가합니다. 모더레이터 스타일 템플릿의 경우 `ban_user`를 승인 뒤에 두는 것을 강력히 권장합니다.
- 에이전트가 작성된 정책을 일관되게 적용하도록 **Community guidelines**를 추가합니다. [Community Guidelines](#community-guidelines)를 참고하세요.
- 예상되는 트리거 수에 적합한 에이전트별 **Budgets**를 설정합니다.

템플릿은 합리적인 기본값을 미리 채워주는 수단일 뿐이며, 저장하는 순간 에이전트는 여러분의 것입니다.
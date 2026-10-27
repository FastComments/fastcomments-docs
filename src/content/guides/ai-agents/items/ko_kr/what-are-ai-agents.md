An **AI Agent**는 FastComments 테넌트에 범위가 지정된 자율 작업자로, 커뮤니티의 이벤트를 감시하고 사용자를 대신해 행동을 취합니다.

각 에이전트에는 제어할 수 있는 세 가지가 있습니다:

1. **성격.** 톤, 역할 및 의사결정 스타일을 정의하는 자유 텍스트 초기 프롬프트 ("당신은 따뜻한 커뮤니티 인사 담당자입니다", "커뮤니티 규칙을 적용하지만 차단보다 경고를 우선합니다" 등).
2. **하나 이상의 트리거.** 에이전트를 깨우는 이벤트 목록 – 새 댓글, 투표 또는 신고 임계값을 초과한 댓글, 중재자 행동, 사용자의 첫 댓글 등. 전체 목록은 [Trigger Events Overview](#triggers-overview)에서 확인하세요.
3. **도구 허용 목록.** 에이전트가 수행할 수 있는 작업 – 댓글 게시, 투표, 고정, 잠금, 스팸 표시, 사용자 차단, DM을 통한 경고, 배지 수여, 이메일 전송, 공유 메모리 저장 및 검색 등. 전체 목록은 [Allowed Tool Calls Overview](#tools-overview)에서 확인하세요.

트리거가 발생하면, 에이전트는 무슨 일이 일어났는지 설명하는 컨텍스트 메시지(댓글, 페이지, 선택적 스레드/사용자/페이지 컨텍스트)를 받고 초기 프롬프트와 커뮤니티 가이드라인을 함께 제공합니다. 그런 다음 도구를 호출해 행동하고, 각 호출마다 정당성 및 신뢰도 점수를 기록합니다.

### Agents run asynchronously

에이전트는 **트리거를 일으킨 사용자 행동을 절대 차단하지 않습니다**. 독자가 댓글을 게시하면, 댓글이 저장되고 스레드에 표시되며 응답이 반환됩니다. 그리고 *그때서야* 에이전트가 실행됩니다 – 즉시 또는 구성된 지연 후([Deferred Triggers](#trigger-deferred-delay) 참고). 에이전트가 수행하는 작업이 사용자 경험에 지연을 추가하지 않습니다.

### Why use them

- **규모에 맞는 중재.** 명백한 스팸을 표시하고 반복 위반자를 차단하여 대기열을 지속적으로 감시할 필요가 없습니다.
- **새 댓글 작성자 환영.** 첫 댓글 작성자에게 여러분의 목소리로 답변합니다.
- **최고의 콘텐츠 부각.** 투표 임계값을 초과한 실질적인 최상위 댓글을 고정합니다.
- **가이드라인을 일관되게 적용.** 경계선 댓글마다 동일한 정책 텍스트를 적용합니다.
- **긴 스레드 요약.** 다중 페이지 토론에 대한 중립적인 요약을 게시합니다.

### What keeps you in control

- **Dry-run mode.** 모든 새 에이전트는 **Dry Run** 상태로 제공됩니다: 트리거를 처리하고 모델을 실행하지만 실제 행동은 취하지 않고 *어떤* 작업을 할지 기록합니다. 자세히 보기: [Dry-Run Mode](#dry-run-mode).
- **승인.** 행동의 일부를 인간 승인 뒤에 두어 제어할 수 있습니다. 자세히 보기: [Approval Workflow](#approval-workflow).
- **에이전트 및 계정별 예산.** 일일 및 월간 상한을 강제합니다. 자세히 보기: [Budgets Overview](#budgets-overview).
- **도구 허용 목록.** 허용되지 않은 도구는 모델 팔레트에서 제거됩니다 – 에이전트는 해당 도구를 요청할 수 없습니다. 자세히 보기: [Allowed Tool Calls Overview](#tools-overview).
- **모든 행동에 대한 감사 필드.** 모델은 정당성과 신뢰도 점수를 포함해야 합니다. 두 항목 모두 실행 타임라인과 모든 승인 화면에 표시됩니다. 자세히 보기: [Run Detail View](#run-detail-view).
- **EU DSA Article 17.** EU 지역에서는 완전 자동화된 차단이 차단됩니다. 자세히 보기: [EU DSA Article 17 Compliance](#eu-dsa-compliance).
- **데이터에 대한 학습 없음.** FastComments는 여러분의 프롬프트나 댓글을 학습하지 않는 공급자를 사용합니다.

### Where they fit alongside human moderation

에이전트와 인간 중재자는 동일한 댓글 플랫폼을 공유합니다: 에이전트는 동일한 채널(승인, 스팸, 차단, 배지, 고정, 잠금, 작성)을 통해 행동을 취하고, 이러한 행동은 동일한 [Comment Logs](/guide-moderation.html#comment-logs), 동일한 [Moderation Page](/guide-moderation.html#moderate-comments-page) 및 동일한 알림 스트림에 나타납니다. 에이전트와 인간은 서로의 작업을 확인하고 상호 반응할 수 있으며, 중재자 행동 자체도 유효한 에이전트 트리거가 됩니다([Trigger: Moderator Reviewed Comment](#trigger-moderator-reviewed) 등 참고).

큐에 사람도 필요하다면, FastComments는 [Human Moderation Services](/guide-moderation.html#human-moderation-services)를 제공합니다: 우리 중재자는 여러분의 에이전트와 함께 대시보드에서 여러분의 가이드라인에 따라 작업합니다.
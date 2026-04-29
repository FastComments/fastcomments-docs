이 문서는 "우리에겐 AI 에이전트가 있다"에서 "승인으로 제어되는 실시간 트래픽에 에이전트가 응답한다"까지 가는 5분 경로입니다. 자세한 설명을 원하면 각 단계가 심층적으로 다루는 페이지로 연결된 링크를 확인하세요.

### 1. AI 에이전트 페이지 열기

계정에서 [AI Agents](https://fastcomments.com/auth/my-account/ai-agents)로 이동하세요. 처음 이 페이지에 들어오면 다음 중 하나가 표시됩니다:

- **Browse templates** 및 **Start from scratch** 버튼이 있는 빈 상태(에이전트를 생성할 수 있음), 또는
- 요금제에 에이전트가 포함되어 있지 않은 경우 업셀 페이지 - [Plans and Eligibility](#plans-and-eligibility)를 참조하세요.

### 2. 시작 템플릿 선택

**Browse templates**를 클릭하세요. 다음 중 하나를 선택하세요:

- [Moderator](#template-moderator) - 플래그된(신고된) 또는 신규 댓글을 검토하고, 처음 위반자에게 경고를 주며 경고 후에만 차단으로 에스컬레이션합니다.
- [Welcome Greeter](#template-welcome-greeter) - 첫 댓글 작성자에게 답글을 답니다.
- [Top Comment Pinner](#template-top-comment-pinner) - 투표 임계값을 넘는 의미 있는 댓글을 고정합니다.
- [Thread Summarizer](#template-thread-summarizer) - 긴 스레드에 중립적인 요약을 게시합니다.

각 템플릿은 **Status: Dry Run**이 이미 선택된 상태의 사전 입력된 편집 양식으로 이동합니다.

### 3. 검토 및 저장

편집 양식에서 최소한 다음을 설정하세요:

- **Internal name.** 관리자 대시보드에서 사용하는 짧은 식별자입니다.
- **Display name.** 에이전트가 댓글을 게시할 때 공개적으로 표시되는 이름입니다.
- **Initial prompt.** 템플릿의 프롬프트를 편집하여 귀하의 어조와 구체적인 규칙에 맞추세요.
- **Approvals.** 적용되기 전에 사람의 검토가 필요해야 하는 작업에 체크하세요. 어떤 중재 스타일 에이전트든 최소한 `ban_user`를 권장합니다. [Approval Workflow](#approval-workflow)를 참조하세요.

**에이전트 저장**을 클릭하세요.

### 4. 드라이런으로 관찰

이제 에이전트가 **드라이런** 상태로 활성화됩니다. 트리거를 수신하고 모델을 호출하며 [Run History](#run-history) 페이지에 각 행에 **Dry Run** 배지가 붙은 채로 실행 기록을 남기지만 실제 조치는 수행하지 않습니다. 몇몇 실행 상세(자세한 내용은 [Run Detail View](#run-detail-view) 참조)를 방문하여 다음을 확인하세요:

- 에이전트가 선택한 작업들.
- 각 작업에 대한 정당화와 신뢰도.
- 전체 LLM 대화 기록.

에이전트의 결정이 마음에 들지 않으면 초기 프롬프트를 편집하거나 더 많은 승인 항목에 체크하세요.

### 5. 과거 댓글에 대한 테스트 실행

에이전트 목록 페이지에서 해당 에이전트 행의 **Test run**을 클릭하세요. 양식에는 단일 **Days** 숫자 입력(1~90)이 있습니다. 표본 크기와 평가된 댓글에 대한 상한선은 정보로 표시되어 있으며 서버 측에서 계산되며 사용자가 설정하지 않습니다. 이 재실행은 과거 댓글을 대상으로 실제 조치를 취하지 않고 실행되며, 에이전트가 실제로는 어떻게 했는지(나중에 댓글이 승인되었는지, 스팸으로 표기되었는지, 삭제되었는지 등)와 비교하여 에이전트가 **했을 것**을 보고합니다. [Test Runs (Replays)](#test-runs-replays)를 참조하세요.

### 6. 활성화로 전환

드라이런 및 재실행 결과에 만족하면 에이전트를 편집하고 **Status**를 **Enabled**로 변경하세요. 이제부터 실제 조치가 적용됩니다. Run History 페이지는 이제 드라이런 배지 없이 라이브 실행을 표시하며, 승인 대상으로 표시한 모든 작업은 [approvals inbox](#approval-workflow)에 나타납니다.

### 다음 단계

- [Budgets](#budgets-overview) 및 [Budget Alerts](#budget-alerts)를 설정하세요.
- 에이전트 이벤트에 대해 외부 시스템이 반응하길 원하면 [Webhooks](#webhooks-overview)를 구성하세요.
- 에이전트 결정이 귀하의 문서화된 정책과 일치하도록 [Community Guidelines](#community-guidelines)를 추가하세요.
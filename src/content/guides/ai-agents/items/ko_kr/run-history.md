Run History는 각 에이전트별로 실행된 모든 트리거의 로그입니다. 에이전트 목록 페이지의 **Runs** 버튼을 통해 접근하거나, `/auth/my-account/ai-agents/{agentId}/runs`에서 직접 접근할 수 있습니다.

### What's on the page

한 실행마다 한 행으로 구성된 페이지네이션된 테이블:

| Column | Meaning |
|---|---|
| Date | 트리거가 실행된 시점(또는 연기된 트리거가 실행된 시점). |
| Status | **Started**, **Success**, 또는 **Error**. 실행이 dry-run 모드였던 경우 **Dry Run** 배지가 함께 표시됩니다. |
| Cost | 테넌트의 통화 기준 실행 당 비용. 진행 중(Started) 실행의 경우 비어 있음. |
| Actions | 해당 실행에서의 툴 호출 수. |
| Details | [Run Detail View](#run-detail-view)를 여는 **View** 버튼. |

### Status meanings

- **Started** - 실행이 진행 중이거나 완료되지 못하고 중단됨. 평소보다 오래 **Started** 상태에 머무르는 실행은 보통 LLM 호출의 타임아웃을 의미합니다.
- **Error** - 실행은 완료되었으나 실패함 - LLM 호출이 에러를 반환했거나, 툴 디스패치가 실패하는 등. 상세 보기에는 구체적인 에러가 포함됩니다.
- **Success** - 실행이 에러 없이 완료됨. 에이전트는 0번, 1번 또는 여러 번의 작업을 수행했을 수 있습니다.

### Empty state

에이전트에 실행이 없을 때, 페이지에는 다음 메시지가 표시됩니다: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

마지막 문장은 의도된 것입니다 - [test run flow](#test-runs-replays)는 신규 에이전트에서 Run History를 채우는 권장 방법입니다.

### What's not on the run history page

- **Live triggers that never dispatched** - 예산, 범위 또는 속도 제한 때문에 드롭된 트리거는 이 페이지에 표시되지 않습니다. 해당 항목들은 [Analytics page](#analytics-page)에서 "Triggers skipped" 항목으로 표시됩니다.
- **Approvals** - 이 실행에서 수행된 작업에 대한 대기 중인 승인 항목은 [approvals inbox](#approval-workflow)에 있습니다. 해당 작업은 실행 상세 보기에서 **Pending approval**로 표시됩니다.

### Retention

개별 실행 레코드는 90일 동안 보관되며, 그 이후에는 기록에서 사라집니다. 비용 및 트리거 수는 장기 분석 요약에 계속 집계되므로 [Analytics page](#analytics-page)에는 해당 기간 이후의 누적 합계가 계속 표시됩니다.

### Replays

리플레이로 생성된 실행은 기본적으로 라이브 실행 보기에서 제외됩니다. 해당 항목은 [Test Runs (Replays)](#test-runs-replays) 페이지에서 확인할 수 있습니다.

### Filtering across agents

실행 테이블은 에이전트별입니다. 에이전트 간 실행 통합 뷰는 제공되지 않으며, 에이전트 간 요약은 [Analytics page](#analytics-page)에 있습니다. 여러 에이전트에 걸친 실행을 검사해야 하는 경우, 자체 시스템으로 전달할 수 있는 [Webhooks](#webhooks-overview) 의 `trigger.succeeded` 및 `trigger.failed` 이벤트를 사용하세요.
실행 기록(Run History)은 실행된 모든 트리거에 대한 에이전트별 로그입니다. 에이전트 목록 페이지에서 **Runs** 버튼을 통해 접근하거나, `/auth/my-account/ai-agents/{agentId}/runs`에서 직접 접근할 수 있습니다.

### 페이지 구성

실행당 한 행씩 표시되는 페이지네이션된 테이블입니다:

| 컬럼 | 의미 |
|---|---|
| Date | 트리거가 발동된 시점(또는 지연된 트리거가 실행된 시점). |
| Status | **Started**, **Success**, 또는 **Error**. 실행이 드라이런 모드였던 경우 **Dry Run** 배지가 함께 표시됩니다. |
| Cost | 테넌트 통화 기준의 실행당 비용. 진행 중(Started)인 실행의 경우 비어 있습니다. |
| Actions | 해당 실행에서의 도구 호출 횟수. |
| Details | [실행 상세 보기](#run-detail-view)를 여는 **View** 버튼. |

### 상태 의미

- **Started** - 실행이 진행 중이거나, 완료 전에 중단되었습니다. 비정상적으로 오랫동안 "Started" 상태로 머물러 있는 실행은 보통 LLM 호출 타임아웃을 의미합니다.
- **Error** - 실행은 완료되었지만 어딘가에서 실패했습니다 - LLM 호출이 오류를 반환했거나, 도구 디스패치가 실패하는 등의 경우입니다. 상세 보기에 구체적인 오류가 표시됩니다.
- **Success** - 실행이 오류 없이 완료되었습니다. 에이전트는 0개, 1개 또는 여러 개의 작업을 수행했을 수 있습니다.

### 빈 상태

에이전트에 실행 기록이 없는 경우, 페이지에 다음과 같이 표시됩니다: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

마지막 부분은 의도된 것입니다 - [테스트 실행 플로우](#test-runs-replays)가 새 에이전트의 실행 기록을 채우는 데 권장되는 방법입니다.

### 실행 기록 페이지에 없는 것들

- **디스패치되지 않은 라이브 트리거** - 예산, 범위 또는 속도 제한으로 인해 폐기된 트리거는 이 페이지에 표시되지 않습니다. 이러한 항목은 [Analytics 페이지](#analytics-page)의 "Triggers skipped" 아래에 표시됩니다.
- **승인** - 이 실행에서 수행된 작업에 대한 대기 중인 승인은 [승인 수신함](#approval-workflow)에 있습니다. 해당 작업은 실행 상세 보기에 **Pending approval**로 표시됩니다.

### 보존 기간

개별 실행 기록은 90일 동안 보존되며, 이후에는 기록에서 사라집니다. 비용과 트리거 횟수는 장기 분석 요약에 계속 집계되므로, [Analytics 페이지](#analytics-page)에는 해당 기간 이후의 누적 합계도 계속 표시됩니다.

### 리플레이

리플레이로 생성된 실행은 기본적으로 라이브 실행 보기에서 제외됩니다. 이러한 항목은 [테스트 실행(리플레이)](#test-runs-replays) 페이지에서 확인할 수 있습니다.

### 에이전트 간 필터링

실행 테이블은 에이전트별로 제공됩니다. 에이전트 간 실행 보기는 없으며, [Analytics 페이지](#analytics-page)가 에이전트 간 요약을 제공합니다. 여러 에이전트에 걸친 실행을 검토해야 하는 경우, [웹훅](#webhooks-overview)의 `trigger.succeeded` 및 `trigger.failed` 이벤트를 자체 시스템으로 전달하는 방법을 사용할 수 있습니다.
There are four agent webhook event types. Each event has a numeric enum value (used in payloads) and a canonical string name (used in the `event` envelope field and in the `X-FastComments-Agent-Event` HTTP header).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | 에이전트 실행이 `SUCCESS` 상태로 완료될 때. |
| `trigger.failed` | 1 | 에이전트 실행이 `ERROR` 상태로 완료될 때. |
| `approval.requested` | 2 | 승인이 `PENDING` 상태로 대기열에 들어갈 때. |
| `approval.decided` | 3 | 승인이 `APPROVED`, `REJECTED`, 또는 `EXECUTION_FAILED` 로 전환될 때. |

### `trigger.succeeded`

Fires after the agent's run finishes without error. The payload's `data` field includes:

- `triggerId` - 고유 실행 ID.
- `triggerType` - 실행을 시작한 [trigger reason enum](#triggers-overview).
- `status` - `SUCCESS` (문자열).
- `tokensUsed` - 이번 실행에서 사용된 토큰.
- `wasDryRun` - 에이전트가 [dry-run mode](#dry-run-mode)인 경우 true.
- `actions` - `TenantAgentAction` 레코드 배열 (see [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`,`urlId` - 트리거에 해당 항목이 있는 경우.

If the run took zero actions, the `actions` array is empty - this is a successful "the agent decided to do nothing" run, which is useful to know.

### `trigger.failed`

Fires when a run errors. Same payload shape as `trigger.succeeded`, with `status: 'ERROR'` and an additional `errorMessage` field describing what went wrong. Possible errors include LLM call failures, tool dispatch failures, and budget exhaustion mid-run.

`actions` may still contain entries for tool calls that completed before the error.

### `approval.requested`

Fires the moment an approval is queued in `PENDING` state. Payload includes:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - LLM 호출에서 **그대로 전달된** 도구 인수. 형태는 도구마다 다르며 안정적인 공개 계약이 아니므로 새로운 도구가 추가될 때 스키마가 변경될 수 있습니다.
- `createdAt`.
- `justification`, `confidence` - 에이전트가 제공한 경우.
- `contextSnapshot` - 승인이 관련된 댓글/페이지 컨텍스트.

Useful for forwarding pending approvals into a chat ops channel: a Slack bot subscribed to `approval.requested` can post the action and reasoning into a moderation channel for at-a-glance review.

### `approval.decided`

Fires when an approval moves out of `PENDING`. Payload includes:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`.
- `decidedBy` - 결정을 내린 중재자의 사용자 ID.
- `decidedAt` - 결정을 내린 시점.
- `executedAt` - APPROVED인 경우, 플랫폼이 승인된 작업을 실행한 시점.
- `executionResult` - APPROVED인 경우, 실행자의 결과를 설명하는 문자열.
- `contextSnapshot` - 댓글/페이지 컨텍스트.

This event covers all decision outcomes:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` 설정, `executionResult`는 성공 메시지입니다.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` 설정, `executionResult`는 실패를 설명합니다.
- **Rejected** -> `status: REJECTED`, `executedAt`는 null, `executionResult`는 null.

### Header

Every delivery includes an `X-FastComments-Agent-Event` HTTP header with the event's canonical string name (`trigger.succeeded`, etc.). Useful if your endpoint is a single URL handling multiple event types.

### See also

- [Webhook Payloads](#webhook-payloads) - 각 이벤트별 전체 페이로드 스키마.
- [Webhook Signing](#webhook-signing) - HMAC 방식.
- [Webhook Retries](#webhook-retries) - 전달 의미론.
There are four agent webhook event types. Each event has a numeric enum value (used in payloads) and a canonical string name (used in the `event` envelope field and in the `X-FastComments-Agent-Event` HTTP header).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | An agent run completes with status `SUCCESS`. |
| `trigger.failed` | 1 | An agent run completes with status `ERROR`. |
| `approval.requested` | 2 | An approval is queued in `PENDING` state. |
| `approval.decided` | 3 | An approval transitions to `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`. |

### `trigger.succeeded`

Fires after the agent's run finishes without error. The payload's `data` field includes:

- `triggerId` - the unique run ID.
- `triggerType` - the [trigger reason enum](#triggers-overview) that started the run.
- `status` - `SUCCESS` (string).
- `tokensUsed` - tokens consumed in this run.
- `wasDryRun` - true if the agent was in [dry-run mode](#dry-run-mode).
- `actions` - array of `TenantAgentAction` records (see [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - if the trigger had them.

If the run took zero actions, the `actions` array is empty - this is a successful "the agent decided to do nothing" run, which is useful to know.

### `trigger.failed`

Fires when a run errors. Same payload shape as `trigger.succeeded`, with `status: 'ERROR'` and an additional `errorMessage` field describing what went wrong. Possible errors include LLM call failures, tool dispatch failures, and budget exhaustion mid-run.

`actions` may still contain entries for tool calls that completed before the error.

### `approval.requested`

Fires the moment an approval is queued in `PENDING` state. Payload includes:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - the tool's arguments **passed through verbatim** from the LLM call. The shape is per-tool and not a stable public contract - the schema can change as new tools are added.
- `createdAt`.
- `justification`, `confidence` - if the agent supplied them.
- `contextSnapshot` - the comment / page context the approval relates to.

Useful for forwarding pending approvals into a chat ops channel: a Slack bot subscribed to `approval.requested` can post the action and reasoning into a moderation channel for at-a-glance review.

### `approval.decided`

Fires when an approval moves out of `PENDING`. Payload includes:

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`.
- `decidedBy` - the user ID of the moderator who decided.
- `decidedAt` - when they decided.
- `executedAt` - if APPROVED, when the platform executed the approved action.
- `executionResult` - if APPROVED, a string describing the executor's result.
- `contextSnapshot` - the comment / page context.

This event covers all decision outcomes:

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` set, `executionResult` is the success message.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` set, `executionResult` describes the failure.
- **Rejected** -> `status: REJECTED`, `executedAt` is null, `executionResult` is null.

### Header

Every delivery includes an `X-FastComments-Agent-Event` HTTP header with the event's canonical string name (`trigger.succeeded`, etc.). Useful if your endpoint is a single URL handling multiple event types.

### See also

- [Webhook Payloads](#webhook-payloads) for full per-event payload schemas.
- [Webhook Signing](#webhook-signing) for the HMAC scheme.
- [Webhook Retries](#webhook-retries) for delivery semantics.

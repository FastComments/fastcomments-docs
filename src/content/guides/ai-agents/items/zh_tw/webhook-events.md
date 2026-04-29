有四種 agent webhook 事件類型。每個事件都有一個數字 enum 值（用於 payloads）和一個標準字串名稱（用於 `event` 封裝欄位以及 `X-FastComments-Agent-Event` HTTP 標頭）。

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | An agent run completes with status `SUCCESS`. |
| `trigger.failed` | 1 | An agent run completes with status `ERROR`. |
| `approval.requested` | 2 | An approval is queued in `PENDING` state. |
| `approval.decided` | 3 | An approval transitions to `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`. |

### `trigger.succeeded`

當 agent 的執行在沒有錯誤的情況下結束時觸發。payload 的 `data` 欄位包含：

- `triggerId` - 唯一的執行 ID。
- `triggerType` - 啟動該執行的 [trigger reason enum](#triggers-overview)。
- `status` - `SUCCESS`（字串）。
- `tokensUsed` - 此次執行消耗的 tokens。
- `wasDryRun` - 如果 agent 處於[dry-run 模式](#dry-run-mode)則為 true。
- `actions` - `TenantAgentAction` 紀錄的陣列（見 [Webhook Payloads](#webhook-payloads)）。
- `commentId`, `url`, `urlId` - 如果該 trigger 帶有這些資訊。

如果執行沒有任何 actions，`actions` 陣列為空 —— 這表示成功的「agent 決定不做任何事」執行，這本身就是有用的資訊。

### `trigger.failed`

當執行發生錯誤時觸發。payload 結構與 `trigger.succeeded` 相同，但 `status: 'ERROR'` 且額外包含一個描述錯誤原因的 `errorMessage` 欄位。可能的錯誤包括 LLM 呼叫失敗、工具分派失敗，以及執行中途耗盡配額等情況。

`actions` 仍可能包含在錯誤發生前已完成的工具呼叫紀錄。

### `approval.requested`

當核准被排入 `PENDING` 狀態的那一刻觸發。payload 包含：

- `approvalId`, `triggerId`。
- `toolName`, `actionType`。
- `status: 'PENDING'`。
- `args` - 工具的參數，**會原封不動地**從 LLM 呼叫傳遞過來。參數結構依工具而異，並非穩定的公開契約 —— 隨著新工具的加入，架構可能改變。
- `createdAt`。
- `justification`, `confidence` - 如果 agent 提供會有此欄位。
- `contextSnapshot` - 與該核准相關的評論 / 頁面上下文。

此事件適合將待處理的核准轉送到 chat ops 頻道：訂閱 `approval.requested` 的 Slack 機器人可以把該動作與推理貼到審核頻道，方便快速檢視。

### `approval.decided`

當核准脫離 `PENDING` 時觸發。payload 包含：

- `approvalId`, `triggerId`。
- `toolName`, `actionType`。
- `status` - `APPROVED`, `REJECTED`, 或 `EXECUTION_FAILED`。
- `decidedBy` - 做出決定的審核者使用者 ID。
- `decidedAt` - 做出決定的時間。
- `executedAt` - 若為 APPROVED，平台執行核准動作的時間。
- `executionResult` - 若為 APPROVED，描述執行者結果的字串。
- `contextSnapshot` - 評論 / 頁面上下文。

此事件涵蓋所有決策結果：

- **已核准並成功執行** -> `status: APPROVED`，`executedAt` 已設定，`executionResult` 為成功訊息。
- **已核准但執行者失敗** -> `status: EXECUTION_FAILED`，`executedAt` 已設定，`executionResult` 描述失敗原因。
- **已拒絕** -> `status: REJECTED`，`executedAt` 為 null，`executionResult` 為 null。

### Header

每次送達都會包含一個 `X-FastComments-Agent-Event` HTTP 標頭，裡面放事件的標準字串名稱（`trigger.succeeded` 等）。當你的端點為一個同時處理多種事件類型的單一 URL 時，這個標頭很有用。

### See also

- [Webhook Payloads](#webhook-payloads) for full per-event payload schemas.
- [Webhook Signing](#webhook-signing) for the HMAC scheme.
- [Webhook Retries](#webhook-retries) for delivery semantics.
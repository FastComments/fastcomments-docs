共有四種 agent webhook 事件類型。每個事件都有一個數值列舉值(用於 payload 中)和一個標準字串名稱(用於 `event` 信封欄位以及 `X-FastComments-Agent-Event` HTTP 標頭中)。

| 事件名稱 | 列舉值 | 觸發時機 |
|---|---|---|
| `trigger.succeeded` | 0 | 一次 agent 執行以 `SUCCESS` 狀態完成時。 |
| `trigger.failed` | 1 | 一次 agent 執行以 `ERROR` 狀態完成時。 |
| `approval.requested` | 2 | 一個審核請求被排入 `PENDING` 狀態時。 |
| `approval.decided` | 3 | 一個審核請求轉換為 `APPROVED`、`REJECTED` 或 `EXECUTION_FAILED` 時。 |

### `trigger.succeeded`

在 agent 的執行無錯誤完成後觸發。payload 的 `data` 欄位包含:

- `triggerId` - 唯一的執行 ID。
- `triggerType` - 啟動此次執行的[觸發原因列舉](#triggers-overview)。
- `status` - `SUCCESS`(字串)。
- `tokensUsed` - 此次執行消耗的 token 數。
- `wasDryRun` - 如果 agent 處於[dry-run 模式](#dry-run-mode)則為 true。
- `actions` - `TenantAgentAction` 記錄陣列(參見 [Webhook Payloads](#webhook-payloads))。
- `commentId`、`url`、`urlId` - 如果觸發來源包含這些資訊。

如果執行過程沒有採取任何動作,`actions` 陣列會是空的 - 這是一次成功的「agent 決定不做任何事」的執行,知道這一點很有用。

### `trigger.failed`

在執行發生錯誤時觸發。payload 結構與 `trigger.succeeded` 相同,但 `status: 'ERROR'` 並額外包含一個描述錯誤原因的 `errorMessage` 欄位。可能的錯誤包括 LLM 呼叫失敗、工具分發失敗,以及執行中途預算耗盡。

`actions` 仍可能包含在錯誤發生前已完成的工具呼叫記錄。

### `approval.requested`

在審核請求被排入 `PENDING` 狀態的那一刻觸發。payload 包含:

- `approvalId`、`triggerId`。
- `toolName`、`actionType`。
- `status: 'PENDING'`。
- `args` - 從 LLM 呼叫**原封不動傳遞**的工具參數。其結構因工具而異,並非穩定的公開契約 - 隨著新工具的新增,schema 可能會改變。
- `createdAt`。
- `justification`、`confidence` - 如果 agent 有提供這些資訊。
- `contextSnapshot` - 該審核請求相關的留言 / 頁面上下文。

適合用於將待審核的請求轉發到 chat ops 頻道:訂閱 `approval.requested` 的 Slack 機器人可以將動作與理由發布到審核頻道,以便一目了然地審查。

### `approval.decided`

在審核請求離開 `PENDING` 狀態時觸發。payload 包含:

- `approvalId`、`triggerId`。
- `toolName`、`actionType`。
- `status` - `APPROVED`、`REJECTED` 或 `EXECUTION_FAILED`。
- `decidedBy` - 做出決定的審核者使用者 ID。
- `decidedAt` - 決定的時間。
- `executedAt` - 如果為 APPROVED,則為平台執行該核准動作的時間。
- `executionResult` - 如果為 APPROVED,則為描述執行結果的字串。
- `contextSnapshot` - 留言 / 頁面上下文。

此事件涵蓋所有決定結果:

- **核准且順利執行** -> `status: APPROVED`,`executedAt` 已設定,`executionResult` 為成功訊息。
- **核准但執行器失敗** -> `status: EXECUTION_FAILED`,`executedAt` 已設定,`executionResult` 描述失敗原因。
- **拒絕** -> `status: REJECTED`,`executedAt` 為 null,`executionResult` 為 null。

### Header

每次傳遞都包含一個帶有事件標準字串名稱(`trigger.succeeded` 等)的 `X-FastComments-Agent-Event` HTTP 標頭。如果您的端點是處理多種事件類型的單一 URL,這會很有用。

### See also

- [Webhook Payloads](#webhook-payloads) - 每個事件的完整 payload schema。
- [Webhook Signing](#webhook-signing) - HMAC 簽章機制。
- [Webhook Retries](#webhook-retries) - 傳遞語義。
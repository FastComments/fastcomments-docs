所有 Agent webhook 負載都共用一個共同的信封結構，並附加一個事件專屬的 `data` 區塊。本頁面列出每個事件的完整 schema。

### 信封結構（每個事件）

無論事件類型為何，每個負載都具有以下頂層欄位：

[inline-code-attrs-start title = 'Webhook 信封 Schema'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - the matched domain for this delivery",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 timestamp",
  "data": { /* event-specific, see below */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema：

[inline-code-attrs-start title = '觸發事件資料 Schema'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "triggerId": "string",
  "triggerType": 0,
  "status": "SUCCESS | ERROR",
  "tokensUsed": 1234,
  "wasDryRun": false,
  "actions": [
    {
      "type": 0,
      "commentId": "string - optional",
      "userId": "string - optional",
      "badgeId": "string - optional",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - present on trigger.failed",
  "url": "string - optional",
  "urlId": "string - optional",
  "commentId": "string - optional"
}
[inline-code-end]

`triggerType` 是來自[觸發事件列表](#triggers-overview)的數字列舉值。

`actions[].type` 是來自[工具列表](#tools-overview)的數字列舉值。

當動作被排入[審核](#approval-workflow)佇列而非直接執行時，`actions[].pending` 為 `true`。

### `approval.requested`

`data` schema：

[inline-code-attrs-start title = '審核請求資料 Schema'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* per-tool, see below */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - optional, agent reasoning",
  "confidence": 0.85,
  "contextSnapshot": { /* the comment/page context the approval is about */ }
}
[inline-code-end]

**`args`** 物件是 LLM 工具呼叫所攜帶的內容。其結構取決於工具：

- 對於 `ban_user`：`{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`。
- 對於 `mark_comment_spam`：`{ commentId, isSpam }`。
- 對於 `write_comment`：`{ comment, urlId, parentId? }`。
- ...依此類推。

工具參數的結構集合**並非穩定的公開契約**。未來可能會新增工具，且平台會原封不動地傳遞參數。除非消費者明確了解所涉及的工具，否則應將 args 視為不透明的資料區塊。

**`contextSnapshot`** 擷取審核排入佇列時的留言、頁面與使用者情境。其結構與觸發事件的 context message 相同。

### `approval.decided`

`data` schema：

[inline-code-attrs-start title = '審核決定資料 Schema'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - the userId of the moderator who decided",
  "decidedAt": "string - ISO 8601 - optional, only present once decided",
  "executedAt": "string - ISO 8601 - present when APPROVED and execution finished",
  "executionResult": "string - executor result message - present after execute",
  "contextSnapshot": { /* same as approval.requested */ }
}
[inline-code-end]

### TenantAgentAction 結構

在觸發負載的 `actions[]` 中，每個動作都具有：

[inline-code-attrs-start title = 'TenantAgentAction Schema'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - optional",
  "userId": "string - optional",
  "badgeId": "string - optional",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

`type` 列舉值對應 `AgentActionType`：

- 0: `WRITE_COMMENT`
- 1: `VOTE_COMMENT`
- 2: `PIN_COMMENT`
- 3: `UNPIN_COMMENT`
- 4: `LOCK_COMMENT`
- 5: `UNLOCK_COMMENT`
- 6: `MARK_COMMENT_REVIEWED`
- 7: `MARK_COMMENT_APPROVED`
- 8: `MARK_COMMENT_SPAM`
- 9: `AWARDED_BADGE`
- 10: `BAN_USER`
- 11: `SENT_EMAIL`
- 12: `WARNED_USER`
- 13: `SAVED_MEMORY`

`SEARCH_MEMORY` 不會出現在 `actions[]` 中，因為它是唯讀且未經稽核的。

### `triggerType` 列舉值

`AgentTriggerReasonType`：

- 0: `COMMENT_ADD`
- 1: `COMMENT_EDIT`
- 2: `COMMENT_DELETE`
- 3: `COMMENT_PIN`
- 4: `COMMENT_UNPIN`
- 5: `COMMENT_LOCK`
- 6: `COMMENT_UNLOCK`
- 7: `COMMENT_VOTE_THRESHOLD`
- 8: `MODERATOR_REVIEWED_COMMENT`
- 9: `MODERATOR_APPROVED_COMMENT`
- 10: `MODERATOR_SPAMMED_COMMENT`
- 11: `MODERATOR_AWARDED_BADGE`
- 12: `COMMENT_FLAG_THRESHOLD`
- 13: `NEW_USER_FIRST_COMMENT`
- 14: `COMMENT_AUTO_SPAMMED`
- 15: `REPLAY`（內部使用；不會傳送至 webhook）

### 標頭

每次傳送都包含：

- `X-FastComments-Agent-Event` - 標準事件名稱（`trigger.succeeded` 等）。
- `X-FastComments-Signature` - 使用您的 API secret 對原始 body 計算的 HMAC-SHA256。請參閱 [Webhook 簽署](#webhook-signing)。

### 穩定性

信封欄位以及每個事件所記載的 `data` 欄位皆屬於公開契約的一部分。在現有負載中新增選填欄位是允許的，且不視為破壞性變更 - 您的消費者應忽略未知欄位。`args` 與 `contextSnapshot` 的結構**不屬於**契約的一部分。
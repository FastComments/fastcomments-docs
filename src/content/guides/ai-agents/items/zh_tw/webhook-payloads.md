所有代理程式 webhook 載荷共享一個共同的信封，並加入事件特定的 `data` 區塊。本頁列出每個事件的完整結構。

### Envelope (every event)

每個載荷，不論事件類型，都具有以下頂層欄位：

[inline-code-attrs-start title = 'Webhook 信封結構'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - 此次傳遞匹配的網域",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 時間戳記",
  "data": { /* 事件特定，見下文 */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema:

[inline-code-attrs-start title = '觸發事件資料結構'; type='json' inline-code-attrs-end]
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
      "commentId": "string - 可選",
      "userId": "string - 可選",
      "badgeId": "string - 可選",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - 在 trigger.failed 時存在",
  "url": "string - 可選",
  "urlId": "string - 可選",
  "commentId": "string - 可選"
}
[inline-code-end]

`triggerType` 是來自 [觸發事件清單](#triggers-overview) 的數值列舉。

`actions[].type` 是來自 [工具清單](#tools-overview) 的數值列舉。

`actions[].pending` 為 `true` 當該操作被排入等待 [審核工作流程](#approval-workflow) 而非執行時。

### `approval.requested`

`data` schema:

[inline-code-attrs-start title = '請求審核資料結構'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* 每個工具不同，見下文 */ },
  "createdAt": "string - ISO 8601 時間戳記",
  "justification": "string - 可選，代理推理",
  "confidence": 0.85,
  "contextSnapshot": { /* 申請審核所涉及的評論/頁面上下文 */ }
}
[inline-code-end]

The **`args`** object is whatever the LLM tool call carried. Its shape depends on the tool:

- 對於 `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- 對於 `mark_comment_spam`: `{ commentId, isSpam }`.
- 對於 `write_comment`: `{ comment, urlId, parentId? }`.
- ...等等。

工具參數形狀集合並非一個穩定的公開合約。未來可能會新增工具，平台會原封不動地傳遞 args。消費者應將 args 視為不透明的資料塊，除非他們明確瞭解相關工具。

The **`contextSnapshot`** captures the comment, page, and user context the approval was queued from. Its shape mirrors the trigger's context message.

### `approval.decided`

`data` schema:

[inline-code-attrs-start title = '審核決定資料結構'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - 做決定的管理員的 userId",
  "decidedAt": "string - ISO 8601 - 可選，僅在已決定後存在",
  "executedAt": "string - ISO 8601 - 在 APPROVED 且執行完成時存在",
  "executionResult": "string - 執行者結果訊息 - 執行後存在",
  "contextSnapshot": { /* 與 approval.requested 相同 */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = '租戶代理動作結構'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - 可選",
  "userId": "string - 可選",
  "badgeId": "string - 可選",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

`type` enum values match `AgentActionType`:

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

`SEARCH_MEMORY` does not appear in `actions[]` because it is read-only and unaudited.

### `triggerType` enum values

`AgentTriggerReasonType`:

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
- 15: `REPLAY` (內部使用；不會傳送到 webhook)

### Headers

Every delivery includes:

- `X-FastComments-Agent-Event` - 標準事件名稱 (`trigger.succeeded`, 等)。
- `X-FastComments-Signature` - 對原始主體使用您的 API 密鑰進行 HMAC-SHA256。請參閱 [Webhook 簽名](#webhook-signing)。

### Stability

The envelope fields and the documented `data` fields per event are part of the public contract. Adding new optional fields to existing payloads is allowed and not considered a breaking change - your consumer should ignore unknown fields. The shape of `args` and `contextSnapshot` is **not** part of the contract.

---
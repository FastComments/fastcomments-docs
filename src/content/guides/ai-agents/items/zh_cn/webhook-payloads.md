所有代理 webhook 有共同的信封，并添加事件特定的 `data` 块。本页列出每种事件的完整模式。

### Envelope（每个事件）

每个负载（无论事件类型）都具有这些顶级字段：

[inline-code-attrs-start title = 'Webhook 信封架构'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - 与此传递匹配的域名",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 时间戳",
  "data": { /* 事件特定，见下文 */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` 模式：

[inline-code-attrs-start title = '触发事件数据架构'; type='json' inline-code-attrs-end]
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
      "commentId": "string - 可选",
      "userId": "string - 可选",
      "badgeId": "string - 可选",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - 在 trigger.failed 时存在",
  "url": "string - 可选",
  "urlId": "string - 可选",
  "commentId": "string - 可选"
}
[inline-code-end]

`triggerType` 是来自 [触发事件列表](#triggers-overview) 的数字枚举。

`actions[].type` 是来自 [工具列表](#tools-overview) 的数字枚举。

`actions[].pending` 在该操作被排队等待 [审批](#approval-workflow) 而非执行时为 `true`。

### `approval.requested`

`data` 模式：

[inline-code-attrs-start title = '请求审批数据架构'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* 每个工具不同，见下文 */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - 可选，代理推理",
  "confidence": 0.85,
  "contextSnapshot": { /* 关于该审批的评论/页面上下文 */ }
}
[inline-code-end]

**`args`** 对象包含 LLM 工具调用携带的内容。其结构取决于具体工具：

- For `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- For `mark_comment_spam`: `{ commentId, isSpam }`.
- For `write_comment`: `{ comment, urlId, parentId? }`.
- ...and so on.

工具参数结构集合并非稳定的公共契约。将来可以添加工具，平台会按原样传递 args。使用方应将 args 视为不透明的二进制块，除非明确理解所涉及的工具。

**`contextSnapshot`** 捕获了发起该审批的评论、页面和用户上下文。其结构与触发器的上下文消息相同。

### `approval.decided`

`data` 模式：

[inline-code-attrs-start title = '审批决定数据架构'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - 做出决定的版主的 userId",
  "decidedAt": "string - ISO 8601 - 可选，仅在已决定后存在",
  "executedAt": "string - ISO 8601 - 在 APPROVED 且执行完成时存在",
  "executionResult": "string - 执行者结果消息 - 在执行后存在",
  "contextSnapshot": { /* 与 approval.requested 相同 */ }
}
[inline-code-end]

### TenantAgentAction 结构

在触发器负载的 `actions[]` 中，每个操作具有：

[inline-code-attrs-start title = '租户代理操作架构'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - 可选",
  "userId": "string - 可选",
  "badgeId": "string - 可选",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

`type` 枚举值对应 `AgentActionType`：

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

`SEARCH_MEMORY` 不会出现在 `actions[]` 中，因为它是只读且不受审计的。

### `triggerType` 枚举值

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
- 15: `REPLAY` (内部；不投递到 webhooks)

### Headers

每次投递都会包含：

- `X-FastComments-Agent-Event` - 规范事件名称（`trigger.succeeded` 等）。
- `X-FastComments-Signature` - 使用您的 API 密钥对原始正文进行 HMAC-SHA256。参见 [Webhook 签名](#webhook-signing)。

### 稳定性

信封字段和每个事件文档中记录的 `data` 字段是公共契约的一部分。向现有负载添加新的可选字段是允许的，不被视为破坏性更改——您的消费者应忽略未知字段。`args` 和 `contextSnapshot` 的结构并不属于该契约。

---
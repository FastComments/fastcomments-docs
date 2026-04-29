すべてのエージェント webhook ペイロードは共通のエンベロープを共有し、イベント固有の `data` ブロックを追加します。このページでは各イベントの完全なスキーマを示します。

### エンベロープ（すべてのイベント）

イベントの種類に関係なく、すべてのペイロードは次のトップレベルフィールドを持ちます:

[inline-code-attrs-start title = 'Webhook エンベロープのスキーマ'; type='json' inline-code-attrs-end]
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

`data` スキーマ:

[inline-code-attrs-start title = 'トリガーイベントのデータスキーマ'; type='json' inline-code-attrs-end]
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

`triggerType` は [trigger event list](#triggers-overview) の数値列挙型です。

`actions[].type` は [tool list](#tools-overview) の数値列挙型です。

`actions[].pending` は、アクションが実行されたのではなく [approval](#approval-workflow) のためにキューに入れられた場合に `true` になります。

### `approval.requested`

`data` スキーマ:

[inline-code-attrs-start title = '承認要求のデータスキーマ'; type='json' inline-code-attrs-end]
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

**`args`** オブジェクトは LLM ツール呼び出しが持っていたものそのものです。その形状はツールによって異なります:

- `ban_user` の場合: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- `mark_comment_spam` の場合: `{ commentId, isSpam }`.
- `write_comment` の場合: `{ comment, urlId, parentId? }`.
- ...など。

ツールの引数形状の集合は **公開された安定した契約ではありません**。将来的にツールが追加される可能性があり、プラットフォームは args をそのまま渡します。利用側は、関与するツールを明示的に理解している場合を除き、args を不透明なデータとして扱うべきです。

**`contextSnapshot`** は、承認がキューに入れられたコメント、ページ、ユーザーのコンテキストをキャプチャします。その形状はトリガーのコンテキストメッセージと一致します。

### `approval.decided`

`data` スキーマ:

[inline-code-attrs-start title = '承認決定のデータスキーマ'; type='json' inline-code-attrs-end]
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

### TenantAgentAction の形

トリガーペイロードの `actions[]` 内で、各アクションは次のようになります:

[inline-code-attrs-start title = 'TenantAgentAction のスキーマ'; type='json' inline-code-attrs-end]
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

`type` の列挙値は `AgentActionType` と一致します:

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

`SEARCH_MEMORY` は読み取り専用で監査対象外のため `actions[]` には現れません。

### `triggerType` 列挙値

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
- 15: `REPLAY` (内部用; webhooks には配信されません)

### ヘッダー

すべての配信には次が含まれます:

- `X-FastComments-Agent-Event` - 正式なイベント名（`trigger.succeeded` など）。
- `X-FastComments-Signature` - API シークレットを使用して生のボディに対して計算した HMAC-SHA256。詳細は [Webhook Signing](#webhook-signing) を参照してください。

### 安定性

エンベロープのフィールドおよび各イベントに文書化された `data` フィールドは公開契約の一部です。既存のペイロードに対して新しいオプションフィールドを追加することは許容され、破壊的変更とは見なされません — 利用側は不明なフィールドを無視するべきです。`args` および `contextSnapshot` の形状は契約の一部ではありません。
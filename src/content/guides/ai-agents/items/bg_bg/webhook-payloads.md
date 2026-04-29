Всички полезни натоварвания на агентските webhook-и споделят обща обвивка и добавят специфичен за събитието блок `data`. Тази страница изброява пълната схема за всеки.

### Обвивка (всяко събитие)

Всяко полезно натоварване, независимо от типа на събитието, има следните основни полета:

[inline-code-attrs-start title = 'Схема на обвивката на Webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - съответстващият домейн за тази доставка",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 времеви печат",
  "data": { /* специфично за събитието, вижте по-долу */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema:

[inline-code-attrs-start title = 'Схема на данните за тригерното събитие'; type='json' inline-code-attrs-end]
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
      "commentId": "string - незадължително",
      "userId": "string - незадължително",
      "badgeId": "string - незадължително",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - присъства при trigger.failed",
  "url": "string - незадължително",
  "urlId": "string - незадължително",
  "commentId": "string - незадължително"
}
[inline-code-end]

`triggerType` is a numeric enum from the [списък с trigger събития](#triggers-overview).

`actions[].type` is a numeric enum from the [списъка с инструменти](#tools-overview).

`actions[].pending` е `true` когато действието е поставено в опашка за [одобрение](#approval-workflow) вместо да бъде изпълнено.

### `approval.requested`

`data` schema:

[inline-code-attrs-start title = 'Схема на данните за заявено одобрение'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* зависи от инструмента, вижте по-долу */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - незадължително, мотивировка на агента",
  "confidence": 0.85,
  "contextSnapshot": { /* контекстът на коментара/страницата, относно който е заявката за одобрение */ }
}
[inline-code-end]

The **`args`** object is whatever the LLM tool call carried. Its shape depends on the tool:

- For `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- For `mark_comment_spam`: `{ commentId, isSpam }`.
- For `write_comment`: `{ comment, urlId, parentId? }`.
- ...и така нататък.

The set of tool argument shapes is **not a stable public contract**. Tools can be added in future and the platform passes args through verbatim. Consumers should treat args as an opaque blob unless they explicitly understand the tool involved.

The **`contextSnapshot`** captures the comment, page, and user context the approval was queued from. Its shape mirrors the trigger's context message.

### `approval.decided`

`data` schema:

[inline-code-attrs-start title = 'Схема на данните при решение за одобрение'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - userId на модератора, който взе решението",
  "decidedAt": "string - ISO 8601 - незадължително, налично само след като е взето решение",
  "executedAt": "string - ISO 8601 - налично когато е APPROVED и изпълнението е завършено",
  "executionResult": "string - съобщение с резултата от изпълнителя - налично след изпълнение",
  "contextSnapshot": { /* същото като approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'Схема на TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - незадължително",
  "userId": "string - незадължително",
  "badgeId": "string - незадължително",
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
- 15: `REPLAY` (вътрешно; не се доставя до webhook-и)

### Заглавки

Всяка доставка включва:

- `X-FastComments-Agent-Event` - каноничното име на събитието (`trigger.succeeded`, и т.н.).
- `X-FastComments-Signature` - HMAC-SHA256 на суровото тяло, използвайки вашата API тайна. Вижте [Подписване на Webhook](#webhook-signing).

### Стабилност

Полетата в обвивката и документираните полета `data` за всяко събитие са част от публичния контракт. Добавянето на нови незадължителни полета към съществуващите полезни натоварвания е позволено и не се счита за нарушаваща промяна — получателят трябва да игнорира неизвестните полета. Формата на `args` и `contextSnapshot` **не** е част от контракта.

---
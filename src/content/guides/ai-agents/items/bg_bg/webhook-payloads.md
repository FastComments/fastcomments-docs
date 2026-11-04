Всички webhook полезни товари на агентите споделят общ плик (envelope) и добавят блок `data`, специфичен за събитието. Тази страница изброява пълната схема за всеки.

### Плик (всяко събитие)

Всеки полезен товар, независимо от типа събитие, има тези полета на най-високо ниво:

[inline-code-attrs-start title = 'Схема на webhook плика'; type='json' inline-code-attrs-end]
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

Схема на `data`:

[inline-code-attrs-start title = 'Схема на данните за събитието trigger'; type='json' inline-code-attrs-end]
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

`triggerType` е числов enum от [списъка на събитията-тригери](#triggers-overview).

`actions[].type` е числов enum от [списъка на инструментите](#tools-overview).

`actions[].pending` е `true`, когато действието е поставено на опашка за [одобрение](#approval-workflow) вместо да бъде изпълнено.

### `approval.requested`

Схема на `data`:

[inline-code-attrs-start title = 'Схема на данните за поискано одобрение'; type='json' inline-code-attrs-end]
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

Обектът **`args`** съдържа това, което е предадено при извикването на LLM инструмента. Формата му зависи от инструмента:

- За `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- За `mark_comment_spam`: `{ commentId, isSpam }`.
- За `write_comment`: `{ comment, urlId, parentId? }`.
- ...и т.н.

Наборът от форми на аргументите на инструментите **не е стабилен публичен договор**. Инструменти могат да бъдат добавяни в бъдеще, а платформата предава аргументите дословно. Консуматорите трябва да третират args като непрозрачен блок, освен ако изрично не разбират съответния инструмент.

**`contextSnapshot`** улавя контекста на коментара, страницата и потребителя, от който е поставено на опашка одобрението. Формата му огледално отразява контекстното съобщение на тригера.

### `approval.decided`

Схема на `data`:

[inline-code-attrs-start title = 'Схема на данните за взето решение по одобрение'; type='json' inline-code-attrs-end]
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

### Форма на TenantAgentAction

В `actions[]` на полезните товари на тригерите всяко действие има:

[inline-code-attrs-start title = 'Схема на TenantAgentAction'; type='json' inline-code-attrs-end]
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

Стойностите на enum `type` съответстват на `AgentActionType`:

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

`SEARCH_MEMORY` не се появява в `actions[]`, защото е само за четене и не се одитира.

### Стойности на enum `triggerType`

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
- 15: `REPLAY` (вътрешно; не се доставя към webhooks)

### Хедъри

Всяка доставка включва:

- `X-FastComments-Agent-Event` - каноничното име на събитието (`trigger.succeeded` и т.н.).
- `X-FastComments-Signature` - HMAC-SHA256 на суровото тяло с вашия API секрет. Вижте [Подписване на Webhook](#webhook-signing).

### Стабилност

Полетата на плика и документираните полета `data` за всяко събитие са част от публичния договор. Добавянето на нови незадължителни полета към съществуващи полезни товари е разрешено и не се счита за разрушаваща промяна - вашият консуматор трябва да игнорира непознати полета. Формата на `args` и `contextSnapshot` **не** е част от договора.
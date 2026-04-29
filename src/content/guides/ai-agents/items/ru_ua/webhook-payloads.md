Все полезные нагрузки вебхуков агента имеют общую оболочку и добавляют блок `data`, специфичный для события. На этой странице приведена полная схема для каждого из них.

### Оболочка (каждое событие)

Каждая полезная нагрузка, независимо от типа события, имеет следующие поля верхнего уровня:

[inline-code-attrs-start title = 'Схема конверта вебхука'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - соответствующий домен для этой доставки",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - метка времени в формате ISO 8601",
  "data": { /* зависит от события, см. ниже */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

Схема `data`:

[inline-code-attrs-start title = 'Схема данных события триггера'; type='json' inline-code-attrs-end]
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
      "commentId": "string - необязательное",
      "userId": "string - необязательное",
      "badgeId": "string - необязательное",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - присутствует при trigger.failed",
  "url": "string - необязательное",
  "urlId": "string - необязательное",
  "commentId": "string - необязательное"
}
[inline-code-end]

`triggerType` — числовой enum из [trigger event list](#triggers-overview).

`actions[].type` — числовой enum из [tool list](#tools-overview).

`actions[].pending` равно `true`, когда действие было поставлено в очередь на [approval](#approval-workflow) вместо непосредственного выполнения.

### `approval.requested`

Схема `data`:

[inline-code-attrs-start title = 'Схема данных запроса на одобрение'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* зависит от инструмента, см. ниже */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - необязательное, обоснование агента",
  "confidence": 0.85,
  "contextSnapshot": { /* контекст комментария/страницы, для которого запрошено одобрение */ }
}
[inline-code-end]

The **`args`** object is whatever the LLM tool call carried. Its shape depends on the tool:

- Для `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Для `mark_comment_spam`: `{ commentId, isSpam }`.
- Для `write_comment`: `{ comment, urlId, parentId? }`.
- ...and so on.

Набор форм аргументов инструментов **не является стабильным публичным контрактом**. Инструменты могут быть добавлены в будущем, и платформа передаёт args без изменений. Потребителям следует рассматривать args как непрозрачный блок, если они явно не понимают, как работает задействованный инструмент.

The **`contextSnapshot`** фиксирует контекст комментария, страницы и пользователя, из которого был поставлен запрос на одобрение. Его форма зеркалирует контекстное сообщение триггера.

### `approval.decided`

Схема `data`:

[inline-code-attrs-start title = 'Схема данных решения по одобрению'; type='json' inline-code-attrs-end]
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
  "contextSnapshot": { /* то же, что и approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Внутри `actions[]` в payload триггера каждый action имеет:

[inline-code-attrs-start title = 'Схема TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - необязательное",
  "userId": "string - необязательное",
  "badgeId": "string - необязательное",
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

`SEARCH_MEMORY` не появляется в `actions[]`, потому что он только для чтения и неаудируемый.

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
- 15: `REPLAY` (internal; not delivered to webhooks)

### Заголовки

Каждая доставка включает:

- `X-FastComments-Agent-Event` - каноническое имя события (`trigger.succeeded`, и т.д.).
- `X-FastComments-Signature` - HMAC-SHA256 от необработанного тела запроса с использованием вашего секретного ключа API. См. [Webhook Signing](#webhook-signing).

### Стабильность

Поля оболочки и задокументированные поля `data` для каждого события являются частью публичного контракта. Добавление новых необязательных полей в существующие полезные нагрузки допустимо и не считается нарушением обратной совместимости — ваш потребитель должен игнорировать неизвестные поля. Форма `args` и `contextSnapshot` **не** является частью контракта.
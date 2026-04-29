Все полезные нагрузки webhook агента используют общую обёртку и добавляют блок `data`, специфичный для события. На этой странице приведена полная схема для каждого события.

### Обёртка (для каждого события)

Каждая полезная нагрузка, независимо от типа события, содержит следующие поля верхнего уровня:

[inline-code-attrs-start title = 'Схема обёртки вебхука'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - соответствующий домен для этой доставки",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - метка времени ISO 8601",
  "data": { /* для конкретного события, см. ниже */ }
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
      "commentId": "string - необязательно",
      "userId": "string - необязательно",
      "badgeId": "string - необязательно",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - присутствует при trigger.failed",
  "url": "string - необязательно",
  "urlId": "string - необязательно",
  "commentId": "string - необязательно"
}
[inline-code-end]

`triggerType` — числовой enum из [списка триггеров](#triggers-overview).

`actions[].type` — числовой enum из [списка инструментов](#tools-overview).

`actions[].pending` равно `true`, когда действие поставлено в очередь на [одобрение](#approval-workflow) вместо выполнения.

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
  "args": { /* для каждого инструмента, см. ниже */ },
  "createdAt": "string - метка времени ISO 8601",
  "justification": "string - необязательно, обоснование агента",
  "confidence": 0.85,
  "contextSnapshot": { /* контекст комментария/страницы, к которому относится запрос на одобрение */ }
}
[inline-code-end]

Объект **`args`** содержит то, что передавал вызов LLM-инструмента. Его структура зависит от инструмента:

- Для `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Для `mark_comment_spam`: `{ commentId, isSpam }`.
- Для `write_comment`: `{ comment, urlId, parentId? }`.
- ...и так далее.

Набор форм аргументов инструментов **не является стабильным публичным контрактом**. В будущем могут добавляться новые инструменты, и платформа передаёт args как есть. Потребители должны рассматривать args как непрозрачный блок, если они явно не понимают задействованный инструмент.

**`contextSnapshot`** содержит снимок комментария, страницы и пользователя, из которого был поставлен запрос на одобрение. Его структура соответствует сообщению контекста триггера.

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
  "decidedBy": "string - userId модератора, который принял решение",
  "decidedAt": "string - ISO 8601 - необязательно, присутствует только после принятия решения",
  "executedAt": "string - ISO 8601 - присутствует, когда APPROVED и выполнение завершено",
  "executionResult": "string - сообщение о результате исполнителя - присутствует после выполнения",
  "contextSnapshot": { /* то же, что и в approval.requested */ }
}
[inline-code-end]

### TenantAgentAction форма

Внутри `actions[]` в полезных нагрузках триггера каждое действие имеет:

[inline-code-attrs-start title = 'Схема TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - необязательно",
  "userId": "string - необязательно",
  "badgeId": "string - необязательно",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

Значения enum `type` соответствуют `AgentActionType`:

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

`SEARCH_MEMORY` не появляется в `actions[]`, потому что это операция только для чтения и она не проходит аудит.

### Значения enum `triggerType`

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
- 15: `REPLAY` (внутренний; не доставляется в вебхуки)

### Заголовки

Каждая доставка содержит:

- `X-FastComments-Agent-Event` - каноническое имя события (`trigger.succeeded`, и т. д.).
- `X-FastComments-Signature` - HMAC-SHA256 от необработанного тела с использованием вашего API-секрета. См. [Webhook Signing](#webhook-signing).

### Стабильность

Поля обёртки и задокументированные поля `data` для каждого события являются частью публичного контракта. Добавление новых необязательных полей в существующие полезные нагрузки допускается и не считается несовместимым изменением — потребитель должен игнорировать неизвестные поля. Форма `args` и `contextSnapshot` **не** является частью контракта.
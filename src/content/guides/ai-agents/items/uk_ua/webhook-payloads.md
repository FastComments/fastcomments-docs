Усі payload-и вебхуків агента мають спільну обгортку та додають специфічний для події блок `data`. На цій сторінці наведено повну схему для кожної з них.

### Обгортка (кожна подія)

Кожен payload, незалежно від типу події, має такі поля верхнього рівня:

[inline-code-attrs-start title = 'Схема обгортки вебхуку'; type='json' inline-code-attrs-end]
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

Схема `data`:

[inline-code-attrs-start title = 'Схема даних події тригера'; type='json' inline-code-attrs-end]
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

`triggerType` — це числове перерахування зі [списку подій тригерів](#triggers-overview).

`actions[].type` — це числове перерахування зі [списку інструментів](#tools-overview).

`actions[].pending` має значення `true`, коли дію було поставлено в чергу на [схвалення](#approval-workflow) замість виконання.

### `approval.requested`

Схема `data`:

[inline-code-attrs-start title = 'Схема даних запиту на схвалення'; type='json' inline-code-attrs-end]
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

Обʼєкт **`args`** містить те, що передав виклик інструменту LLM. Його форма залежить від інструменту:

- Для `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Для `mark_comment_spam`: `{ commentId, isSpam }`.
- Для `write_comment`: `{ comment, urlId, parentId? }`.
- ...і так далі.

Набір форм аргументів інструментів **не є стабільним публічним контрактом**. Інструменти можуть додаватися в майбутньому, а платформа передає args без змін. Споживачі повинні розглядати args як непрозорий блок даних, якщо вони явно не розуміють відповідний інструмент.

**`contextSnapshot`** фіксує контекст коментаря, сторінки та користувача, з якого було поставлено запит на схвалення в чергу. Його форма віддзеркалює контекстне повідомлення тригера.

### `approval.decided`

Схема `data`:

[inline-code-attrs-start title = 'Схема даних рішення щодо схвалення'; type='json' inline-code-attrs-end]
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

### Форма TenantAgentAction

Усередині `actions[]` у payload-ах тригерів кожна дія має:

[inline-code-attrs-start title = 'Схема TenantAgentAction'; type='json' inline-code-attrs-end]
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

Значення перерахування `type` відповідають `AgentActionType`:

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

`SEARCH_MEMORY` не зʼявляється в `actions[]`, оскільки він доступний лише для читання та не аудитується.

### Значення перерахування `triggerType`

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
- 15: `REPLAY` (внутрішнє; не доставляється у вебхуки)

### Заголовки

Кожна доставка включає:

- `X-FastComments-Agent-Event` - канонічна назва події (`trigger.succeeded` тощо).
- `X-FastComments-Signature` - HMAC-SHA256 сирого тіла з використанням вашого API-секрету. Дивіться [Підписання вебхуків](#webhook-signing).

### Стабільність

Поля обгортки та задокументовані поля `data` для кожної події є частиною публічного контракту. Додавання нових необовʼязкових полів до наявних payload-ів дозволено і не вважається критичною зміною — ваш споживач повинен ігнорувати невідомі поля. Форма `args` та `contextSnapshot` **не** є частиною контракту.
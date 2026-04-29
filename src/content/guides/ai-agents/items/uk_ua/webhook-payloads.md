Всі корисні навантаження (payloads) агентських вебхуків мають спільну обгортку та додають блок `data`, специфічний для події. На цій сторінці наведено повну схему для кожної події.

### Обгортка (кожна подія)

Кожне корисне навантаження, незалежно від типу події, має ці поля верхнього рівня:

[inline-code-attrs-start title = 'Схема пакету вебхука'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - відповідний домен для цієї доставки",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - мітка часу у форматі ISO 8601",
  "data": { /* специфічні для події, див. нижче */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` схема:

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
      "commentId": "string - необов'язково",
      "userId": "string - необов'язково",
      "badgeId": "string - необов'язково",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - присутній у trigger.failed",
  "url": "string - необов'язково",
  "urlId": "string - необов'язково",
  "commentId": "string - необов'язково"
}
[inline-code-end]

`triggerType` — це числовий enum із [переліку подій тригера](#triggers-overview).

`actions[].type` — це числовий enum із [переліку інструментів](#tools-overview).

`actions[].pending` дорівнює `true`, коли дія була поставлена в чергу для [затвердження](#approval-workflow) замість виконання.

### `approval.requested`

`data` схема:

[inline-code-attrs-start title = 'Схема даних запиту на затвердження'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* залежно від інструмента, див. нижче */ },
  "createdAt": "string - мітка часу у форматі ISO 8601",
  "justification": "string - необов'язково, обґрунтування агента",
  "confidence": 0.85,
  "contextSnapshot": { /* контекст коментаря/сторінки, до якого стосується затвердження */ }
}
[inline-code-end]

Об'єкт **`args`** — це те, що передавав виклик інструмента LLM. Його форма залежить від інструмента:

- Для `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Для `mark_comment_spam`: `{ commentId, isSpam }`.
- Для `write_comment`: `{ comment, urlId, parentId? }`.
- ...і так далі.

Набір форм аргументів інструментів **не є стабільним публічним контрактом**. Інструменти можуть додаватися в майбутньому, а платформа передає args дослівно. Споживачі повинні трактувати args як непрозорий блок, якщо вони явно не розуміють залучений інструмент.

**`contextSnapshot`** фіксує контекст коментаря, сторінки та користувача, із якого було поставлено запит на затвердження. Його форма відображає контекстне повідомлення тригера.

### `approval.decided`

`data` схема:

[inline-code-attrs-start title = 'Схема даних рішення по затвердженню'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - userId модератора, який ухвалив рішення",
  "decidedAt": "string - мітка часу у форматі ISO 8601 - необов'язково, присутній тільки після ухвалення рішення",
  "executedAt": "string - мітка часу у форматі ISO 8601 - присутній коли APPROVED і виконання завершено",
  "executionResult": "string - повідомлення про результат виконання - присутнє після виконання",
  "contextSnapshot": { /* same as approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Усередині `actions[]` у payload-ах тригера кожна дія має:

[inline-code-attrs-start title = 'Схема TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - необов'язково",
  "userId": "string - необов'язково",
  "badgeId": "string - необов'язково",
  "pending": false,
  "justification": "string",
  "confidence": 0.92
}
[inline-code-end]

Значення enum для `type` відповідають `AgentActionType`:

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

`SEARCH_MEMORY` не з'являється в `actions[]`, бо він тільки для читання і неаудитується.

### Значення enum для `triggerType`

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
- 15: `REPLAY` (внутрішній; не доставляється у вебхуки)

### Заголовки

Кожна доставка включає:

- `X-FastComments-Agent-Event` - канонічна назва події (`trigger.succeeded`, тощо).
- `X-FastComments-Signature` - HMAC-SHA256 від сирого тіла за допомогою вашого секрету API. Див. [Webhook Signing](#webhook-signing).

### Стабільність

Поля обгортки та задокументовані поля `data` для кожної події є частиною публічного контракту. Додавання нових необов'язкових полів до існуючих payload-ів дозволено і не вважається зламом сумісності — ваш споживач повинен ігнорувати невідомі поля. Форма `args` та `contextSnapshot` **не** є частиною контракту.
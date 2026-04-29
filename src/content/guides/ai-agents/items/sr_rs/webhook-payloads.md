---
Све поруке вебхука агента деле заједнички омотач и додају блок `data` специфичан за догађај. Ова страница наводи потпуну шему за сваки.

### Омотач (сваки догађај)

Свака порука, без обзира на тип догађаја, има ова главна поља:

[inline-code-attrs-start title = 'Шема омотача вебхука'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - одговарајући домен за ову доставу",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 временска ознака",
  "data": { /* специфично за догађај, види доле */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` шема:

[inline-code-attrs-start title = 'Шема података тригера'; type='json' inline-code-attrs-end]
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
      "commentId": "string - опционално",
      "userId": "string - опционално",
      "badgeId": "string - опционално",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - присутно у trigger.failed",
  "url": "string - опционално",
  "urlId": "string - опционално",
  "commentId": "string - опционално"
}
[inline-code-end]

`triggerType` је нумерички енум из [списка догађаја тригера](#triggers-overview).

`actions[].type` је нумерички енум из [листе алата](#tools-overview).

`actions[].pending` је `true` када је акција стављена у ред за [одобрење](#approval-workflow) уместо да буде извршена.

### `approval.requested`

`data` шема:

[inline-code-attrs-start title = 'Шема података захтева за одобрење'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* по алату, види доле */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - опционално, образложење агента",
  "confidence": 0.85,
  "contextSnapshot": { /* контекст коментара/странице на који се односи одобрење */ }
}
[inline-code-end]

Објекат **`args`** представља шта год је LLM позив алата пренео. Облик зависи од алата:

- За `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- За `mark_comment_spam`: `{ commentId, isSpam }`.
- За `write_comment`: `{ comment, urlId, parentId? }`.
- ...и тако даље.

Низ облика аргумената алата није стабилан јавни уговор. Алати се могу додавати у будућности и платформа прослеђује args непромењене. Потрошачи треба да третирају args као неинтерпретирани бинарни садржај осим ако експлицитно не разумеју укључени алат.

**`contextSnapshot`** бележи контекст коментара, странице и корисника из кога је одобрење стављено у ред. Његов облик одговара контекстној поруци тригера.

### `approval.decided`

`data` шема:

[inline-code-attrs-start title = 'Шема података одлуке о одобрењу'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - the userId of the moderator who decided",
  "decidedAt": "string - ISO 8601 - опционално, присутно само након одлуке",
  "executedAt": "string - ISO 8601 - присутно када је APPROVED и извршење је завршено",
  "executionResult": "string - executor result message - present after execute",
  "contextSnapshot": { /* исти као approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'Шема TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - опционално",
  "userId": "string - опционално",
  "badgeId": "string - опционално",
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

`SEARCH_MEMORY` се не појављује у `actions[]` јер је само за читање и није ревидирано.

### Вредности енумa `triggerType`

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
- 15: `REPLAY` (интерно; не доставља се вебхуковима)

### Заглавља

Свака испорука укључује:

- `X-FastComments-Agent-Event` - канонско име догађаја (`trigger.succeeded`, итд.).
- `X-FastComments-Signature` - HMAC-SHA256 нестркутурисаног тела користећи вашу API тајну. Погледајте [Webhook Signing](#webhook-signing).

### Стабилност

Поља омотача и документована поља `data` по догађају су део јавног уговора. Додавање нових опционих поља у постојеће поруке је дозвољено и не сматра се breaking променом — ваш конзумер треба да игнорише непозната поља. Облик `args` и `contextSnapshot` **није** део уговора.

---
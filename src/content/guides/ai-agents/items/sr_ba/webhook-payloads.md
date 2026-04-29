Сви agent webhook payload-ови деле заједнички омот и додају блок `data` специфичан за догађај. Ова страница наводи пуну шему за сваки.

### Омотница (сваког догађаја)

Сваки payload, без обзира на тип догађаја, има ова горњи поља:

[inline-code-attrs-start title = 'Шема омотнице вебхука'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - домен који је одговарао овој достави",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 временска ознака",
  "data": { /* специфично за догађај, види доле */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema:

[inline-code-attrs-start title = 'Шема података догађаја тригера'; type='json' inline-code-attrs-end]
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
  "errorMessage": "string - присутно код trigger.failed",
  "url": "string - опционално",
  "urlId": "string - опционално",
  "commentId": "string - опционално"
}
[inline-code-end]

`triggerType` је нумерички enum из [листе тригер догађаја](#triggers-overview).

`actions[].type` је нумерички enum из [листе алата](#tools-overview).

`actions[].pending` је `true` када је акција стављена у ред за [одобрење](#approval-workflow) уместо да се изврши.

### `approval.requested`

`data` schema:

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
  "justification": "string - опционално, разлози агента",
  "confidence": 0.85,
  "contextSnapshot": { /* снимак контекста коментара/странице на који се односи захтев за одобрење */ }
}
[inline-code-end]

Објекат **`args`** је оно што је LLM позив алата носио. Облик зависи од алата:

- За `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- За `mark_comment_spam`: `{ commentId, isSpam }`.
- За `write_comment`: `{ comment, urlId, parentId? }`.
- ...и тако даље.

Скуп облика аргумената алата није стабилан јавни уговор. Алате могу бити додате у будућности и платформа прослеђује args непромењено. Потрошачи треба да третирају args као непрозирну целину осим ако изричито не разумеју укључени алат.

**`contextSnapshot`** снима контекст коментара, странице и корисника из ког је захтев за одобрење редован. Његов облик огледа контекстну поруку тригера.

### `approval.decided`

`data` schema:

[inline-code-attrs-start title = 'Шема података одлуке о одобрењу'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - userId модератора који је донео одлуку",
  "decidedAt": "string - ISO 8601 - опционално, присутно само након одлуке",
  "executedAt": "string - ISO 8601 - присутно када је APPROVED и извршавање завршено",
  "executionResult": "string - порука резултата извршиоца - присутно након извршења",
  "contextSnapshot": { /* исто као у approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Унутар `actions[]` на trigger payload-овима, свака акција има:

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

`type` enum вредности одговарају `AgentActionType`:

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

`SEARCH_MEMORY` се не појављује у `actions[]` јер је само за читање и не ревидира се.

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
- 15: `REPLAY` (интерно; не испоручује се вебхуковима)

### Заглавља

Свака испорука садржи:

- `X-FastComments-Agent-Event` - канонско име догађаја (`trigger.succeeded`, итд.).
- `X-FastComments-Signature` - HMAC-SHA256 сировог тела користећи ваш API тајни кључ. Види [Потписивање вебхука](#webhook-signing).

### Стабилност

Поља омотнице и документована `data` поља по догађају су део јавног уговора. Додавање нових опционих поља у постојеће payload-ове је дозвољено и не сматра се breaking променом — ваш потрошач треба да игнорише непозната поља. Облик `args` и `contextSnapshot` **није** део уговора.

---
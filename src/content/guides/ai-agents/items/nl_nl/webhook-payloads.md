Alle agent-webhook-payloads delen een gemeenschappelijke envelope en voegen een eventspecifiek `data`-blok toe. Deze pagina bevat het volledige schema voor elk ervan.

### Envelope (elk event)

Elke payload heeft, ongeacht het eventtype, deze velden op het hoogste niveau:

[inline-code-attrs-start title = 'Webhook Envelope Schema'; type='json' inline-code-attrs-end]
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

`data`-schema:

[inline-code-attrs-start title = 'Trigger Event Data Schema'; type='json' inline-code-attrs-end]
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

`triggerType` is een numerieke enum uit de [trigger event list](#triggers-overview).

`actions[].type` is een numerieke enum uit de [tool list](#tools-overview).

`actions[].pending` is `true` wanneer de actie in de wachtrij is geplaatst voor [goedkeuring](#approval-workflow) in plaats van uitgevoerd.

### `approval.requested`

`data`-schema:

[inline-code-attrs-start title = 'Approval Requested Data Schema'; type='json' inline-code-attrs-end]
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

Het **`args`**-object bevat wat de LLM-toolcall heeft meegegeven. De vorm ervan hangt af van de tool:

- Voor `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Voor `mark_comment_spam`: `{ commentId, isSpam }`.
- Voor `write_comment`: `{ comment, urlId, parentId? }`.
- ...enzovoort.

De set tool-argumentvormen is **geen stabiel publiek contract**. Tools kunnen in de toekomst worden toegevoegd en het platform geeft args ongewijzigd door. Consumenten moeten args behandelen als een ondoorzichtig gegevensblok, tenzij ze de betreffende tool expliciet begrijpen.

De **`contextSnapshot`** legt de comment-, pagina- en gebruikerscontext vast waaruit de goedkeuring in de wachtrij is geplaatst. De vorm ervan weerspiegelt het contextbericht van de trigger.

### `approval.decided`

`data`-schema:

[inline-code-attrs-start title = 'Approval Decided Data Schema'; type='json' inline-code-attrs-end]
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

### TenantAgentAction-vorm

Binnen `actions[]` in de trigger-payloads heeft elke actie:

[inline-code-attrs-start title = 'TenantAgentAction Schema'; type='json' inline-code-attrs-end]
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

De `type`-enumwaarden komen overeen met `AgentActionType`:

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

`SEARCH_MEMORY` verschijnt niet in `actions[]` omdat het alleen-lezen is en niet wordt gecontroleerd.

### `triggerType`-enumwaarden

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
- 15: `REPLAY` (intern; niet geleverd aan webhooks)

### Headers

Elke levering bevat:

- `X-FastComments-Agent-Event` - de canonieke eventnaam (`trigger.succeeded`, enz.).
- `X-FastComments-Signature` - HMAC-SHA256 van de ruwe body met behulp van je API-geheim. Zie [Webhook Signing](#webhook-signing).

### Stabiliteit

De envelope-velden en de gedocumenteerde `data`-velden per event maken deel uit van het publieke contract. Het toevoegen van nieuwe optionele velden aan bestaande payloads is toegestaan en wordt niet als een breaking change beschouwd - je consument zou onbekende velden moeten negeren. De vorm van `args` en `contextSnapshot` is **geen** deel van het contract.
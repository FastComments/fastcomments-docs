Alle agent-webhook-payloads deler en fælles konvolut og tilføjer et hændelsesspecifikt `data`-blok. Denne side angiver hele skemaet for hver.

### Konvolut (hver begivenhed)

Hver payload, uanset hændelsestype, har disse topniveaufelter:

[inline-code-attrs-start title = 'Webhook-envelopeskema'; type='json' inline-code-attrs-end]
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

`data` skema:

[inline-code-attrs-start title = 'Trigger-hændelsesdataskema'; type='json' inline-code-attrs-end]
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

`triggerType` er en numerisk enum fra [listen over trigger-hændelser](#triggers-overview).

`actions[].type` er en numerisk enum fra [listen over værktøjer](#tools-overview).

`actions[].pending` er `true`, når handlingen blev køet til [godkendelse](#approval-workflow) i stedet for at blive udført.

### `approval.requested`

`data` skema:

[inline-code-attrs-start title = 'Godkendelsesanmodnings-dataskema'; type='json' inline-code-attrs-end]
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

Objektet **`args`** er hvad LLM-værktøjskaldet sendte med. Dets form afhænger af værktøjet:

- For `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- For `mark_comment_spam`: `{ commentId, isSpam }`.
- For `write_comment`: `{ comment, urlId, parentId? }`.
- ...og så videre.

Sættet af værktøjsargumentformater er **ikke en stabil offentlig kontrakt**. Værktøjer kan blive tilføjet i fremtiden, og platformen videresender `args` uændret. Forbrugere bør behandle `args` som en uigennemsigtig blob, medmindre de eksplicit forstår det involverede værktøj.

Objektet **`contextSnapshot`** indfanger kommentar-, side- og bruger-konteksten som godkendelsen blev køet fra. Dets form spejler triggerens kontekstbesked.

### `approval.decided`

`data` skema:

[inline-code-attrs-start title = 'Godkendelsesbeslutnings-dataskema'; type='json' inline-code-attrs-end]
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

### TenantAgentAction-struktur

Inde i `actions[]` i trigger-payloads, har hver handling:

[inline-code-attrs-start title = 'TenantAgentAction-skema'; type='json' inline-code-attrs-end]
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

`type`-enumværdier svarer til `AgentActionType`:

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

`SEARCH_MEMORY` fremgår ikke i `actions[]`, fordi det er skrivebeskyttet og ikke auditeret.

### `triggerType` enumværdier

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
- 15: `REPLAY` (internt; ikke leveret til webhooks)

### Headere

Hver levering inkluderer:

- `X-FastComments-Agent-Event` - det kanoniske hændelsesnavn (`trigger.succeeded`, osv.).
- `X-FastComments-Signature` - HMAC-SHA256 af den rå body ved hjælp af din API-secret. Se [Webhook-signering](#webhook-signing).

### Stabilitet

Konvolutfelterne og de dokumenterede `data`-felter pr. hændelse er en del af den offentlige kontrakt. Tilføjelse af nye valgfrie felter til eksisterende payloads er tilladt og betragtes ikke som et brud på kompatibilitet — din forbruger bør ignorere ukendte felter. Formen af `args` og `contextSnapshot` er **ikke** en del af kontrakten.
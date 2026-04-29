Wszystkie ładunki webhooków agenta korzystają ze wspólnej koperty i dodają specyficzny dla zdarzenia blok `data`. Ta strona zawiera pełny schemat dla każdego z nich.

### Koperta (każde zdarzenie)

Każdy ładunek, niezależnie od typu zdarzenia, zawiera następujące pola najwyższego poziomu:

[inline-code-attrs-start title = 'Schemat koperty webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - dopasowana domena dla tej dostawy",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - znacznik czasu ISO 8601",
  "data": { /* specyficzne dla zdarzenia, patrz niżej */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema:

[inline-code-attrs-start title = 'Schemat danych zdarzenia trigger'; type='json' inline-code-attrs-end]
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
      "commentId": "string - opcjonalne",
      "userId": "string - opcjonalne",
      "badgeId": "string - opcjonalne",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - obecne w trigger.failed",
  "url": "string - opcjonalne",
  "urlId": "string - opcjonalne",
  "commentId": "string - opcjonalne"
}
[inline-code-end]

`triggerType` is a numeric enum from the [trigger event list](#triggers-overview).

`actions[].type` is a numeric enum from the [tool list](#tools-overview).

`actions[].pending` is `true` when the action was queued for [approval](#approval-workflow) instead of executed.

### `approval.requested`

`data` schema:

[inline-code-attrs-start title = 'Schemat danych żądania zatwierdzenia'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* w zależności od narzędzia, patrz niżej */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - opcjonalne, uzasadnienie agenta",
  "confidence": 0.85,
  "contextSnapshot": { /* kontekst komentarza/strony, którego dotyczy zatwierdzenie */ }
}
[inline-code-end]

The **`args`** object is whatever the LLM tool call carried. Its shape depends on the tool:

- For `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- For `mark_comment_spam`: `{ commentId, isSpam }`.
- For `write_comment`: `{ comment, urlId, parentId? }`.
- ...and so on.

The set of tool argument shapes is **not a stable public contract**. Tools can be added in future and the platform passes args through verbatim. Consumers should treat args as an opaque blob unless they explicitly understand the tool involved.

The **`contextSnapshot`** captures the comment, page, and user context the approval was queued from. Its shape mirrors the trigger's context message.

### `approval.decided`

`data` schema:

[inline-code-attrs-start title = 'Schemat danych decyzji zatwierdzenia'; type='json' inline-code-attrs-end]
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

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'TenantAgentAction Schema'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - opcjonalne",
  "userId": "string - opcjonalne",
  "badgeId": "string - opcjonalne",
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
- 15: `REPLAY` (internal; not delivered to webhooks)

### Nagłówki

Każda dostawa zawiera:

- `X-FastComments-Agent-Event` - kanoniczna nazwa zdarzenia (`trigger.succeeded`, itd.).
- `X-FastComments-Signature` - HMAC-SHA256 surowego ciała przy użyciu twojego sekretu API. Zobacz [Podpis webhooków](#webhook-signing).

### Stabilność

Pola koperty oraz udokumentowane pola `data` dla każdego zdarzenia są częścią publicznego kontraktu. Dodawanie nowych opcjonalnych pól do istniejących payloadów jest dozwolone i nie jest uważane za zmianę łamiącą kompatybilność — konsumenci powinni ignorować nieznane pola. Struktura `args` i `contextSnapshot` **nie** jest częścią kontraktu.
Vsi agentni webhook payloadi imajo skupno ovojnico in dodajo dogodek-specifičen blok `data`. Ta stran navaja celotno shemo za vsak dogodek.

### Envelope (every event)

Every payload, regardless of event type, has these top-level fields:

[inline-code-attrs-start title = 'Shema ovojnice webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - ujemajoča se domena za to dostavo",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - časovni žig v formatu ISO 8601",
  "data": { /* event-specific, see below */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema:

[inline-code-attrs-start title = 'Shema podatkov sproženega dogodka'; type='json' inline-code-attrs-end]
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
      "commentId": "string - neobvezno",
      "userId": "string - neobvezno",
      "badgeId": "string - neobvezno",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - prisoten pri trigger.failed",
  "url": "string - neobvezno",
  "urlId": "string - neobvezno",
  "commentId": "string - neobvezno"
}
[inline-code-end]

`triggerType` is a numeric enum from the [trigger event list](#triggers-overview).

`actions[].type` is a numeric enum from the [tool list](#tools-overview).

`actions[].pending` is `true` when the action was queued for [approval](#approval-workflow) instead of executed.

### `approval.requested`

`data` schema:

[inline-code-attrs-start title = 'Shema podatkov zahteve za odobritev'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* glede na orodje, glej spodaj */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - neobvezno, utemeljitev agenta",
  "confidence": 0.85,
  "contextSnapshot": { /* the comment/page context the approval is about */ }
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

[inline-code-attrs-start title = 'Shema podatkov odločitve o odobritvi'; type='json' inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Shema TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - neobvezno",
  "userId": "string - neobvezno",
  "badgeId": "string - neobvezno",
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

### Headers

Every delivery includes:

- `X-FastComments-Agent-Event` - the canonical event name (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 of the raw body using your API secret. See [Podpisovanje webhookov](#webhook-signing).

### Stability

The envelope fields and the documented `data` fields per event are part of the public contract. Adding new optional fields to existing payloads is allowed and not considered a breaking change - your consumer should ignore unknown fields. The shape of `args` and `contextSnapshot` is **not** part of the contract.
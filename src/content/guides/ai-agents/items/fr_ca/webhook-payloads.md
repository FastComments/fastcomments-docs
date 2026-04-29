Tous les payloads webhook des agents partagent une enveloppe commune et ajoutent un bloc `data` spécifique à l'événement. Cette page liste le schéma complet pour chacun.

### Envelope (every event)

Every payload, regardless of event type, has these top-level fields:

[inline-code-attrs-start title = 'Schéma de l\'enveloppe du webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - le domaine correspondant à cette livraison",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - horodatage ISO 8601",
  "data": { /* spécifique à l'événement, voir ci-dessous */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema:

[inline-code-attrs-start title = 'Schéma des données de l\'événement trigger'; type='json' inline-code-attrs-end]
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
      "commentId": "string - optionnel",
      "userId": "string - optionnel",
      "badgeId": "string - optionnel",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - présent sur trigger.failed",
  "url": "string - optionnel",
  "urlId": "string - optionnel",
  "commentId": "string - optionnel"
}
[inline-code-end]

`triggerType` is a numeric enum from the [trigger event list](#triggers-overview).

`actions[].type` is a numeric enum from the [tool list](#tools-overview).

`actions[].pending` is `true` when the action was queued for [approval](#approval-workflow) instead of executed.

### `approval.requested`

`data` schema:

[inline-code-attrs-start title = 'Schéma des données de demande d\'approbation'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* par outil, voir ci-dessous */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - optionnel, raisonnement de l'agent",
  "confidence": 0.85,
  "contextSnapshot": { /* le contexte du commentaire/page auquel l'approbation se rapporte */ }
}
[inline-code-end]

The **`args`** object is whatever the LLM tool call carried. Its shape depends on the tool:

- For `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- For `mark_comment_spam`: `{ commentId, isSpam }`.
- For `write_comment`: `{ comment, urlId, parentId? }`.
- ...and so on.

L'ensemble des formes des arguments d'outil **n'est pas un contrat public stable**. Des outils peuvent être ajoutés à l'avenir et la plateforme transmet les args tels quels. Les consommateurs devraient traiter args comme un blob opaque à moins de comprendre explicitement l'outil impliqué.

The **`contextSnapshot`** captures the comment, page, and user context the approval was queued from. Its shape mirrors the trigger's context message.

### `approval.decided`

`data` schema:

[inline-code-attrs-start title = 'Schéma des données de décision d\'approbation'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - the userId of the moderator who decided",
  "decidedAt": "string - ISO 8601 - optionnel, présent seulement une fois décidé",
  "executedAt": "string - ISO 8601 - présent quand APPROVED et l'exécution est terminée",
  "executionResult": "string - executor result message - présent après l'exécution",
  "contextSnapshot": { /* identique à approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'Schéma TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - optionnel",
  "userId": "string - optionnel",
  "badgeId": "string - optionnel",
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
- 15: `REPLAY` (interne; non envoyé aux webhooks)

### Headers

Every delivery includes:

- `X-FastComments-Agent-Event` - the canonical event name (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 of the raw body using your API secret. See [Webhook Signing](#webhook-signing).

### Stability

The envelope fields and the documented `data` fields per event are part of the public contract. Adding new optional fields to existing payloads is allowed and not considered a breaking change - your consumer should ignore unknown fields. La forme de `args` et `contextSnapshot` **ne fait pas** partie du contrat.
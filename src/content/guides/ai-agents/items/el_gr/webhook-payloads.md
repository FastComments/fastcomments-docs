Όλα τα agent webhook payloads μοιράζονται ένα κοινό φάκελο και προσθέτουν ένα ειδικό για το συμβάν μπλοκ `data`. Αυτή η σελίδα απαριθμεί το πλήρες σχήμα για το καθένα.

### Envelope (every event)

Κάθε payload, ανεξάρτητα από τον τύπο συμβάντος, έχει αυτά τα κορυφαία πεδία:

[inline-code-attrs-start title = 'Σχήμα Φακέλου Webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - το αντιστοιχισμένο domain για αυτήν την παράδοση",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - χρονική σήμανση ISO 8601",
  "data": { /* συγκεκριμένο για το συμβάν, δείτε παρακάτω */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schema:

[inline-code-attrs-start title = 'Σχήμα Δεδομένων Συμβάντος Trigger'; type='json' inline-code-attrs-end]
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
      "commentId": "string - προαιρετικό",
      "userId": "string - προαιρετικό",
      "badgeId": "string - προαιρετικό",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - παρόν στο trigger.failed",
  "url": "string - προαιρετικό",
  "urlId": "string - προαιρετικό",
  "commentId": "string - προαιρετικό"
}
[inline-code-end]

`triggerType` is a numeric enum from the [trigger event list](#triggers-overview).

`actions[].type` is a numeric enum from the [tool list](#tools-overview).

`actions[].pending` is `true` when the action was queued for [approval](#approval-workflow) instead of executed.

### `approval.requested`

`data` schema:

[inline-code-attrs-start title = 'Σχήμα Δεδομένων Αιτήματος Έγκρισης'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* ανά εργαλείο, δείτε παρακάτω */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - προαιρετικό, αιτιολόγηση του agent",
  "confidence": 0.85,
  "contextSnapshot": { /* το πλαίσιο σχολίου/σελίδας στο οποίο αφορά η έγκριση */ }
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

[inline-code-attrs-start title = 'Σχήμα Δεδομένων Απόφασης Έγκρισης'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - το userId του moderator που αποφάσισε",
  "decidedAt": "string - ISO 8601 - προαιρετικό, παρόν μόνο αφού ληφθεί απόφαση",
  "executedAt": "string - ISO 8601 - παρόν όταν είναι APPROVED και η εκτέλεση ολοκληρώθηκε",
  "executionResult": "string - μήνυμα αποτελέσματος εκτελεστή - παρόν μετά την εκτέλεση",
  "contextSnapshot": { /* ίδιο με approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'Σχήμα TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - προαιρετικό",
  "userId": "string - προαιρετικό",
  "badgeId": "string - προαιρετικό",
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

- `X-FastComments-Agent-Event` - το κανονικό όνομα του συμβάντος (`trigger.succeeded`, κ.λπ.).
- `X-FastComments-Signature` - HMAC-SHA256 του raw body χρησιμοποιώντας το API secret σας. Δείτε [Webhook Signing](#webhook-signing).

### Stability

Τα πεδία του φακέλου και τα τεκμηριωμένα πεδία `data` ανά συμβάν είναι μέρος του δημόσιου συμβολαίου. Η προσθήκη νέων προαιρετικών πεδίων σε υπάρχοντα payloads επιτρέπεται και δεν θεωρείται breaking change - ο καταναλωτής σας θα πρέπει να αγνοεί άγνωστα πεδία. Το σχήμα των `args` και `contextSnapshot` **δεν** αποτελεί μέρος του συμβολαίου.
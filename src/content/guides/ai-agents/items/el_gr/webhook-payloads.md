Όλα τα payloads των agent webhooks μοιράζονται ένα κοινό envelope και προσθέτουν ένα μπλοκ `data` ειδικό για το συμβάν. Αυτή η σελίδα παραθέτει το πλήρες schema για καθένα.

### Envelope (κάθε συμβάν)

Κάθε payload, ανεξάρτητα από τον τύπο συμβάντος, έχει αυτά τα πεδία κορυφαίου επιπέδου:

[inline-code-attrs-start title = 'Σχήμα Envelope Webhook'; type='json' inline-code-attrs-end]
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

Σχήμα `data`:

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

Το `triggerType` είναι ένας αριθμητικός enum από τη [λίστα συμβάντων trigger](#triggers-overview).

Το `actions[].type` είναι ένας αριθμητικός enum από τη [λίστα εργαλείων](#tools-overview).

Το `actions[].pending` είναι `true` όταν η ενέργεια μπήκε στην ουρά για [έγκριση](#approval-workflow) αντί να εκτελεστεί.

### `approval.requested`

Σχήμα `data`:

[inline-code-attrs-start title = 'Σχήμα Δεδομένων Approval Requested'; type='json' inline-code-attrs-end]
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

Το αντικείμενο **`args`** είναι ό,τι μετέφερε η κλήση εργαλείου του LLM. Το σχήμα του εξαρτάται από το εργαλείο:

- Για `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Για `mark_comment_spam`: `{ commentId, isSpam }`.
- Για `write_comment`: `{ comment, urlId, parentId? }`.
- ...και ούτω καθεξής.

Το σύνολο των σχημάτων ορισμάτων των εργαλείων **δεν είναι σταθερό δημόσιο συμβόλαιο**. Εργαλεία μπορούν να προστεθούν στο μέλλον και η πλατφόρμα περνά τα args αυτούσια. Οι καταναλωτές πρέπει να αντιμετωπίζουν τα args ως αδιαφανές blob, εκτός αν κατανοούν ρητά το εμπλεκόμενο εργαλείο.

Το **`contextSnapshot`** καταγράφει το πλαίσιο σχολίου, σελίδας και χρήστη από το οποίο μπήκε στην ουρά η έγκριση. Το σχήμα του αντικατοπτρίζει το μήνυμα πλαισίου του trigger.

### `approval.decided`

Σχήμα `data`:

[inline-code-attrs-start title = 'Σχήμα Δεδομένων Approval Decided'; type='json' inline-code-attrs-end]
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

### Σχήμα TenantAgentAction

Μέσα στο `actions[]` των payloads των trigger, κάθε ενέργεια έχει:

[inline-code-attrs-start title = 'Σχήμα TenantAgentAction'; type='json' inline-code-attrs-end]
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

Οι τιμές του enum `type` αντιστοιχούν στο `AgentActionType`:

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

Το `SEARCH_MEMORY` δεν εμφανίζεται στο `actions[]` επειδή είναι μόνο για ανάγνωση και δεν καταγράφεται.

### Τιμές enum `triggerType`

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
- 15: `REPLAY` (εσωτερικό· δεν παραδίδεται στα webhooks)

### Headers

Κάθε παράδοση περιλαμβάνει:

- `X-FastComments-Agent-Event` - το κανονικό όνομα του συμβάντος (`trigger.succeeded`, κ.λπ.).
- `X-FastComments-Signature` - HMAC-SHA256 του ακατέργαστου body χρησιμοποιώντας το API secret σας. Δείτε [Υπογραφή Webhook](#webhook-signing).

### Σταθερότητα

Τα πεδία του envelope και τα τεκμηριωμένα πεδία `data` ανά συμβάν αποτελούν μέρος του δημόσιου συμβολαίου. Η προσθήκη νέων προαιρετικών πεδίων σε υπάρχοντα payloads επιτρέπεται και δεν θεωρείται αλλαγή που χαλάει τη συμβατότητα - ο καταναλωτής σας πρέπει να αγνοεί άγνωστα πεδία. Το σχήμα των `args` και `contextSnapshot` **δεν** αποτελεί μέρος του συμβολαίου.
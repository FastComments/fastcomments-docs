Όλα τα payloads webhook του agent μοιράζονται ένα κοινό περίβλημα και προσθέτουν ένα συμβάν-ειδικό μπλοκ `data`. Αυτή η σελίδα καταγράφει το πλήρες σχήμα για το κάθε ένα.

### Envelope (κάθε συμβάν)

Κάθε payload, ανεξαρτήτως τύπου συμβάντος, έχει αυτά τα κορυφαία πεδία:

[inline-code-attrs-start title = 'Σχήμα Περιβλήματος Webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - ο αντιστοιχισμένος τομέας για αυτή την παράδοση",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - χρονοσφραγίδα ISO 8601",
  "data": { /* event-specific, see below */ }
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
  "errorMessage": "string - παρών σε trigger.failed",
  "url": "string - προαιρετικό",
  "urlId": "string - προαιρετικό",
  "commentId": "string - προαιρετικό"
}
[inline-code-end]

`triggerType` είναι ένας αριθμητικός enum από τη [λίστα συμβάντων trigger](#triggers-overview).

`actions[].type` είναι ένας αριθμητικός enum από τη [λίστα εργαλείων](#tools-overview).

`actions[].pending` είναι `true` όταν η ενέργεια μπήκε σε ουρά για [έγκριση](#approval-workflow) αντί να εκτελεστεί.

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
  "createdAt": "string - χρονοσφραγίδα ISO 8601",
  "justification": "string - προαιρετικό, αιτιολόγηση του agent",
  "confidence": 0.85,
  "contextSnapshot": { /* το στιγμιότυπο συμφραζομένων του σχολίου/σελίδας για το οποίο αφορά η έγκριση */ }
}
[inline-code-end]

Το αντικείμενο **`args`** είναι ό,τι μετέφερε η κλήση εργαλείου LLM. Η μορφή του εξαρτάται από το εργαλείο:

- Για `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Για `mark_comment_spam`: `{ commentId, isSpam }`.
- Για `write_comment`: `{ comment, urlId, parentId? }`.
- ...και ούτω κάθε εξής.

Το σύνολο των σχημάτων επιχειρημάτων εργαλείων **δεν αποτελεί σταθερό δημόσιο συμβόλαιο**. Μπορούν να προστεθούν εργαλεία στο μέλλον και η πλατφόρμα μεταβιβάζει τα args αυτούσια. Οι καταναλωτές θα πρέπει να θεωρούν τα args ως αδιαφανές blob εκτός αν κατανοούν ρητώς το εμπλεκόμενο εργαλείο.

Το **`contextSnapshot`** καταγράφει τα συμφραζόμενα σχολίου, σελίδας και χρήστη από τα οποία μπήκε η έγκριση στην ουρά. Η μορφή του αντικατοπτρίζει το μήνυμα συμφραζομένων του trigger.

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
  "decidedAt": "string - χρονοσφραγίδα ISO 8601 - προαιρετικό, παρών μόνο αφού αποφασιστεί",
  "executedAt": "string - χρονοσφραγίδα ISO 8601 - παρών όταν είναι APPROVED και η εκτέλεση ολοκληρώθηκε",
  "executionResult": "string - μήνυμα αποτελέσματος εκτελεστή - παρών μετά την εκτέλεση",
  "contextSnapshot": { /* ίδιο με το approval.requested */ }
}
[inline-code-end]

### TenantAgentAction σχήμα

Μέσα στο `actions[]` στα payloads trigger, κάθε ενέργεια έχει:

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

Οι τιμές του enum `type` αντιστοιχούν σε `AgentActionType`:

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

`SEARCH_MEMORY` δεν εμφανίζεται στο `actions[]` επειδή είναι μόνο για ανάγνωση και δεν ελέγχεται (unaudited).

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
- 15: `REPLAY` (εσωτερικό; δεν αποστέλλεται σε webhooks)

### Headers

Κάθε παράδοση περιλαμβάνει:

- `X-FastComments-Agent-Event` - το κανονικοποιημένο όνομα συμβάντος (`trigger.succeeded`, κ.λπ.).
- `X-FastComments-Signature` - HMAC-SHA256 του ακατέργαστου σώματος χρησιμοποιώντας το API secret σας. Δείτε [Webhook Signing](#webhook-signing).

### Σταθερότητα

Τα πεδία του περιβλήματος και τα τεκμηριωμένα πεδία `data` ανά συμβάν αποτελούν μέρος του δημόσιου συμβολαίου. Η προσθήκη νέων προαιρετικών πεδίων σε υπάρχοντα payloads επιτρέπεται και δεν θεωρείται αλλαγή που σπάει συμβατότητα — ο καταναλωτής σας πρέπει να αγνοεί άγνωστα πεδία. Η μορφή των `args` και `contextSnapshot` **δεν** αποτελεί μέρος του συμβολαίου.
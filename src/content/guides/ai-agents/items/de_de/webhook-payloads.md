Alle Agent-Webhook-Payloads teilen sich einen gemeinsamen Umschlag und fügen einen ereignisspezifischen `data`-Block hinzu. Diese Seite listet das vollständige Schema für jedes Ereignis auf.

### Umschlag (jedes Ereignis)

Jede Payload hat, unabhängig vom Ereignistyp, diese Felder auf oberster Ebene:

[inline-code-attrs-start title = 'Webhook-Umschlag-Schema'; type='json' inline-code-attrs-end]
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

`data`-Schema:

[inline-code-attrs-start title = 'Trigger-Ereignis-Datenschema'; type='json' inline-code-attrs-end]
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

`triggerType` ist ein numerischer Enum aus der [Trigger-Ereignisliste](#triggers-overview).

`actions[].type` ist ein numerischer Enum aus der [Tool-Liste](#tools-overview).

`actions[].pending` ist `true`, wenn die Aktion zur [Freigabe](#approval-workflow) in die Warteschlange gestellt wurde, statt ausgeführt zu werden.

### `approval.requested`

`data`-Schema:

[inline-code-attrs-start title = 'Approval-Request-Datenschema'; type='json' inline-code-attrs-end]
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

Das **`args`**-Objekt enthält alles, was der LLM-Tool-Aufruf mitgebracht hat. Seine Struktur hängt vom Tool ab:

- Für `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Für `mark_comment_spam`: `{ commentId, isSpam }`.
- Für `write_comment`: `{ comment, urlId, parentId? }`.
- ...und so weiter.

Die Menge der Tool-Argumentstrukturen ist **kein stabiler öffentlicher Vertrag**. Tools können in Zukunft hinzugefügt werden und die Plattform reicht die Argumente unverändert weiter. Consumer sollten die Argumente als opaken Datenblock behandeln, es sei denn, sie verstehen das betreffende Tool explizit.

Der **`contextSnapshot`** erfasst den Kommentar-, Seiten- und Benutzerkontext, aus dem die Freigabe in die Warteschlange gestellt wurde. Seine Struktur entspricht der Kontextnachricht des Triggers.

### `approval.decided`

`data`-Schema:

[inline-code-attrs-start title = 'Approval-Decision-Datenschema'; type='json' inline-code-attrs-end]
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

### TenantAgentAction-Struktur

Innerhalb von `actions[]` in den Trigger-Payloads hat jede Aktion:

[inline-code-attrs-start title = 'TenantAgentAction-Schema'; type='json' inline-code-attrs-end]
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

Die `type`-Enum-Werte entsprechen `AgentActionType`:

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

`SEARCH_MEMORY` erscheint nicht in `actions[]`, da es schreibgeschützt und nicht auditiert ist.

### `triggerType`-Enum-Werte

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
- 15: `REPLAY` (intern; wird nicht an Webhooks zugestellt)

### Header

Jede Zustellung enthält:

- `X-FastComments-Agent-Event` - den kanonischen Ereignisnamen (`trigger.succeeded` usw.).
- `X-FastComments-Signature` - HMAC-SHA256 des Rohdaten-Body mit Ihrem API-Geheimnis. Siehe [Webhook-Signierung](#webhook-signing).

### Stabilität

Die Umschlagfelder und die dokumentierten `data`-Felder pro Ereignis sind Teil des öffentlichen Vertrags. Das Hinzufügen neuer optionaler Felder zu bestehenden Payloads ist erlaubt und gilt nicht als Breaking Change - Ihr Consumer sollte unbekannte Felder ignorieren. Die Struktur von `args` und `contextSnapshot` ist **nicht** Teil des Vertrags.
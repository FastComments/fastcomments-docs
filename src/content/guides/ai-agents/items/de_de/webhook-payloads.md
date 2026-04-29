Alle Agent-Webhook-Payloads teilen eine gemeinsame Envelope und fügen einen ereignisspezifischen `data`-Block hinzu. Diese Seite listet das vollständige Schema für jedes auf.

### Envelope (every event)

Jede Payload, unabhängig vom Ereignistyp, hat diese Top-Level-Felder:

[inline-code-attrs-start title = 'Webhook-Envelope-Schema'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - die für diese Zustellung übereinstimmende Domain",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - ISO 8601 Zeitstempel",
  "data": { /* ereignisspezifisch, siehe unten */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data`-Schema:

[inline-code-attrs-start title = 'Trigger-Ereignis-Daten-Schema'; type='json' inline-code-attrs-end]
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
  "errorMessage": "string - vorhanden bei trigger.failed",
  "url": "string - optional",
  "urlId": "string - optional",
  "commentId": "string - optional"
}
[inline-code-end]

`triggerType` ist ein numerisches Enum aus der [trigger event list](#triggers-overview).

`actions[].type` ist ein numerisches Enum aus der [tool list](#tools-overview).

`actions[].pending` ist `true`, wenn die Aktion zur [Genehmigung](#approval-workflow) in die Warteschlange gestellt wurde, statt ausgeführt zu werden.

### `approval.requested`

`data`-Schema:

[inline-code-attrs-start title = 'Daten-Schema für angeforderte Genehmigung'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* pro Tool, siehe unten */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - optional, Begründung des Agents",
  "confidence": 0.85,
  "contextSnapshot": { /* der Kommentar-/Seitenkontext, auf den sich die Genehmigung bezieht */ }
}
[inline-code-end]

Das **`args`**-Objekt enthält genau die Daten, die der LLM-Tool-Aufruf getragen hat. Seine Form hängt vom Tool ab:

- Für `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Für `mark_comment_spam`: `{ commentId, isSpam }`.
- Für `write_comment`: `{ comment, urlId, parentId? }`.
- ...und so weiter.

Die Menge der Tool-Argument-Formen ist **kein stabiler öffentlicher Vertrag**. Tools können in Zukunft hinzugefügt werden und die Plattform leitet args unverändert weiter. Empfänger sollten args als undurchsichtiges Blob behandeln, es sei denn, sie verstehen das beteiligte Tool explizit.

Der **`contextSnapshot`** erfasst den Kommentar-, Seiten- und Benutzerkontext, aus dem die Genehmigung angefordert wurde. Seine Form spiegelt die Kontextnachricht des Triggers wider.

### `approval.decided`

`data`-Schema:

[inline-code-attrs-start title = 'Daten-Schema für Entscheidung zur Genehmigung'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - die userId des Moderators, der entschieden hat",
  "decidedAt": "string - ISO 8601 - optional, nur vorhanden nach Entscheidung",
  "executedAt": "string - ISO 8601 - vorhanden, wenn APPROVED und die Ausführung beendet wurde",
  "executionResult": "string - Ergebnisnachricht des Ausführers - vorhanden nach der Ausführung",
  "contextSnapshot": { /* wie bei approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

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

Die `type`-Enum-Werte stimmen mit `AgentActionType` überein:

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

`SEARCH_MEMORY` erscheint nicht in `actions[]`, weil es schreibgeschützt und nicht auditiert ist.

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
- 15: `REPLAY` (intern; wird nicht an Webhooks geliefert)

### Headers

Jede Zustellung enthält:

- `X-FastComments-Agent-Event` - den kanonischen Ereignisnamen (`trigger.succeeded`, usw.).
- `X-FastComments-Signature` - HMAC-SHA256 des rohen Bodys unter Verwendung Ihres API-Secrets. Siehe [Webhook-Signierung](#webhook-signing).

### Stability

Die Envelope-Felder und die dokumentierten `data`-Felder pro Ereignis sind Teil des öffentlichen Vertrags. Das Hinzufügen neuer optionaler Felder zu bestehenden Payloads ist erlaubt und gilt nicht als Breaking Change – Ihr Empfänger sollte unbekannte Felder ignorieren. Die Form von `args` und `contextSnapshot` ist **nicht** Teil des Vertrags.

---
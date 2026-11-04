Tutti i payload dei webhook degli agent condividono una busta comune e aggiungono un blocco `data` specifico per l'evento. Questa pagina elenca lo schema completo per ciascuno.

### Busta (ogni evento)

Ogni payload, indipendentemente dal tipo di evento, ha questi campi di primo livello:

[inline-code-attrs-start title = 'Schema della busta del webhook'; type='json' inline-code-attrs-end]
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

Schema di `data`:

[inline-code-attrs-start title = 'Schema dei dati dell\'evento trigger'; type='json' inline-code-attrs-end]
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

`triggerType` è un enum numerico dell'[elenco degli eventi trigger](#triggers-overview).

`actions[].type` è un enum numerico dell'[elenco degli strumenti](#tools-overview).

`actions[].pending` è `true` quando l'azione è stata messa in coda per l'[approvazione](#approval-workflow) invece di essere eseguita.

### `approval.requested`

Schema di `data`:

[inline-code-attrs-start title = 'Schema dei dati di approval.requested'; type='json' inline-code-attrs-end]
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

L'oggetto **`args`** è ciò che la chiamata allo strumento LLM ha trasportato. La sua forma dipende dallo strumento:

- Per `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Per `mark_comment_spam`: `{ commentId, isSpam }`.
- Per `write_comment`: `{ comment, urlId, parentId? }`.
- ...e così via.

L'insieme delle forme degli argomenti degli strumenti **non è un contratto pubblico stabile**. Gli strumenti possono essere aggiunti in futuro e la piattaforma passa gli argomenti così come sono. I consumatori dovrebbero trattare gli args come un blob opaco a meno che non comprendano esplicitamente lo strumento coinvolto.

**`contextSnapshot`** cattura il contesto di commento, pagina e utente da cui l'approvazione è stata messa in coda. La sua forma rispecchia il messaggio di contesto del trigger.

### `approval.decided`

Schema di `data`:

[inline-code-attrs-start title = 'Schema dei dati di approval.decided'; type='json' inline-code-attrs-end]
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

### Forma di TenantAgentAction

All'interno di `actions[]` nei payload dei trigger, ogni azione ha:

[inline-code-attrs-start title = 'Schema di TenantAgentAction'; type='json' inline-code-attrs-end]
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

I valori dell'enum `type` corrispondono a `AgentActionType`:

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

`SEARCH_MEMORY` non compare in `actions[]` perché è di sola lettura e non auditato.

### Valori dell'enum `triggerType`

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
- 15: `REPLAY` (interno; non consegnato ai webhook)

### Header

Ogni consegna include:

- `X-FastComments-Agent-Event` - il nome canonico dell'evento (`trigger.succeeded`, ecc.).
- `X-FastComments-Signature` - HMAC-SHA256 del corpo grezzo utilizzando il tuo segreto API. Vedi [Firma dei webhook](#webhook-signing).

### Stabilità

I campi della busta e i campi `data` documentati per ogni evento fanno parte del contratto pubblico. L'aggiunta di nuovi campi opzionali ai payload esistenti è consentita e non è considerata una modifica breaking - il tuo consumatore dovrebbe ignorare i campi sconosciuti. La forma di `args` e `contextSnapshot` **non** fa parte del contratto.
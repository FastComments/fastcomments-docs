Tutti i payload dei webhook degli agent condividono un involucro comune e aggiungono un blocco `data` specifico per l'evento. Questa pagina elenca lo schema completo per ciascuno.

### Involucro (ogni evento)

Ogni payload, indipendentemente dal tipo di evento, ha questi campi di primo livello:

[inline-code-attrs-start title = 'Schema dell\'involucro webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - il dominio corrispondente per questa consegna",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - timestamp ISO 8601",
  "data": { /* specifico per l'evento, vedi sotto */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

Lo schema di `data`:

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
      "commentId": "string - opzionale",
      "userId": "string - opzionale",
      "badgeId": "string - opzionale",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - presente in trigger.failed",
  "url": "string - opzionale",
  "urlId": "string - opzionale",
  "commentId": "string - opzionale"
}
[inline-code-end]

`triggerType` è un enum numerico dall'[elenco degli eventi trigger](#triggers-overview).

`actions[].type` è un enum numerico dalla [lista degli strumenti](#tools-overview).

`actions[].pending` è `true` quando l'azione è stata messa in coda per [approvazione](#approval-workflow) invece di essere eseguita.

### `approval.requested`

Lo schema di `data`:

[inline-code-attrs-start title = 'Schema dei dati di richiesta di approvazione'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* per strumento, vedi sotto */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - opzionale, ragionamento dell'agente",
  "confidence": 0.85,
  "contextSnapshot": { /* il contesto del commento/pagina a cui si riferisce la richiesta di approvazione */ }
}
[inline-code-end]

L'oggetto **`args`** è qualunque cosa la chiamata allo strumento LLM abbia trasportato. La sua forma dipende dallo strumento:

- Per `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Per `mark_comment_spam`: `{ commentId, isSpam }`.
- Per `write_comment`: `{ comment, urlId, parentId? }`.
- ...e così via.

L'insieme delle forme degli argomenti degli strumenti **non è un contratto pubblico stabile**. Possono essere aggiunti strumenti in futuro e la piattaforma passa `args` così come sono. I consumatori dovrebbero trattare `args` come un blob opaco a meno che non comprendano esplicitamente lo strumento coinvolto.

Il **`contextSnapshot`** cattura il contesto del commento, della pagina e dell'utente da cui è stata messa in coda l'approvazione. La sua struttura rispecchia il messaggio di contesto del trigger.

### `approval.decided`

Lo schema di `data`:

[inline-code-attrs-start title = 'Schema dei dati della decisione di approvazione'; type='json' inline-code-attrs-end]
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

All'interno di `actions[]` nei payload dei trigger, ogni azione ha:

[inline-code-attrs-start title = 'Schema TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - opzionale",
  "userId": "string - opzionale",
  "badgeId": "string - opzionale",
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

`SEARCH_MEMORY` non appare in `actions[]` perché è di sola lettura e non è tracciato/auditato.

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
- 15: `REPLAY` (interno; non inviato ai webhook)

### Headers

Ogni invio include:

- `X-FastComments-Agent-Event` - il nome canonico dell'evento (`trigger.succeeded`, ecc.).
- `X-FastComments-Signature` - HMAC-SHA256 del corpo grezzo usando il tuo secret API. Vedi [Webhook Signing](#webhook-signing).

### Stabilità

I campi dell'involucro e i campi `data` documentati per evento fanno parte del contratto pubblico. Aggiungere nuovi campi opzionali ai payload esistenti è consentito e non è considerato una breaking change: il tuo consumer dovrebbe ignorare i campi sconosciuti. La forma di `args` e `contextSnapshot` **non** fa parte del contratto.
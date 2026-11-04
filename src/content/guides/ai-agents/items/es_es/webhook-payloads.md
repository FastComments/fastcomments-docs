Todos los payloads de webhook de agente comparten un sobre común y añaden un bloque `data` específico del evento. Esta página lista el esquema completo de cada uno.

### Sobre (todos los eventos)

Cada payload, independientemente del tipo de evento, tiene estos campos de nivel superior:

[inline-code-attrs-start title = 'Esquema del Sobre de Webhook'; type='json' inline-code-attrs-end]
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

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de Datos del Evento Trigger'; type='json' inline-code-attrs-end]
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

`triggerType` es un enum numérico de la [lista de eventos de trigger](#triggers-overview).

`actions[].type` es un enum numérico de la [lista de herramientas](#tools-overview).

`actions[].pending` es `true` cuando la acción se puso en cola para [aprobación](#approval-workflow) en lugar de ejecutarse.

### `approval.requested`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de Datos de Approval Requested'; type='json' inline-code-attrs-end]
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

El objeto **`args`** es lo que la llamada a la herramienta del LLM llevó. Su forma depende de la herramienta:

- Para `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Para `mark_comment_spam`: `{ commentId, isSpam }`.
- Para `write_comment`: `{ comment, urlId, parentId? }`.
- ...y así sucesivamente.

El conjunto de formas de argumentos de las herramientas **no es un contrato público estable**. Se pueden añadir herramientas en el futuro y la plataforma pasa los args tal cual. Los consumidores deben tratar los args como un blob opaco a menos que entiendan explícitamente la herramienta involucrada.

El **`contextSnapshot`** captura el contexto de comentario, página y usuario desde el que se puso en cola la aprobación. Su forma refleja el mensaje de contexto del trigger.

### `approval.decided`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de Datos de Approval Decided'; type='json' inline-code-attrs-end]
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

### Forma de TenantAgentAction

Dentro de `actions[]` en los payloads de trigger, cada acción tiene:

[inline-code-attrs-start title = 'Esquema de TenantAgentAction'; type='json' inline-code-attrs-end]
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

Los valores del enum `type` coinciden con `AgentActionType`:

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

`SEARCH_MEMORY` no aparece en `actions[]` porque es de solo lectura y no se audita.

### Valores del enum `triggerType`

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
- 15: `REPLAY` (interno; no se entrega a los webhooks)

### Cabeceras

Cada entrega incluye:

- `X-FastComments-Agent-Event` - el nombre canónico del evento (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 del cuerpo sin procesar usando tu secreto de API. Ver [Firma de Webhooks](#webhook-signing).

### Estabilidad

Los campos del sobre y los campos `data` documentados por evento forman parte del contrato público. Añadir nuevos campos opcionales a los payloads existentes está permitido y no se considera un cambio disruptivo - tu consumidor debe ignorar los campos desconocidos. La forma de `args` y `contextSnapshot` **no** forma parte del contrato.
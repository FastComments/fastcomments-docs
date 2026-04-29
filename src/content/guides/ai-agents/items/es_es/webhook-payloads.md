---
Todos los payloads de webhook del agente comparten un envoltorio común y agregan un bloque `data` específico del evento. Esta página enumera el esquema completo para cada uno.

### Envoltorio (cada evento)

Cada payload, independientemente del tipo de evento, tiene estos campos de nivel superior:

[inline-code-attrs-start title = 'Esquema del envoltorio del webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - el dominio coincidente para esta entrega",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - marca de tiempo ISO 8601",
  "data": { /* específico del evento, ver abajo */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de datos del evento Trigger'; type='json' inline-code-attrs-end]
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
      "commentId": "string - opcional",
      "userId": "string - opcional",
      "badgeId": "string - opcional",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - presente en trigger.failed",
  "url": "string - opcional",
  "urlId": "string - opcional",
  "commentId": "string - opcional"
}
[inline-code-end]

`triggerType` es un enum numérico de la [lista de eventos trigger](#triggers-overview).

`actions[].type` es un enum numérico de la [lista de herramientas](#tools-overview).

`actions[].pending` es `true` cuando la acción fue encolada para [aprobación](#approval-workflow) en lugar de ejecutada.

### `approval.requested`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de datos de solicitud de aprobación'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* por herramienta, ver abajo */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - opcional, razonamiento del agente",
  "confidence": 0.85,
  "contextSnapshot": { /* el contexto del comentario/página sobre el que trata la aprobación */ }
}
[inline-code-end]

El objeto **`args`** es lo que haya llevado la llamada a la herramienta LLM. Su forma depende de la herramienta:

- For `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- For `mark_comment_spam`: `{ commentId, isSpam }`.
- For `write_comment`: `{ comment, urlId, parentId? }`.
- ...and so on.

El conjunto de formas de argumentos de las herramientas **no es un contrato público estable**. Se pueden añadir herramientas en el futuro y la plataforma pasa `args` tal cual. Los consumidores deben tratar `args` como un blob opaco a menos que entiendan explícitamente la herramienta implicada.

El **`contextSnapshot`** captura el contexto del comentario, la página y el usuario desde el que se encoló la aprobación. Su forma refleja el mensaje de contexto del trigger.

### `approval.decided`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de datos de decisión de aprobación'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - el userId del moderador que decidió",
  "decidedAt": "string - ISO 8601 - opcional, presente solo una vez decidido",
  "executedAt": "string - ISO 8601 - presente cuando APPROVED y la ejecución haya finalizado",
  "executionResult": "string - mensaje de resultado del ejecutor - presente después de la ejecución",
  "contextSnapshot": { /* mismo que approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'Esquema de TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - opcional",
  "userId": "string - opcional",
  "badgeId": "string - opcional",
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

`SEARCH_MEMORY` no aparece en `actions[]` porque es de solo lectura y no auditado.

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

### Encabezados

Cada entrega incluye:

- `X-FastComments-Agent-Event` - el nombre canónico del evento (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 del cuerpo bruto usando tu secreto de API. Ver [Firma del webhook](#webhook-signing).

### Estabilidad

Los campos del envoltorio y los campos documentados de `data` por evento son parte del contrato público. Añadir nuevos campos opcionales a los payloads existentes está permitido y no se considera un cambio incompatible: tu consumidor debe ignorar los campos desconocidos. La forma de `args` y `contextSnapshot` **no** forma parte del contrato.

---
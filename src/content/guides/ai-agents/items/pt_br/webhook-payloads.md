Todos os payloads de webhook de agente compartilham um envelope comum e adicionam um bloco `data` específico do evento. Esta página lista o schema completo de cada um.

### Envelope (todos os eventos)

Cada payload, independentemente do tipo de evento, possui estes campos de nível superior:

[inline-code-attrs-start title = 'Schema do Envelope do Webhook'; type='json' inline-code-attrs-end]
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

Schema do `data`:

[inline-code-attrs-start title = 'Schema dos Dados do Evento de Trigger'; type='json' inline-code-attrs-end]
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

`triggerType` é um enum numérico da [lista de eventos de trigger](#triggers-overview).

`actions[].type` é um enum numérico da [lista de ferramentas](#tools-overview).

`actions[].pending` é `true` quando a ação foi colocada na fila para [aprovação](#approval-workflow) em vez de executada.

### `approval.requested`

Schema do `data`:

[inline-code-attrs-start title = 'Schema dos Dados de Aprovação Solicitada'; type='json' inline-code-attrs-end]
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

O objeto **`args`** é o que a chamada de ferramenta do LLM carregou. Sua forma depende da ferramenta:

- Para `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Para `mark_comment_spam`: `{ commentId, isSpam }`.
- Para `write_comment`: `{ comment, urlId, parentId? }`.
- ...e assim por diante.

O conjunto de formas de argumentos das ferramentas **não é um contrato público estável**. Ferramentas podem ser adicionadas no futuro e a plataforma repassa os args verbatim. Os consumidores devem tratar os args como um blob opaco, a menos que entendam explicitamente a ferramenta envolvida.

O **`contextSnapshot`** captura o contexto de comentário, página e usuário do qual a aprovação foi enfileirada. Sua forma espelha a mensagem de contexto do trigger.

### `approval.decided`

Schema do `data`:

[inline-code-attrs-start title = 'Schema dos Dados de Aprovação Decidida'; type='json' inline-code-attrs-end]
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

### Forma do TenantAgentAction

Dentro de `actions[]` nos payloads de trigger, cada ação possui:

[inline-code-attrs-start title = 'Schema do TenantAgentAction'; type='json' inline-code-attrs-end]
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

Os valores do enum `type` correspondem a `AgentActionType`:

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

`SEARCH_MEMORY` não aparece em `actions[]` porque é somente leitura e não auditado.

### Valores do enum `triggerType`

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
- 15: `REPLAY` (interno; não entregue aos webhooks)

### Headers

Cada entrega inclui:

- `X-FastComments-Agent-Event` - o nome canônico do evento (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 do corpo bruto usando seu segredo de API. Veja [Assinatura de Webhook](#webhook-signing).

### Estabilidade

Os campos do envelope e os campos `data` documentados por evento fazem parte do contrato público. Adicionar novos campos opcionais a payloads existentes é permitido e não é considerado uma mudança disruptiva - seu consumidor deve ignorar campos desconhecidos. A forma de `args` e `contextSnapshot` **não** faz parte do contrato.
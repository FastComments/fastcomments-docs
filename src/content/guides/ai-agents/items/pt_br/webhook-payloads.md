Todos os payloads de webhook do agente compartilham um envelope comum e adicionam um bloco `data` específico do evento. Esta página lista o esquema completo para cada um.

### Envelope (every event)

Todo payload, independentemente do tipo de evento, possui estes campos de nível superior:

[inline-code-attrs-start title = 'Esquema do Envelope do Webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - o domínio correspondente para esta entrega",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - timestamp ISO 8601",
  "data": { /* event-specific, see below */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de Dados do Evento de Trigger'; type='json' inline-code-attrs-end]
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
  "errorMessage": "string - presente em trigger.failed",
  "url": "string - opcional",
  "urlId": "string - opcional",
  "commentId": "string - opcional"
}
[inline-code-end]

`triggerType` é um enum numérico da [lista de eventos de trigger](#triggers-overview).

`actions[].type` é um enum numérico da [lista de ferramentas](#tools-overview).

`actions[].pending` é `true` quando a ação foi enfileirada para [approval](#approval-workflow) em vez de executada.

### `approval.requested`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de Dados de approval.requested'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* por ferramenta, veja abaixo */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - opcional, raciocínio do agente",
  "confidence": 0.85,
  "contextSnapshot": { /* o contexto do comentário/página ao qual a aprovação se refere */ }
}
[inline-code-end]

O objeto **`args`** é o que a chamada da ferramenta LLM carregou. Sua forma depende da ferramenta:

- Para `ban_user`: `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Para `mark_comment_spam`: `{ commentId, isSpam }`.
- Para `write_comment`: `{ comment, urlId, parentId? }`.
- ...e assim por diante.

O conjunto de formas de argumentos das ferramentas **não é um contrato público estável**. Ferramentas podem ser adicionadas no futuro e a plataforma passa args de forma literal. Os consumidores devem tratar args como um blob opaco, a menos que compreendam explicitamente a ferramenta envolvida.

O **`contextSnapshot`** captura o contexto do comentário, da página e do usuário de onde a aprovação foi enfileirada. Sua forma espelha a mensagem de contexto do trigger.

### `approval.decided`

Esquema de `data`:

[inline-code-attrs-start title = 'Esquema de Dados de approval.decided'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - o userId do moderador que decidiu",
  "decidedAt": "string - ISO 8601 - opcional, presente somente após a decisão",
  "executedAt": "string - ISO 8601 - presente quando APPROVED e a execução terminou",
  "executionResult": "string - mensagem de resultado do executor - presente após a execução",
  "contextSnapshot": { /* igual a approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Dentro de `actions[]` nos payloads de trigger, cada ação possui:

[inline-code-attrs-start title = 'Esquema do TenantAgentAction'; type='json' inline-code-attrs-end]
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

`type` valores do enum correspondem a `AgentActionType`:

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
- 15: `REPLAY` (interno; não entregue aos webhooks)

### Headers

Cada entrega inclui:

- `X-FastComments-Agent-Event` - o nome canônico do evento (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 do corpo bruto usando seu segredo de API. Veja [Assinatura de Webhook](#webhook-signing).

### Stability

Os campos do envelope e os campos `data` documentados por evento fazem parte do contrato público. Adicionar novos campos opcionais aos payloads existentes é permitido e não é considerado uma mudança quebras - seu consumidor deve ignorar campos desconhecidos. A forma de `args` e `contextSnapshot` **não** faz parte do contrato.
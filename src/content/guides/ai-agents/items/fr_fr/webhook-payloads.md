Tous les payloads de webhook d'agent partagent une enveloppe commune et ajoutent un bloc `data` spécifique à l'événement. Cette page liste le schéma complet pour chacun.

### Enveloppe (chaque événement)

Chaque payload, quel que soit le type d'événement, possède ces champs de premier niveau :

[inline-code-attrs-start title = 'Schéma de l\'enveloppe du webhook'; type='json' inline-code-attrs-end]
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

Schéma de `data` :

[inline-code-attrs-start title = 'Schéma des données de l\'événement Trigger'; type='json' inline-code-attrs-end]
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

`triggerType` est une énumération numérique issue de la [liste des événements de déclenchement](#triggers-overview).

`actions[].type` est une énumération numérique issue de la [liste des outils](#tools-overview).

`actions[].pending` vaut `true` lorsque l'action a été mise en file pour [approbation](#approval-workflow) au lieu d'être exécutée.

### `approval.requested`

Schéma de `data` :

[inline-code-attrs-start title = 'Schéma des données d\'approbation demandée'; type='json' inline-code-attrs-end]
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

L'objet **`args`** contient ce que l'appel d'outil du LLM a transmis. Sa forme dépend de l'outil :

- Pour `ban_user` : `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Pour `mark_comment_spam` : `{ commentId, isSpam }`.
- Pour `write_comment` : `{ comment, urlId, parentId? }`.
- ...et ainsi de suite.

L'ensemble des formes d'arguments des outils **n'est pas un contrat public stable**. Des outils peuvent être ajoutés à l'avenir et la plateforme transmet les args tels quels. Les consommateurs doivent traiter les args comme un blob opaque, sauf s'ils comprennent explicitement l'outil concerné.

Le **`contextSnapshot`** capture le contexte du commentaire, de la page et de l'utilisateur à partir duquel l'approbation a été mise en file. Sa forme reflète le message de contexte du déclencheur.

### `approval.decided`

Schéma de `data` :

[inline-code-attrs-start title = 'Schéma des données d\'approbation décidée'; type='json' inline-code-attrs-end]
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

### Forme de TenantAgentAction

Dans `actions[]` des payloads de déclenchement, chaque action possède :

[inline-code-attrs-start title = 'Schéma TenantAgentAction'; type='json' inline-code-attrs-end]
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

Les valeurs de l'énumération `type` correspondent à `AgentActionType` :

- 0 : `WRITE_COMMENT`
- 1 : `VOTE_COMMENT`
- 2 : `PIN_COMMENT`
- 3 : `UNPIN_COMMENT`
- 4 : `LOCK_COMMENT`
- 5 : `UNLOCK_COMMENT`
- 6 : `MARK_COMMENT_REVIEWED`
- 7 : `MARK_COMMENT_APPROVED`
- 8 : `MARK_COMMENT_SPAM`
- 9 : `AWARDED_BADGE`
- 10 : `BAN_USER`
- 11 : `SENT_EMAIL`
- 12 : `WARNED_USER`
- 13 : `SAVED_MEMORY`

`SEARCH_MEMORY` n'apparaît pas dans `actions[]` car il est en lecture seule et non audité.

### Valeurs de l'énumération `triggerType`

`AgentTriggerReasonType` :

- 0 : `COMMENT_ADD`
- 1 : `COMMENT_EDIT`
- 2 : `COMMENT_DELETE`
- 3 : `COMMENT_PIN`
- 4 : `COMMENT_UNPIN`
- 5 : `COMMENT_LOCK`
- 6 : `COMMENT_UNLOCK`
- 7 : `COMMENT_VOTE_THRESHOLD`
- 8 : `MODERATOR_REVIEWED_COMMENT`
- 9 : `MODERATOR_APPROVED_COMMENT`
- 10 : `MODERATOR_SPAMMED_COMMENT`
- 11 : `MODERATOR_AWARDED_BADGE`
- 12 : `COMMENT_FLAG_THRESHOLD`
- 13 : `NEW_USER_FIRST_COMMENT`
- 14 : `COMMENT_AUTO_SPAMMED`
- 15 : `REPLAY` (interne ; non livré aux webhooks)

### En-têtes

Chaque livraison inclut :

- `X-FastComments-Agent-Event` - le nom canonique de l'événement (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 du corps brut en utilisant votre secret d'API. Voir [Signature des webhooks](#webhook-signing).

### Stabilité

Les champs de l'enveloppe et les champs `data` documentés par événement font partie du contrat public. L'ajout de nouveaux champs optionnels aux payloads existants est autorisé et n'est pas considéré comme un changement cassant - votre consommateur doit ignorer les champs inconnus. La forme de `args` et de `contextSnapshot` ne fait **pas** partie du contrat.
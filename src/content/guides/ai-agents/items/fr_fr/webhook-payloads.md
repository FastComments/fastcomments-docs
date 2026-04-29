Tous les payloads d'agent webhook partagent une enveloppe commune et ajoutent un bloc `data` spécifique à l'événement. Cette page liste le schéma complet pour chacun.

### Enveloppe (chaque événement)

Chaque payload, quel que soit le type d'événement, possède ces champs au niveau supérieur :

[inline-code-attrs-start title = 'Schéma de l\'enveloppe Webhook'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "event": "trigger.succeeded | trigger.failed | approval.requested | approval.decided",
  "eventType": 0 | 1 | 2 | 3,
  "tenantId": "string",
  "domain": "string - le domaine correspondant à cette livraison",
  "agentId": "string",
  "agentInternalName": "string",
  "agentDisplayName": "string",
  "occurredAt": "string - horodatage ISO 8601",
  "data": { /* spécifique à l'événement, voir ci-dessous */ }
}
[inline-code-end]

### `trigger.succeeded` / `trigger.failed`

`data` schéma:

[inline-code-attrs-start title = 'Schéma des données d\'événement Trigger'; type='json' inline-code-attrs-end]
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
      "commentId": "string - optionnel",
      "userId": "string - optionnel",
      "badgeId": "string - optionnel",
      "pending": false,
      "justification": "string",
      "confidence": 0.92
    }
  ],
  "errorMessage": "string - présent dans trigger.failed",
  "url": "string - optionnel",
  "urlId": "string - optionnel",
  "commentId": "string - optionnel"
}
[inline-code-end]

`triggerType` est un enum numérique issu de la [liste des événements de trigger](#triggers-overview).

`actions[].type` est un enum numérique issu de la [liste des outils](#tools-overview).

`actions[].pending` vaut `true` lorsque l'action a été mise en file d'attente pour [approbation](#approval-workflow) plutôt que exécutée.

### `approval.requested`

`data` schéma:

[inline-code-attrs-start title = 'Schéma des données de demande d\'approbation'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "PENDING",
  "args": { /* par outil, voir ci-dessous */ },
  "createdAt": "string - ISO 8601",
  "justification": "string - optionnel, raisonnement de l'agent",
  "confidence": 0.85,
  "contextSnapshot": { /* le contexte du commentaire/page concerné par la demande d'approbation */ }
}
[inline-code-end]

L'objet **`args`** contient ce que l'appel d'outil LLM a transmis. Sa forme dépend de l'outil :

- Pour `ban_user` : `{ userId, commentId, duration, shadowBan, deleteAllUsersComments?, banIP? }`.
- Pour `mark_comment_spam` : `{ commentId, isSpam }`.
- Pour `write_comment` : `{ comment, urlId, parentId? }`.
- ...et ainsi de suite.

L'ensemble des schémas d'arguments d'outils **n'est pas un contrat public stable**. Des outils peuvent être ajoutés à l'avenir et la plateforme transmet les args tels quels. Les consommateurs doivent traiter les args comme un blob opaque à moins qu'ils ne comprennent explicitement l'outil concerné.

Le **`contextSnapshot`** capture le contexte du commentaire, de la page et de l'utilisateur depuis lequel la demande d'approbation a été mise en file. Sa forme reflète le message de contexte du trigger.

### `approval.decided`

`data` schéma:

[inline-code-attrs-start title = 'Schéma des données de décision d\'approbation'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "approvalId": "string",
  "triggerId": "string",
  "toolName": "ban_user | mark_comment_spam | ...",
  "actionType": 10,
  "status": "APPROVED | REJECTED | EXECUTION_FAILED",
  "decidedBy": "string - l'userId du modérateur ayant pris la décision",
  "decidedAt": "string - ISO 8601 - optionnel, présent seulement une fois décidé",
  "executedAt": "string - ISO 8601 - présent lorsque APPROVED et que l'exécution est terminée",
  "executionResult": "string - message de résultat de l'exécuteur - présent après l'exécution",
  "contextSnapshot": { /* idem que approval.requested */ }
}
[inline-code-end]

### TenantAgentAction shape

Inside `actions[]` on the trigger payloads, each action has:

[inline-code-attrs-start title = 'Schéma TenantAgentAction'; type='json' inline-code-attrs-end]
[inline-code-start]
{
  "type": 0,
  "commentId": "string - optionnel",
  "userId": "string - optionnel",
  "badgeId": "string - optionnel",
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

`SEARCH_MEMORY` does not appear in `actions[]` because it is read-only and unaudited.

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
- 15: `REPLAY` (interne ; non livré aux webhooks)

### En-têtes

Chaque livraison inclut :

- `X-FastComments-Agent-Event` - le nom canonique de l'événement (`trigger.succeeded`, etc.).
- `X-FastComments-Signature` - HMAC-SHA256 du corps brut en utilisant votre secret API. Voir [Signature des webhooks](#webhook-signing).

### Stabilité

Les champs de l'enveloppe et les champs `data` documentés par événement font partie du contrat public. L'ajout de nouveaux champs optionnels aux payloads existants est autorisé et n'est pas considéré comme un changement incompatible — votre consommateur doit ignorer les champs inconnus. La forme de `args` et `contextSnapshot` **ne** fait **pas** partie du contrat.
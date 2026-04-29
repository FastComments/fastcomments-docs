Il existe quatre types d'événements webhook d'agent. Chaque événement possède une valeur d'énumération numérique (utilisée dans les payloads) et un nom canonique sous forme de chaîne (utilisé dans le champ d'enveloppe `event` et dans l'en-tête HTTP `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | An agent run completes with status `SUCCESS`. |
| `trigger.failed` | 1 | An agent run completes with status `ERROR`. |
| `approval.requested` | 2 | An approval is queued in `PENDING` state. |
| `approval.decided` | 3 | An approval transitions to `APPROVED`, `REJECTED`, or `EXECUTION_FAILED`. |

### `trigger.succeeded`

Se déclenche après que l'exécution de l'agent se termine sans erreur. Le champ `data` du payload inclut :

- `triggerId` - l'ID unique de l'exécution.
- `triggerType` - l'[énumération des raisons de déclenchement](#triggers-overview) qui a démarré l'exécution.
- `status` - `SUCCESS` (string).
- `tokensUsed` - jetons consommés lors de cette exécution.
- `wasDryRun` - true si l'agent était en [dry-run mode](#dry-run-mode).
- `actions` - tableau d'enregistrements `TenantAgentAction` (voir [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - si le trigger en disposait.

Si l'exécution n'a effectué aucune action, le tableau `actions` est vide - il s'agit d'une exécution réussie "the agent decided to do nothing", ce qui est utile à savoir.

### `trigger.failed`

Se déclenche lorsqu'une exécution rencontre une erreur. Même format de payload que `trigger.succeeded`, avec `status: 'ERROR'` et un champ additionnel `errorMessage` décrivant ce qui a échoué. Les erreurs possibles incluent des échecs d'appel LLM, des échecs d'envoi vers des outils, et une panne de budget en cours d'exécution.

`actions` peut encore contenir des entrées pour des appels d'outils qui se sont terminés avant l'erreur.

### `approval.requested`

Se déclenche au moment où une approbation est mise en file d'attente dans l'état `PENDING`. Le payload inclut :

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - les arguments de l'outil **transmis tels quels** depuis l'appel LLM. La structure dépend de l'outil et n'est pas un contrat public stable — le schéma peut changer à mesure que de nouveaux outils sont ajoutés.
- `createdAt`.
- `justification`, `confidence` - si l'agent les a fournis.
- `contextSnapshot` - le contexte du commentaire / de la page auquel l'approbation se rapporte.

Utile pour transférer des approbations en attente vers un canal de chat ops : un bot Slack abonné à `approval.requested` peut poster l'action et le raisonnement dans un canal de modération pour une revue rapide.

### `approval.decided`

Se déclenche lorsqu'une approbation sort de l'état `PENDING`. Le payload inclut :

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ou `EXECUTION_FAILED`.
- `decidedBy` - l'ID utilisateur du modérateur ayant pris la décision.
- `decidedAt` - moment de la décision.
- `executedAt` - si APPROVED, moment où la plateforme a exécuté l'action approuvée.
- `executionResult` - si APPROVED, une chaîne décrivant le résultat de l'exécuteur.
- `contextSnapshot` - le contexte du commentaire / de la page.

Cet événement couvre tous les résultats de décision :

- **Approved + executed cleanly** -> `status: APPROVED`, `executedAt` renseigné, `executionResult` contient le message de succès.
- **Approved + executor failed** -> `status: EXECUTION_FAILED`, `executedAt` renseigné, `executionResult` décrit l'échec.
- **Rejected** -> `status: REJECTED`, `executedAt` est null, `executionResult` est null.

### Header

Chaque livraison inclut un en-tête HTTP `X-FastComments-Agent-Event` avec le nom canonique de l'événement sous forme de chaîne (`trigger.succeeded`, etc.). Utile si votre endpoint est une URL unique gérant plusieurs types d'événements.

### See also

- [Webhook Payloads](#webhook-payloads) pour les schémas complets de payload par événement.
- [Webhook Signing](#webhook-signing) pour le schéma HMAC.
- [Webhook Retries](#webhook-retries) pour la sémantique de livraison.
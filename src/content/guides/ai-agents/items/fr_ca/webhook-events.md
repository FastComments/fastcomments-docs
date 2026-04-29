Il y a quatre types d'événements webhook d'agent. Chaque événement a une valeur d'énumération numérique (utilisée dans les payloads) et un nom canonique sous forme de chaîne (utilisé dans le champ d'enveloppe `event` et dans l'en-tête HTTP `X-FastComments-Agent-Event`).

| Event name | Enum | Fires when |
|---|---|---|
| `trigger.succeeded` | 0 | Un run d'agent se termine avec le statut `SUCCESS`. |
| `trigger.failed` | 1 | Un run d'agent se termine avec le statut `ERROR`. |
| `approval.requested` | 2 | Une approbation est mise en file d'attente en état `PENDING`. |
| `approval.decided` | 3 | Une approbation passe à `APPROVED`, `REJECTED`, ou `EXECUTION_FAILED`. |

### `trigger.succeeded`

Se déclenche après que le run de l'agent se termine sans erreur. Le champ `data` du payload inclut :

- `triggerId` - l'ID unique du run.
- `triggerType` - le [trigger reason enum](#triggers-overview) qui a démarré le run.
- `status` - `SUCCESS` (chaîne).
- `tokensUsed` - jetons consommés durant ce run.
- `wasDryRun` - true si l'agent était en [dry-run mode](#dry-run-mode).
- `actions` - tableau d'enregistrements `TenantAgentAction` (voir [Webhook Payloads](#webhook-payloads)).
- `commentId`, `url`, `urlId` - si le trigger en disposait.

Si le run n'a effectué aucune action, le tableau `actions` est vide - il s'agit d'un run réussi « l'agent a décidé de ne rien faire », ce qui est utile à savoir.

### `trigger.failed`

Se déclenche lorsqu'un run rencontre une erreur. Même structure de payload que `trigger.succeeded`, avec `status: 'ERROR'` et un champ additionnel `errorMessage` décrivant ce qui a mal tourné. Les erreurs possibles incluent des échecs d'appel LLM, des échecs d'envoi d'outil, et l'épuisement du budget en cours de run.

`actions` peut encore contenir des entrées pour des appels d'outils qui se sont terminés avant l'erreur.

### `approval.requested`

Se déclenche au moment où une approbation est mise en file d'attente en état `PENDING`. Le payload inclut :

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - les arguments de l'outil **transmis tels quels** depuis l'appel LLM. La forme dépend de l'outil et n'est pas un contrat public stable - le schéma peut changer à mesure que de nouveaux outils sont ajoutés.
- `createdAt`.
- `justification`, `confidence` - si l'agent les a fournis.
- `contextSnapshot` - le contexte du commentaire / de la page auquel l'approbation se rapporte.

Utile pour transférer des approbations en attente vers un canal chat ops : un bot Slack abonné à `approval.requested` peut publier l'action et le raisonnement dans un canal de modération pour une revue rapide.

### `approval.decided`

Se déclenche lorsqu'une approbation sort de l'état `PENDING`. Le payload inclut :

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED`, ou `EXECUTION_FAILED`.
- `decidedBy` - l'ID utilisateur du modérateur qui a pris la décision.
- `decidedAt` - moment de la décision.
- `executedAt` - si APPROVED, moment où la plateforme a exécuté l'action approuvée.
- `executionResult` - si APPROVED, une chaîne décrivant le résultat de l'exécuteur.
- `contextSnapshot` - le contexte du commentaire / de la page.

Cet événement couvre tous les résultats de décision :

- **Approuvé + exécuté correctement** -> `status: APPROVED`, `executedAt` défini, `executionResult` est le message de succès.
- **Approuvé + échec de l'exécuteur** -> `status: EXECUTION_FAILED`, `executedAt` défini, `executionResult` décrit l'échec.
- **Rejeté** -> `status: REJECTED`, `executedAt` est null, `executionResult` est null.

### Header

Chaque livraison inclut un en-tête HTTP `X-FastComments-Agent-Event` contenant le nom canonique de l'événement (`trigger.succeeded`, etc.). Utile si votre point de terminaison est une URL unique gérant plusieurs types d'événements.

### Voir aussi

- [Webhook Payloads](#webhook-payloads) pour les schémas complets de payloads par événement.
- [Webhook Signing](#webhook-signing) pour le schéma HMAC.
- [Webhook Retries](#webhook-retries) pour la sémantique de livraison.
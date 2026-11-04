Il existe quatre types d'événements de webhook d'agent. Chaque événement possède une valeur d'énumération numérique (utilisée dans les payloads) et un nom de chaîne canonique (utilisé dans le champ d'enveloppe `event` et dans l'en-tête HTTP `X-FastComments-Agent-Event`).

| Nom de l'événement | Enum | Se déclenche quand |
|---|---|---|
| `trigger.succeeded` | 0 | Une exécution d'agent se termine avec le statut `SUCCESS`. |
| `trigger.failed` | 1 | Une exécution d'agent se termine avec le statut `ERROR`. |
| `approval.requested` | 2 | Une approbation est mise en file d'attente à l'état `PENDING`. |
| `approval.decided` | 3 | Une approbation passe à `APPROVED`, `REJECTED` ou `EXECUTION_FAILED`. |

### `trigger.succeeded`

Se déclenche après que l'exécution de l'agent se termine sans erreur. Le champ `data` du payload inclut :

- `triggerId` - l'identifiant unique de l'exécution.
- `triggerType` - l'[énumération des raisons de déclenchement](#triggers-overview) qui a lancé l'exécution.
- `status` - `SUCCESS` (chaîne de caractères).
- `tokensUsed` - jetons consommés lors de cette exécution.
- `wasDryRun` - vrai si l'agent était en [mode dry-run](#dry-run-mode).
- `actions` - tableau d'enregistrements `TenantAgentAction` (voir [Payloads de webhook](#webhook-payloads)).
- `commentId`, `url`, `urlId` - si le déclencheur en avait.

Si l'exécution n'a effectué aucune action, le tableau `actions` est vide - il s'agit d'une exécution réussie de type « l'agent a décidé de ne rien faire », ce qui est utile à savoir.

### `trigger.failed`

Se déclenche lorsqu'une exécution échoue. Même forme de payload que `trigger.succeeded`, avec `status: 'ERROR'` et un champ supplémentaire `errorMessage` décrivant ce qui s'est mal passé. Les erreurs possibles incluent les échecs d'appels LLM, les échecs de dispatch d'outils et l'épuisement du budget en cours d'exécution.

`actions` peut encore contenir des entrées pour les appels d'outils qui se sont terminés avant l'erreur.

### `approval.requested`

Se déclenche dès qu'une approbation est mise en file d'attente à l'état `PENDING`. Le payload inclut :

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status: 'PENDING'`.
- `args` - les arguments de l'outil **transmis tels quels** depuis l'appel LLM. La forme est propre à chaque outil et ne constitue pas un contrat public stable - le schéma peut évoluer à mesure que de nouveaux outils sont ajoutés.
- `createdAt`.
- `justification`, `confidence` - si l'agent les a fournis.
- `contextSnapshot` - le contexte du commentaire / de la page auquel l'approbation se rapporte.

Utile pour transférer les approbations en attente vers un canal de chat ops : un bot Slack abonné à `approval.requested` peut publier l'action et le raisonnement dans un canal de modération pour un examen en un coup d'œil.

### `approval.decided`

Se déclenche lorsqu'une approbation quitte l'état `PENDING`. Le payload inclut :

- `approvalId`, `triggerId`.
- `toolName`, `actionType`.
- `status` - `APPROVED`, `REJECTED` ou `EXECUTION_FAILED`.
- `decidedBy` - l'identifiant utilisateur du modérateur qui a pris la décision.
- `decidedAt` - le moment de la décision.
- `executedAt` - si APPROVED, le moment où la plateforme a exécuté l'action approuvée.
- `executionResult` - si APPROVED, une chaîne décrivant le résultat de l'exécuteur.
- `contextSnapshot` - le contexte du commentaire / de la page.

Cet événement couvre tous les résultats de décision :

- **Approuvé + exécuté proprement** -> `status: APPROVED`, `executedAt` renseigné, `executionResult` est le message de succès.
- **Approuvé + échec de l'exécuteur** -> `status: EXECUTION_FAILED`, `executedAt` renseigné, `executionResult` décrit l'échec.
- **Rejeté** -> `status: REJECTED`, `executedAt` est null, `executionResult` est null.

### En-tête

Chaque livraison inclut un en-tête HTTP `X-FastComments-Agent-Event` avec le nom de chaîne canonique de l'événement (`trigger.succeeded`, etc.). Utile si votre point de terminaison est une URL unique gérant plusieurs types d'événements.

### Voir aussi

- [Payloads de webhook](#webhook-payloads) pour les schémas de payload complets de chaque événement.
- [Signature des webhooks](#webhook-signing) pour le schéma HMAC.
- [Nouvelles tentatives de webhook](#webhook-retries) pour la sémantique de livraison.
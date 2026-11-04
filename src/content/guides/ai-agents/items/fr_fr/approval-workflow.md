Une **approbation** est un appel d'outil mis en file d'attente qui nécessite qu'un humain l'approuve ou le rejette avant que la plateforme ne l'exécute.

### Configuration des approbations

Sur le formulaire de modification de l'agent, la section **Approvals** liste tous les outils que l'agent est autorisé à invoquer (la liste autorisée (allowlist)) et vous permet de cocher ceux qui doivent être examinés par un humain. Les outils non cochés s'exécutent immédiatement. Les outils cochés sont mis en file d'attente.

Les outils non autorisés sont *refusés immédiatement*, pas mis en file d'attente - les approbations ne s'appliquent qu'aux outils qui sont par ailleurs autorisés.

### Que se passe-t-il lorsqu'une action soumise à approbation est déclenchée

1. L'agent choisit un appel d'outil (par ex. `ban_user`) avec des arguments, une justification et un niveau de confiance.
2. Au lieu d'exécuter, la plateforme met en file d'attente une approbation en état `PENDING` avec le nom de l'outil, les arguments, la justification, la confiance, et un instantané du contexte décrivant le déclencheur qui l'a produit.
3. Des notifications sont envoyées aux réviseurs (voir [Notifications d'approbation](#approval-notifications)).
4. L'exécution de l'agent se termine et est enregistrée - l'action apparaît avec **Pending approval** dans la [Vue détaillée de l'exécution](#run-detail-view).

### Examen des approbations

La boîte de réception des approbations sur [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) liste les approbations en attente, approuvées, rejetées et celles dont l'exécution a échoué. Pour chacune :

- **Nom de l'outil et arguments** - exactement ce que l'agent veut faire.
- **Raisonnement de l'agent** - la justification fournie par l'agent.
- **Confiance** - la confiance auto-évaluée de l'agent, de 0.0 à 1.0.
- **Instantané du contexte** - le commentaire, la page et l'utilisateur ciblés par l'action.

Deux boutons : **Approuver** et **Rejeter**. Approuver exécute réellement l'outil ; Rejeter supprime la demande.

### États du statut d'approbation

Une approbation évolue à travers ces états :

| State | Meaning |
|---|---|
| `PENDING` | En attente d'une décision humaine. |
| `APPROVED` | Un humain a approuvé et l'action a été exécutée. |
| `REJECTED` | Un humain a rejeté. L'action n'a pas été exécutée. |
| `EXECUTION_FAILED` | Un humain a approuvé mais l'action a échoué (par ex., le commentaire ciblé avait déjà été supprimé). |
| `EXECUTING` | Transitoire : un humain a cliqué sur Approuver et l'action est en cours d'exécution. Utilisé pour sérialiser les clics d'approbation concurrents afin qu'un outil ayant de véritables effets secondaires ne s'exécute jamais deux fois. |

L'état `EXECUTING` est important lorsque deux réviseurs cliquent sur Approuver simultanément - l'un l'emporte, l'autre voit que l'approbation a déjà évolué.

### Ce qui est nettoyé

Les approbations en attente restent en attente jusqu'à décision. Elles expirent automatiquement **90 jours** après leur création. Les approbations dans tout autre état sont également supprimées après 90 jours pour des raisons d'hygiène de stockage.

Les champs de l'approbation `"decided by" / "decided at" / "executed at" / "execution result"` sont remplis au fur et à mesure que l'approbation évolue - tous visibles dans la vue détaillée de la boîte de réception.

### Intégration des webhooks

Deux événements webhook sont déclenchés au fur et à mesure que les approbations évoluent :

- **`approval.requested`** - lors de l'insertion en `PENDING`.
- **`approval.decided`** - lors de la transition vers `APPROVED`, `REJECTED` ou `EXECUTION_FAILED`.

Les deux sont signés comme tous les autres webhooks. Voir [Événements webhook](#webhook-events) et [Payloads webhook](#webhook-payloads).

### Renforcement : contrôle des outils connus

Les approbations refusent de mettre en file d'attente tout nom d'outil qui n'est pas un outil d'agent reconnu. Il s'agit d'une vérification en profondeur : même si un chemin de code futur transmet un nom d'outil dérivé d'un LLM dans le flux d'approbation, cette chaîne ne pourra jamais atterrir comme approbation mise en file. L'ensemble des noms d'outils connus est le même que celui listé dans la [Référence des outils](#tools-overview).

### Modèles courants de filtrage

- **Brand-new moderation agent** - protéger `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Surveillez la boîte de réception pendant quelques semaines, puis supprimez le filtrage pour les outils à faible taux d'erreur.
- **Long-term moderation agent** - conservez `ban_user` et toute action irréversible (`deleteAllUsersComments`, `banIP`) sous approbation indéfiniment.
- **EU region** - `ban_user` est activé par défaut par l'article 17 indépendamment de ce que vous cochez. Voir [Conformité à l'article 17 du DSA de l'UE](#eu-dsa-compliance).

### Ce que les approbations ne font **pas**

- Elles ne retardent pas les autres appels d'outil de l'agent. L'exécution de l'agent peut appeler plusieurs outils, et seuls ceux soumis à approbation sont mis en file d'attente - les autres s'exécutent normalement.
- Elles ne font pas revenir en arrière l'exécution de l'agent si l'humain rejette. La portion non soumise à approbation de l'exécution est déjà terminée.
Une **approbation** est un appel d'outil mis en file d'attente qui nécessite l'approbation ou le rejet d'un humain avant que la plateforme ne l'exécute.

### Configurer les approbations

Sur le formulaire d'édition de l'agent, la section **Approvals** liste chaque outil que l'agent est autorisé à invoquer (la liste d'autorisation (allowlist)) et vous permet de cocher ceux qui doivent être examinés par un humain. Les outils non cochés s'exécutent immédiatement. Les outils cochés sont mis en file d'attente.

Les outils non autorisés sont *refusés catégoriquement*, ils ne sont pas mis en file d'attente — les approbations ne s'appliquent qu'aux outils par ailleurs autorisés.

### Que se passe-t-il lorsqu'une action soumise à approbation est déclenchée

1. L'agent sélectionne un appel d'outil (par ex. `ban_user`) avec des arguments, une justification et un niveau de confiance.
2. Au lieu d'exécuter, la plateforme place en file d'attente une approbation en état `PENDING` avec le nom de l'outil, les arguments, la justification, la confiance et un instantané du contexte décrivant le déclencheur qui l'a produit.
3. Des notifications sont envoyées aux réviseurs (voir [Notifications d'approbation](#approval-notifications)).
4. L'exécution de l'agent se termine et est enregistrée — l'action est affichée avec **Approbation en attente** dans la [Vue détaillée de l'exécution](#run-detail-view).

### Examiner les approbations

La boîte de réception des approbations sur [my-account/ai-agent-approvals](https://fastcomments.com/auth/my-account/ai-agent-approvals) liste les approbations en attente, approuvées, rejetées et celles dont l'exécution a échoué. Pour chacune :

- **Nom de l'outil et arguments** - exactement ce que l'agent souhaite faire.
- **Raisonnement de l'agent** - la justification fournie par l'agent.
- **Confiance** - l'auto-évaluation de la confiance de l'agent, de 0.0 à 1.0.
- **Instantané du contexte** - le commentaire, la page et l'utilisateur ciblés par l'action.

Deux boutons : **Approve** et **Reject**. Approve exécute réellement l'outil ; Reject supprime la demande.

### États d'une approbation

Une approbation traverse ces états :

| State | Meaning |
|---|---|
| `PENDING` | En attente d'une décision humaine. |
| `APPROVED` | Un humain a approuvé et l'action a été exécutée. |
| `REJECTED` | Un humain a rejeté. L'action n'a pas été exécutée. |
| `EXECUTION_FAILED` | Un humain a approuvé mais l'exécution a échoué (par ex., le commentaire ciblé était déjà supprimé). |
| `EXECUTING` | Transitoire : un humain a cliqué sur Approve et l'action est en cours d'exécution. Utilisé pour sérialiser les clics d'approbation concurrents afin qu'un outil ayant des effets réels ne s'exécute jamais deux fois. |

L'état `EXECUTING` est important lorsque deux réviseurs cliquent sur Approve simultanément — l'un l'emporte, l'autre voit que l'approbation a déjà évolué.

### Ce qui est nettoyé

Les approbations en attente restent en attente jusqu'à décision. Elles expirent automatiquement **90 jours** après leur création. Les approbations dans tout autre état sont également supprimées après 90 jours pour des raisons d'hygiène de stockage.

Les champs de l'approbation « decided by » / « decided at » / « executed at » / « execution result » sont remplis au fur et à mesure que l'approbation évolue — tous visibles dans la vue détaillée de la boîte de réception.

### Intégration des webhooks

Deux événements webhook sont déclenchés lors des transitions d'approbation :

- **`approval.requested`** - lors de l'insertion en PENDING.
- **`approval.decided`** - lors de la transition vers APPROVED, REJECTED ou EXECUTION_FAILED.

Les deux sont signés comme tous les autres webhooks. Voir [Événements Webhook](#webhook-events) et [Charges utiles Webhook](#webhook-payloads).

### Renforcement : contrôle des outils reconnus

Les approbations refusent de mettre en file d'attente tout nom d'outil qui n'est pas un outil d'agent reconnu. Il s'agit d'une vérification en profondeur : même si un futur chemin de code transmet un nom d'outil dérivé d'un LLM dans le flux d'approbation, cette chaîne ne pourra jamais aboutir en tant qu'approbation mise en file d'attente. L'ensemble des noms d'outils connus est le même que celui répertorié dans [Référence des outils](#tools-overview).

### Modèles courants de filtrage

- **Agent de modération tout neuf** - soumettre `ban_user`, `mark_comment_spam`, `mark_comment_approved`, `send_email`. Surveillez la boîte de réception pendant quelques semaines, puis supprimez le filtrage pour les outils à faible taux d'erreur.
- **Agent de modération à long terme** - gardez `ban_user` et toutes les actions irréversibles (`deleteAllUsersComments`, `banIP`) soumises à approbation indéfiniment.
- **Région UE** - `ban_user` est verrouillé par l'Article 17 indépendamment de vos choix. Voir [Conformité de l'Article 17 du DSA de l'UE](#eu-dsa-compliance).

### Ce que les approbations ne font pas

- Elles ne retardent pas les autres appels d'outils de l'agent. L'exécution de l'agent peut appeler plusieurs outils, et seuls ceux soumis à approbation sont mis en file d'attente — les autres s'exécutent normalement.
- Elles ne rétablissent pas l'exécution de l'agent si l'humain rejette. La partie de l'exécution non soumise à approbation est déjà terminée.
Run History est le journal, par agent, de chaque déclenchement exécuté. Accessible depuis la page de liste des agents via le bouton **Runs**, ou directement à l'adresse `/auth/my-account/ai-agents/{agentId}/runs`.

### Ce que contient la page

Un tableau paginé avec une ligne par exécution :

| Colonne | Signification |
|---|---|
| Date | Quand le déclencheur s'est déclenché (ou quand le déclencheur différé s'est exécuté). |
| Status | **Started**, **Success** ou **Error**. Un badge **Dry Run** est affiché à côté si l'exécution était en mode dry-run. |
| Cost | Coût par exécution dans la devise de votre tenant. Vide pour les exécutions en cours (Started). |
| Actions | Le nombre d'appels d'outils dans l'exécution. |
| Details | Un bouton **View** qui ouvre la [vue détaillée de l'exécution](#run-detail-view). |

### Signification des statuts

- **Started** - l'exécution est en cours, ou elle s'est arrêtée avant de se terminer. Une exécution bloquée en « Started » pendant une durée anormalement longue correspond généralement à un timeout d'appel LLM.
- **Error** - l'exécution s'est terminée mais a échoué quelque part - un appel LLM a renvoyé une erreur, l'invocation d'un outil a échoué, etc. La vue détaillée contient l'erreur spécifique.
- **Success** - l'exécution s'est terminée sans erreur. L'agent peut avoir effectué zéro, une ou plusieurs actions.

### État vide

Lorsqu'un agent n'a aucune exécution, la page affiche : « No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments. »

Cette dernière précision est volontaire - le [flux de test run](#test-runs-replays) est la méthode recommandée pour remplir le Run History d'un agent tout neuf.

### Ce qui ne figure pas sur la page d'historique des exécutions

- **Les déclencheurs en direct jamais dispatchés** - un déclencheur abandonné en raison du budget, du périmètre ou de la limitation de débit n'apparaît pas sur cette page. Ceux-ci figurent sur la [page Analytics](#analytics-page) sous « Triggers skipped ».
- **Les approbations** - les approbations en attente pour les actions effectuées dans cette exécution se trouvent dans la [boîte de réception des approbations](#approval-workflow). L'action apparaît dans la vue détaillée de l'exécution comme **Pending approval**.

### Rétention

Les enregistrements d'exécution individuels sont conservés pendant 90 jours, après quoi l'exécution disparaît de l'historique. Les coûts et les nombres de déclenchements continuent d'être agrégés dans les résumés analytiques à long terme, de sorte que la [page Analytics](#analytics-page) affiche toujours les totaux historiques au-delà de cette fenêtre.

### Replays

Les exécutions produites par les replays sont exclues de la vue des exécutions en direct par défaut. La page [Test Runs (Replays)](#test-runs-replays) est l'endroit où vous les consultez.

### Filtrage entre agents

Le tableau des exécutions est propre à chaque agent. Il n'existe pas de vue multi-agents des exécutions - la [page Analytics](#analytics-page) est le résumé multi-agents. Si vous devez inspecter les exécutions de plusieurs agents, les événements `trigger.succeeded` et `trigger.failed` des [Webhooks](#webhooks-overview) sont ce que vous devriez transmettre à votre propre système.
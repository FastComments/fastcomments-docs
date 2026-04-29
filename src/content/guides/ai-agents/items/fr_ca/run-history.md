Historique des exécutions est le journal par agent de chaque déclencheur qui s'est exécuté. Accessible depuis la page de liste des agents via le bouton **Exécutions**, ou directement à `/auth/my-account/ai-agents/{agentId}/runs`.

### Ce qui se trouve sur la page

Un tableau paginé avec une ligne par exécution :

| Column | Meaning |
|---|---|
| Date | Quand le déclencheur s'est déclenché (ou quand le déclencheur différé s'est exécuté). |
| Status | **Started**, **Success**, ou **Error**. Un badge **Exécution à blanc** est affiché à côté si l'exécution était en mode exécution à blanc. |
| Cost | Coût par exécution dans la devise de votre locataire. Vide pour les exécutions en cours (Started). |
| Actions | Le nombre d'appels d'outil dans l'exécution. |
| Details | Un bouton **Afficher** qui ouvre la [Vue détaillée de l'exécution](#run-detail-view). |

### Signification des statuts

- **Started** - l'exécution est en cours, ou elle s'est arrêtée avant de se terminer. Une exécution qui reste en « Started » pendant une durée inhabituellement longue représente généralement un timeout d'appel LLM-call.
- **Error** - l'exécution est terminée mais a échoué quelque part - un appel LLM a retourné une erreur, un dispatch d'outil a échoué, etc. La vue détaillée contient l'erreur spécifique.
- **Success** - l'exécution s'est terminée sans erreur. L'agent a pu effectuer zéro, une ou plusieurs actions.

### État vide

Quand un agent n'a aucune exécution, la page affiche : "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

Ce dernier point est intentionnel - le [flux d'exécution de test](#test-runs-replays) est la manière recommandée pour remplir l'Historique des exécutions sur un agent neuf.

### Ce qui n'apparaît pas sur la page d'historique des exécutions

- **Déclencheurs en direct qui n'ont jamais été dispatchés** - un déclencheur abandonné à cause du budget, de la portée ou du rate limiting n'apparaît pas sur cette page. Ceux-ci apparaissent sur la [Page d'analyse](#analytics-page) sous « Triggers skipped ».
- **Approbations** - les approbations en attente pour des actions effectuées dans cette exécution se trouvent dans la [boîte de réception des approbations](#approval-workflow). L'action apparaît dans la vue détaillée de l'exécution comme **En attente d'approbation**.

### Rétention

Les enregistrements de chaque exécution sont conservés pendant 90 jours, après quoi l'exécution disparaît de l'historique. Les coûts et les comptes de déclencheurs continuent de s'agréger dans les synthèses analytiques à long terme, donc la [Page d'analyse](#analytics-page) affiche toujours les totaux historiques au-delà de cette période.

### Relectures

Les exécutions produites par des relectures sont exclues de la vue des exécutions en direct par défaut. La page [Exécutions de test (replays)](#test-runs-replays) est l'endroit où vous pouvez les consulter.

### Filtrage entre agents

Le tableau des exécutions est spécifique à chaque agent. Il n'existe pas de vue des exécutions inter-agent - la [Page d'analyse](#analytics-page) est le résumé inter-agent. Si vous devez inspecter des exécutions à travers plusieurs agents, les événements `trigger.succeeded` et `trigger.failed` des [Webhooks](#webhooks-overview) sont ceux que vous devriez transférer vers votre propre système.
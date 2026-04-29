L'historique des exécutions est le journal par agent de chaque déclencheur qui s'est exécuté. Accessible depuis la page de liste des agents via le bouton **Runs**, ou directement à `/auth/my-account/ai-agents/{agentId}/runs`.

### What's on the page

Un tableau paginé avec une ligne par exécution :

| Column | Meaning |
|---|---|
| Date | Quand le déclencheur s'est déclenché (ou quand le déclencheur différé s'est exécuté). |
| Status | **Démarré**, **Succès**, ou **Erreur**. Un badge **Exécution à blanc** est affiché à côté si l'exécution était en mode exécution à blanc. |
| Cost | Coût par exécution dans la devise de votre locataire. Vide pour les exécutions en cours (Démarré). |
| Actions | Le nombre d'appels d'outils dans l'exécution. |
| Details | Un bouton **Voir** qui ouvre [Vue détaillée de l'exécution](#run-detail-view). |

### Status meanings

- **Démarré** - l'exécution est en cours, ou elle est morte avant d'être terminée. Une exécution bloquée sur « Démarré » pendant une durée anormalement longue représente généralement un délai d'attente d'appel LLM.
- **Erreur** - l'exécution s'est terminée mais a échoué quelque part - l'appel LLM a renvoyé une erreur, une distribution d'outil a échoué, etc. La vue détaillée contient l'erreur spécifique.
- **Succès** - l'exécution s'est terminée sans erreur. L'agent peut avoir effectué zéro, une ou plusieurs actions.

### Empty state

When an agent has no runs, the page shows: "No runs yet for this agent. Enabled runs appear here once a trigger fires; use Test run to preview what this agent would do against past comments."

Cette dernière précision est intentionnelle - le [flux d'exécution de test](#test-runs-replays) est la méthode recommandée pour peupler l'historique des exécutions sur un agent neuf.

### What's not on the run history page

- **Live triggers that never dispatched** - un déclencheur supprimé à cause du budget, de la portée ou du rate limiting n'apparaît pas sur cette page. Ceux-ci apparaissent dans la [page Analytics](#analytics-page) sous « Triggers skipped ».
- **Approvals** - les approbations en attente pour les actions effectuées lors de cette exécution se trouvent dans la [boîte de réception des approbations](#approval-workflow). L'action apparaît dans la vue détaillée de l'exécution comme **En attente d'approbation**.

### Retention

Les enregistrements individuels d'exécution sont conservés pendant 90 jours, après quoi l'exécution disparaît de l'historique. Les coûts et les comptes de déclencheurs continuent d'être consolidés dans des résumés analytiques à long terme, donc la [page Analytics](#analytics-page) affiche toujours des totaux historiques au-delà de cette fenêtre.

### Replays

Les exécutions produites par les replays sont exclues de la vue des exécutions en direct par défaut. La page [Exécutions de test (Replays)](#test-runs-replays) est l'endroit où vous pouvez les voir.

### Filtering across agents

Le tableau des exécutions est par agent. Il n'existe pas de vue des exécutions inter-agents - la [page Analytics](#analytics-page) est le résumé inter-agents. Si vous devez inspecter des exécutions sur plusieurs agents, les événements `trigger.succeeded` et `trigger.failed` des [Webhooks](#webhooks-overview) sont ceux que vous devrez transférer vers votre propre système.
**Dry Run** est le mode de sécurité activé par défaut pour tout nouvel agent. L'agent s'exécute de bout en bout sauf pour la partie où il intervient dans votre communauté.

### Ce qui s'exécute en Dry Run

- Les déclencheurs s'activent normalement.
- L'invite de l'agent, les [directives de la communauté](#community-guidelines) et le [contexte](#context-options) sont assemblés.
- Le LLM est appelé.
- Le modèle choisit les appels d'outils et fournit des justifications + des scores de confiance.
- L'exécution est enregistrée avec un badge **Dry Run** afin qu'elle soit clairement distinguée des exécutions en direct.

### Ce qui ne s'exécute pas en Dry Run

- Aucun commentaire n'est publié, aucun vote n'est enregistré, aucun commentaire n'est épinglé/désépinglé/verrouillé/déverrouillé.
- Aucun commentaire n'est marqué comme spam, approuvé ou revu.
- Aucun utilisateur n'est banni, averti ou récompensé par un badge.
- Aucun e-mail n'est envoyé.
- Aucune mémoire n'est écrite. (Oui — y compris la mémoire. Les agents en dry-run ne peuvent pas empoisonner le pool de mémoire partagé avec des décisions hypothétiques.)
- Aucun webhook ne se déclenche pour les actions d'outil. (Les webhooks au niveau du déclencheur `trigger.succeeded` / `trigger.failed` se déclenchent toutefois toujours et la charge utile inclut `wasDryRun: true`. Voir [Webhook Payloads](#webhook-payloads).)

### Ce que cela coûte

Les exécutions en Dry Run effectuent **le même appel LLM** qu'une exécution Activée. Les jetons sont facturés, les [plafonds de budget](#budgets-overview) s'appliquent, et les exécutions comptent dans les limites journalières/mensuelles par agent et par locataire.

Ce coût est le prix à payer pour obtenir un aperçu fidèle. Un mode « sauter l'appel LLM » ne vous donnerait aucun indice sur le comportement réel de l'agent.

### Lecture des résultats du dry-run

Dans l'[Historique des exécutions](#run-history), les exécutions en dry-run sont marquées du badge **Dry Run** dans la colonne d'état. Les actions à l'intérieur de chaque exécution ressemblent visuellement aux actions en direct — même nom d'outil, mêmes arguments, mêmes justifications et scores de confiance — sauf qu'aucune d'entre elles n'a eu lieu.

La [page Analytics](#analytics-page) ventile les exécutions « dry-run vs live » par mois afin que vous puissiez voir quelle part de votre dépense en jetons a été consacrée à l'observation.

### Sortir du Dry Run

Éditez l'agent et changez **Status** en **Enabled**. Le prochain déclencheur s'exécutera en direct.

Vous pouvez aussi procéder dans l'autre sens — passer d'Enabled à Dry Run — si l'agent commence à faire des choses que vous n'aimez pas. Il n'y a aucune pénalité.

### Les relectures forcent le Dry Run

La fonctionnalité [Exécutions de test (Replays)](#test-runs-replays) exécute l'agent contre des commentaires historiques **toujours en dry-run**, quel que soit le statut enregistré de l'agent. Les relectures ne peuvent pas effectuer d'actions réelles sur des commentaires passés. C'est fait exprès — la relecture est un outil d'aperçu, pas un outil de modération.
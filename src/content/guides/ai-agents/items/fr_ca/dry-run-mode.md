**Dry Run** est le mode de sécurité dans lequel tout nouvel agent commence.

### Ce qui s'exécute en Dry Run

- Les déclencheurs s'exécutent normalement.
- L'invite de l'agent, les [directives communautaires](#community-guidelines) et le [contexte](#context-options) sont assemblés.
- Le LLM est appelé.
- Le modèle choisit les appels d'outil et fournit des justifications + des scores de confiance.
- L'exécution est enregistrée avec un badge **Dry Run** afin d'être clairement distinguée des exécutions en direct.

### Ce qui ne s'exécute pas en Dry Run

- Aucun commentaire n'est publié, aucun vote n'est émis, aucun commentaire n'est épinglé/désépinglé/verrouillé/déverrouillé.
- Aucun commentaire n'est marqué comme spam, approuvé ou examiné.
- Aucun utilisateur n'est banni, averti ou récompensé d'un badge.
- Aucun courriel n'est envoyé.
- Aucune mémoire n'est écrite. (Oui - y compris la mémoire. Les agents en dry-run ne peuvent pas empoisonner le pool de mémoire partagé avec des décisions hypothétiques.)
- Aucun webhook ne se déclenche pour les actions d'outil. (Les webhooks au niveau du déclencheur `trigger.succeeded` / `trigger.failed` se déclenchent toutefois et la charge utile inclut `wasDryRun: true`. Voir [Charges utiles des webhooks](#webhook-payloads).)

### Ce que ça coûte

Les exécutions en Dry Run effectuent **le même appel LLM** qu'une exécution Activée. Les jetons sont facturés, les [plafonds de budget](#budgets-overview) s'appliquent, et les exécutions sont comptées dans les limites quotidiennes/mensuelles par agent et par locataire.

Ce coût est le prix à payer pour obtenir un aperçu fidèle. Un mode « ignorer l'appel LLM » ne vous donnerait aucun indice sur le comportement réel de l'agent.

### Lecture des résultats en dry-run

Dans [Historique des exécutions](#run-history), les exécutions en dry-run sont marquées par le badge **Dry Run** dans la colonne d'état. Les actions à l'intérieur de chaque exécution semblent identiques aux actions en direct - même nom d'outil, mêmes arguments, mêmes justification et score de confiance - sauf qu'aucune d'entre elles n'a eu lieu.

La [page d'analytique](#analytics-page) décompose les exécutions « dry-run vs live » par mois afin que vous puissiez voir quelle part de vos dépenses en jetons a servi à l'observation.

### Passer du Dry Run

Modifiez l'agent et changez **Statut** en **Activé**. Le prochain déclenchement s'exécutera en direct.

Vous pouvez aussi faire l'inverse - passer d'Activé à Dry Run - si l'agent commence à faire des choses que vous n'aimez pas. Il n'y a aucune pénalité.

### Les relectures forcent le Dry Run

La fonctionnalité [Exécutions de test (relectures)](#test-runs-replays) exécute l'agent contre des commentaires historiques **toujours en dry-run**, indépendamment du statut enregistré de l'agent. Les relectures ne peuvent pas effectuer d'actions réelles sur des commentaires passés. C'est intentionnel - la relecture est un outil d'aperçu, pas un outil de modération.
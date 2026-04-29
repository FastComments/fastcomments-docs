Chaque agent a des plafonds de dépenses. La plateforme arrête d'envoyer des tâches à l'agent lorsqu'un plafond est atteint et reprend une fois la période renouvelée.

### Deux portées, deux périodes

Il y a quatre plafonds au total — deux portées (par agent, par locataire) combinées avec deux périodes (quotidienne, mensuelle).

| Scope | Period | Where you set it |
|---|---|---|
| Per-agent daily | UTC day | Agent edit form -> **Budget** -> **Daily budget** |
| Per-agent monthly | calendar month | Agent edit form -> **Budget** -> **Monthly budget** |
| Per-tenant daily | UTC day | Plan-derived (no separate user-facing input) |
| Per-tenant monthly | calendar month | Plan-derived (no separate user-facing input) |

Un déclencheur ne se lance que si **les quatre plafonds** le permettent. Le premier plafond épuisé est celui qui bloque le déclenchement.

### Devise

Les budgets par agent sont saisis dans la devise de votre compte.

### Que se passe-t-il lorsqu'un plafond est atteint

- Le déclencheur est enregistré comme **abandonné** avec une [drop reason](#drop-reasons) comme `agentDaily` ou `tenantMonthly`.
- Le nombre d'abandons apparaît sur la [Analytics page](#analytics-page) sous « Déclencheurs ignorés (ce mois-ci) ».
- Aucun appel LLM n'est effectué ; aucun jeton n'est dépensé pour le déclencheur abandonné lui-même.
- Le statut de l'agent reste inchangé — il ne peut simplement pas se déclencher tant que la période n'est pas renouvelée.

### Renouvellement de période

- Les plafonds **quotidiens** sont réinitialisés à minuit UTC.
- Les plafonds **mensuels** sont réinitialisés au début de chaque mois calendaire, UTC.

Il n'y a pas de report du budget inutilisé vers la période suivante.

### Plafond strict vs avertissements souples

Les plafonds sont **stricts**. Il n'existe pas de mode « dépasser de 10 % avec avertissement ». Lorsque le plafond est atteint, l'envoi s'arrête.

La partie « souple » correspond aux courriels d'[Budget Alerts](#budget-alerts) — vous recevez un courriel aux seuils configurables (par défaut 80 % et 100 %) afin que vous puissiez augmenter le plafond avant que le trafic ne commence à chuter.

### Où consulter l'utilisation actuelle

- [Analytics page](#analytics-page) — utilisation du budget par agent et pour le locataire avec repères de plafond.
- La section **Stats** du formulaire de modification de l'agent.
- La vue en liste (nombre d'approbations en attente et exécutions récentes affichés sur la carte de l'agent).

### Choisir un budget

Quelques règles empiriques :

- **Un nouvel agent** — déterminez le budget. Surveillez l'[Run History](#run-history) pendant une semaine. Ajustez en fonction du coût observé par exécution × volume de déclenchements prévu.
- **Un agent à fort volume** (par ex., déclencheur de nouveau commentaire sur un site très fréquenté) — le plafond quotidien est ce qui attrape une boucle incontrôlée. Choisissez un plafond quotidien équivalant à 2–3× votre dépense quotidienne prévue afin qu'une journée normale et chargée reste confortablement en dessous.
- **Un agent de synthèse ou lourd en contexte** — le coût par exécution est élevé. Définissez un plafond quotidien plus strict pour empêcher qu'une mauvaise journée n'explose le mensuel.

### Contournement du budget pour les relectures

[Test runs / replays](#test-runs-replays) sont soumis à leur propre plafond strict (défini sur le formulaire de relecture, séparé des plafonds quotidiens/mensuels de l'agent), ET aux plafonds de l'agent et du locataire. Le premier atteint stoppe la relecture.

### Voir aussi

- [Budget Alerts](#budget-alerts) pour les notifications par courriel.
- [Cost Model](#cost-model) pour la façon dont la plateforme convertit les jetons en dollars.
- [Drop Reasons](#drop-reasons) pour la liste complète des raisons pour lesquelles un déclencheur ne s'exécute pas.
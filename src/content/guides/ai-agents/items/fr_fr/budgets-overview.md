---
Chaque agent a des plafonds de dépenses. La plateforme cesse d'exécuter l'agent lorsqu'un plafond est atteint et reprend une fois la période réinitialisée.

### Deux périmètres, deux périodes

Il y a quatre plafonds au total - deux périmètres (par agent, par locataire) croisés avec deux périodes (quotidienne, mensuelle).

| Périmètre | Période | Où le définir |
|---|---|---|
| Par agent (quotidien) | jour UTC | Formulaire d'édition de l'agent -> **Budget** -> **Budget quotidien** |
| Par agent (mensuel) | mois calendaire | Formulaire d'édition de l'agent -> **Budget** -> **Budget mensuel** |
| Par locataire (quotidien) | jour UTC | Dérivé du plan (pas d'entrée utilisateur distincte) |
| Par locataire (mensuel) | mois calendaire | Dérivé du plan (pas d'entrée utilisateur distincte) |

Un déclencheur ne s'exécute que si **les quatre plafonds** le permettent. Le premier plafond à être épuisé est celui qui provoque l'abandon du déclencheur.

### Devise

Les budgets par agent sont saisis dans la devise de votre compte.

### Que se passe-t-il lorsqu'un plafond est atteint

- Le déclencheur est enregistré comme **abandonné** avec une [raison d'abandon](#drop-reasons) comme `agentDaily` ou `tenantMonthly`.
- Le nombre d'abandons apparaît sur la [page d'analyse](#analytics-page) sous « Déclencheurs ignorés (ce mois-ci) ».
- Aucun appel LLM n'est effectué ; aucun token n'est dépensé pour le déclencheur abandonné lui-même.
- Le statut de l'agent reste inchangé - il ne peut cependant pas se déclencher tant que la période n'est pas réinitialisée.

### Réinitialisation des périodes

- Les plafonds **quotidiens** sont réinitialisés à minuit UTC.
- Les plafonds **mensuels** sont réinitialisés au début de chaque mois calendaire, UTC.

Il n'y a pas de report du budget inutilisé sur la période suivante.

### Plafond strict vs avertissements souples

Les plafonds sont **stricts**. Il n'y a pas de mode « dépasser de 10% avec avertissement ». Lorsque le plafond est atteint, l'exécution s'arrête.

La partie « souple » est constituée par les e-mails [Alertes de budget](#budget-alerts) - vous recevez un e-mail à des seuils configurables (par défaut 80 % et 100 %) afin de pouvoir augmenter le plafond avant que le trafic ne commence à diminuer.

### Où consulter l'utilisation actuelle

- [Page d'analyse](#analytics-page) - utilisation du budget par agent et au niveau du locataire avec des marqueurs de plafond.
- La section **Statistiques** du formulaire d'édition de l'agent.
- La vue en liste (le nombre d'approbations en attente et les exécutions récentes figurent sur la carte de l'agent).

### Choisir un budget

Quelques règles empiriques :

- **Un nouvel agent** - déterminer le budget. Surveillez [Historique d'exécution](#run-history) pendant une semaine. Ajustez en fonction du coût observé par exécution × volume de déclenchements attendu.
- **Un agent à fort volume** (par ex., déclencheur de nouveau commentaire sur un site très fréquenté) - le plafond quotidien est ce qui attrape une boucle incontrôlée. Choisissez un plafond quotidien qui est 2-3x votre dépense quotidienne prévue afin qu'une journée normale très chargée passe confortablement en dessous.
- **Un agent de résumé ou utilisant beaucoup de contexte** - le coût par exécution est élevé. Fixez un plafond quotidien plus strict pour éviter qu'une mauvaise journée ne fasse exploser le budget mensuel.

### Contournement du budget pour les relectures

[Exécutions de test / relectures](#test-runs-replays) sont soumises à leur **propre** plafond strict (défini sur le formulaire de relecture, séparé des plafonds quotidien/mensuel de l'agent), ET aux plafonds de l'agent et du locataire. Celui qui est atteint en premier arrête la relecture.

### Voir aussi

- [Alertes de budget](#budget-alerts) pour les notifications par e-mail.
- [Modèle de coûts](#cost-model) pour la manière dont la plateforme convertit les tokens en dollars.
- [Raisons d'abandon](#drop-reasons) pour la liste complète des raisons pour lesquelles un déclencheur ne s'exécute pas.

---
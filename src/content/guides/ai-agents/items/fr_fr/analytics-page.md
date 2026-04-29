Analytics is the cross-agent dashboard. Reachable from the Agents IA page via the **Analytics** tab (tenant-wide) or per-agent via the **Analytics** button on each agent's row.

### Filtre

Un menu déroulant en haut - **Tous les agents** ou un agent spécifique. Le reste de la page est resserré en conséquence.

### Utilisation du budget

Quatre barres de progression affichant les dépenses de la période en cours par rapport au plafond :

- **Agent today** (lorsque filtré sur un agent spécifique) - plafond quotidien par agent.
- **Agent this month** - plafond mensuel par agent.
- **Account today** - plafond quotidien du locataire.
- **Account this month** - plafond mensuel du locataire.

Lorsque aucun plafond n'est défini, la barre affiche "(aucun plafond défini)" et montre les dépenses brutes.

### Coût quotidien (30 derniers jours)

Un tableau du coût par jour dans la devise de votre locataire pour la portée sélectionnée. Utile pour repérer :

- **Pics soudains de coût** - généralement dus à une boucle incontrôlée ou à un commentaire viral qui déclenche une propagation.
- **Dérive des coûts** - augmentation progressive du coût quotidien à mesure que votre communauté grandit.

### Actions effectuées

Une répartition des types d'actions sur le mois en cours - "A écrit un commentaire : 47", "Marqué un commentaire comme spam : 12", etc. Utile pour vérifier que l'agent fait bien ce que vous attendiez.

### Déclencheurs ignorés (ce mois-ci)

Totaux regroupés par [Raisons d'abandon](#drop-reasons) :

- Dépassement des quotas agent quotidien / agent mensuel / compte quotidien / compte mensuel.
- Bloqués par la limitation de débit.
- Saturation de la concurrence.

Si vous voyez des abandons ici, votre agent atteint un budget ou une limite de débit et manque des déclencheurs sur lesquels il serait autrement intervenu. Voir [Raisons d'abandon](#drop-reasons).

### Exécution à blanc vs en direct (ce mois-ci)

- **Enabled runs** - nombre d'exécutions ayant effectué de véritables actions ce mois-ci.
- **Dry runs** - nombre d'exécutions en mode exécution à blanc ce mois-ci.

Un signal utile pour l'ajustement : un agent tout nouveau qui n'a pas encore été promu en mode Activé n'affichera que des exécutions à blanc. Un agent en mode Activé avec tous les compteurs à zéro dans cette section est inactif - soit il n'est pas déclenché, soit il est exclu de la portée, soit ses déclencheurs ne sont pas configurés correctement.

### Principaux agents par coût mensuel

Lorsque le filtre est **Tous les agents**, la page liste les agents classés par coût cumulé depuis le début du mois. Repérer votre agent le plus coûteux est la première étape pour optimiser les coûts - généralement la solution consiste à « resserrer ses [options de contexte](#context-options) » ou à « diminuer son [plafond budgétaire](#budgets-overview) ».

### Agents atteignant ou proches de leur plafond

Répartition par agent des agents dont les dépenses atteignent ou approchent leurs plafonds individuels pendant la période en cours :

- **near cap** - au-dessus d'un pourcentage configurable du plafond.
- **over cap** - réellement plafonné, avec {count} ignorés déclencheurs dans cette période.

Cliquez sur l'agent dans ce tableau pour augmenter le plafond, restreindre la portée ou le mettre en pause.

### Résumé du compte

Lorsque le filtre est **Tous les agents** :

- **Triggers today** - nombre.
- **Triggers this month** - nombre.
- Pour chacun : un suffixe `dropped` indiquant combien ont été ignorés.

### Devise

Toutes les valeurs monétaires sont affichées dans la devise de votre locataire.

### Ce que cette page ne fait pas

- Elle n'affiche pas **la ventilation des coûts par action** - celles-ci se trouvent dans la [vue détaillée d'exécution](#run-detail-view).
- Elle n'affiche pas **les transcriptions** ni **les réponses LLM**.
- Elle ne permet pas d'agir sur les agents - la modification, la mise en pause et la suppression se font depuis la liste des agents / la page d'édition.
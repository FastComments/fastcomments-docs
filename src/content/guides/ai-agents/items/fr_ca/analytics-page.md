Analytique est le tableau de bord inter-agents. Accessible depuis la page Agents d'IA via l'onglet **Analytique** (au niveau du locataire) ou par agent via le bouton **Analytique** sur la ligne de chaque agent.

### Filtre

Un menu déroulant en haut - **Tous les agents** ou un agent spécifique. Le reste de la page se recadre en conséquence.

### Utilisation du budget

Quatre barres de progression montrant les dépenses de la période en cours par rapport au plafond :

- **Agent aujourd'hui** (lorsque filtré sur un agent spécifique) - plafond quotidien de l'agent.
- **Agent ce mois-ci** - plafond mensuel de l'agent.
- **Compte aujourd'hui** - plafond quotidien du locataire.
- **Compte ce mois-ci** - plafond mensuel du locataire.

Lorsqu'un plafond n'est pas défini, la barre affiche "(aucun plafond défini)" et montre les dépenses brutes.

### Coût quotidien (30 derniers jours)

Un tableau du coût par jour dans la devise de votre locataire pour la portée sélectionnée. Utile pour repérer :

- **Pics de coûts soudains** - généralement causés par une boucle incontrôlée ou un commentaire viral qui déclenche de nombreux déclencheurs.
- **Dérive des coûts** - augmentation progressive du coût quotidien à mesure que votre communauté grandit.

### Actions effectuées

Une répartition des types d'actions sur le mois en cours - "A écrit un commentaire : 47", "A marqué un commentaire comme spam : 12", et ainsi de suite. Utile pour vérifier que l'agent fait ce que vous attendiez.

### Déclencheurs ignorés (ce mois-ci)

Comptes regroupés par [raison d'abandon](#drop-reasons) :

- Dépassement du plafond quotidien de l'agent / mensuel de l'agent / quotidien du compte / mensuel du compte.
- Limitation de débit.
- Concurrence saturée.

Si vous voyez des abandons ici, votre agent atteint un plafond budgétaire ou une limite de taux et manque des déclencheurs sur lesquels il serait autrement passé. Voir [Raisons d'abandon](#drop-reasons).

### Exécutions en mode simulation vs réelles (ce mois-ci)

- **Exécutions activées** - nombre d'exécutions ayant pris des actions réelles ce mois-ci.
- **Exécutions en simulation** - nombre d'exécutions en mode dry-run ce mois-ci.

Un signal de réglage utile : un agent tout nouveau qui n'a pas encore été promu en mode activé n'affichera que des exécutions en simulation. Un agent en mode activé avec des comptes tous à zéro dans cette section est inactif — soit il n'est pas déclenché, soit il est exclu du périmètre, soit ses déclencheurs ne sont pas configurés correctement.

### Agents les plus coûteux ce mois-ci

Lorsque le filtre est **Tous les agents**, la page liste les agents classés par coût depuis le début du mois. Repérer votre agent le plus coûteux est la première étape de l'optimisation des coûts - la réponse est généralement « resserrer ses [options de contexte](#context-options) » ou « abaisser son [plafond budgétaire](#budgets-overview) ».

### Agents à ou près de leur plafond

Répartition par agent des agents dont les dépenses atteignent ou s'approchent des plafonds par agent pour la période en cours :

- **près du plafond** - au-dessus d'un pourcentage configurable du plafond.
- **au-delà du plafond** - effectivement plafonné, avec `{count} dropped` déclencheurs pendant cette période.

Cliquez sur l'agent dans ce tableau pour augmenter le plafond, réduire le périmètre ou le mettre en pause.

### Résumé du compte

Lorsque le filtre est **Tous les agents** :

- **Déclenchements aujourd'hui** - nombre.
- **Déclenchements ce mois-ci** - nombre.
- Pour chacun : un suffixe `dropped` indiquant combien ont été ignorés.

### Devise

Toutes les valeurs monétaires sont affichées dans la devise de votre locataire.

### Ce que cette page ne fait pas

- Elle n'affiche pas les **ventilations des coûts par action** - celles-ci se trouvent sur [Vue détaillée d'exécution](#run-detail-view).
- Elle n'affiche pas les **transcriptions** ni les **réponses LLM**.
- Elle ne permet pas d'agir sur les agents - la modification, la mise en pause et la suppression se font depuis la liste des agents / la page d'édition.
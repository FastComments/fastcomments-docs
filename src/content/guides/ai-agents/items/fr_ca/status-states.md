Un agent possède l'un des trois statuts :

### Disabled

L'agent est désactivé. Aucun déclencheur n'est traité et l'agent n'apparaît pas dans le chemin de distribution. Son historique d'exécution, ses analyses et sa mémoire restent présents — si vous le réactivez plus tard, les données historiques sont toujours là.

Use `Disabled` when:
- Vous voulez retirer un agent de la rotation sans le perdre.
- Un agent se comporte mal et vous devez l'arrêter immédiatement pendant que vous enquêtez.
- Vous faites une rotation saisonnière des agents (p. ex. un accueil uniquement pendant les vacances).

### Dry Run - valeur par défaut pour les nouveaux agents

L'agent s'exécute de bout en bout - il traite les déclencheurs, appelle le LLM, choisit des appels d'outils, calcule des justifications et la confiance - mais **aucune action réelle n'est effectuée**. Chaque exécution est enregistrée avec le badge **Dry Run** dans [Run History](#run-history).

Use `Dry Run` when:
- Un nouvel agent vient tout juste d'être déployé. Chaque modèle de départ est en dry-run.
- Vous avez modifié le prompt ou changé l'ensemble de déclencheurs et vous voulez voir comment le changement se comporte avant de le valider.
- Vous effectuez un [test run / replay](#test-runs-replays) (les replays forcent le dry-run indépendamment du statut de l'agent).

La plateforme facture des tokens pour les exécutions en dry-run - l'appel LLM a toujours lieu, seuls les effets secondaires sont ignorés. Les plafonds budgétaires s'appliquent aussi au dry-run. Voir [Budgets Overview](#budgets-overview).

### Enabled

L'agent effectue des actions réelles. Les appels d'outils s'exécutent — ou sont mis en file d'attente pour [approval](#approval-workflow) si l'action est soumise à approbation.

Use `Enabled` after dry-run output looks correct.

### Switching status

Vous pouvez basculer entre n'importe quels deux statuts sur le formulaire d'édition. Passer de Dry Run à Enabled ne réexécute pas rétroactivement les actions effectuées en dry-run — celles-ci restent comme historique dry-run. Les nouveaux déclencheurs à partir de ce moment s'exécutent en direct.

Passer d'Enabled à Disabled au milieu d'une exécution **n'interrompt pas** une exécution en cours. Le déclencheur en cours d'exécution se termine (avec ce qu'il a déjà commencé) ; le déclencheur suivant est supprimé parce que l'agent est maintenant Disabled.

### Status during billing problems

Si la facturation de votre locataire devient invalide, tous les agents sont effectivement mis en pause indépendamment du statut enregistré — les déclencheurs sont supprimés avec `BILLING_INVALID` jusqu'à ce que la facturation soit rétablie. Le champ de statut enregistré n'est pas modifié ; le répartiteur refuse simplement d'exécuter. Voir [Plans and Eligibility](#plans-and-eligibility).
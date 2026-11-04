Un agent possède l'un des trois statuts suivants :

### Désactivé (Disabled)

L'agent est désactivé. Aucun déclencheur n'est traité et l'agent n'apparaît pas dans le chemin de dispatch. Son historique d'exécution, ses analyses et sa mémoire sont conservés - si vous le réactivez plus tard, les données historiques sont toujours là.

Utilisez `Disabled` lorsque :
- Vous voulez retirer un agent de la rotation sans le perdre.
- Un agent se comporte mal et vous devez l'arrêter immédiatement pendant votre investigation.
- Vous alternez saisonnièrement les agents (par exemple un greeter réservé aux fêtes).

### Dry Run - valeur par défaut pour les nouveaux agents

L'agent s'exécute de bout en bout - il traite les déclencheurs, appelle le LLM, choisit les appels d'outils, calcule les justifications et la confiance - mais **aucune action réelle n'est entreprise**. Chaque exécution est enregistrée avec le badge **Dry Run** dans [Run History](#run-history).

Utilisez `Dry Run` lorsque :
- Un nouvel agent sort tout juste de la boîte. Chaque template de démarrage arrive en dry-run.
- Vous avez modifié le prompt ou changé l'ensemble des déclencheurs et voulez voir comment le changement se comporte avant de vous engager.
- Vous effectuez une [exécution de test / replay](#test-runs-replays) (les replays forcent le dry-run quel que soit le statut de l'agent).

La plateforme facture des tokens pour les exécutions en dry-run - l'appel au LLM a quand même lieu, seuls les effets de bord sont ignorés. Les plafonds budgétaires s'appliquent aussi au dry-run. Voir [Budgets Overview](#budgets-overview).

### Activé (Enabled)

L'agent entreprend des actions réelles. Les appels d'outils s'exécutent - ou sont mis en file pour [approbation](#approval-workflow) si l'action est contrôlée.

Utilisez `Enabled` une fois que la sortie du dry-run semble correcte.

### Changement de statut

Vous pouvez basculer entre deux statuts quelconques sur le formulaire d'édition. Passer de Dry Run à Enabled ne ré-exécute pas rétroactivement les actions du dry-run - celles-ci restent dans l'historique du dry-run. Les nouveaux déclencheurs à partir de ce moment s'exécutent en réel.

Passer de Enabled à Disabled en cours d'exécution n'interrompt **pas** une exécution en cours. Le déclencheur en cours d'exécution se termine (avec ce qu'il a déjà commencé) ; le déclencheur suivant est abandonné car l'agent est désormais Disabled.

### Statut en cas de problème de facturation

Si la facturation de votre tenant devient invalide, tous les agents sont effectivement en pause quel que soit le statut enregistré - les déclencheurs sont abandonnés avec `BILLING_INVALID` jusqu'à ce que la facturation soit rétablie. Le champ de statut enregistré n'est pas modifié ; le dispatcher refuse simplement de s'exécuter. Voir [Plans and Eligibility](#plans-and-eligibility).
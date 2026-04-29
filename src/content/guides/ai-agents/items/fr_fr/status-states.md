Un agent a l'un des trois statuts :

### Désactivé

L'agent est éteint. Aucun déclencheur n'est traité et l'agent n'apparaît pas dans le chemin de distribution. Son historique d'exécution, ses analyses et sa mémoire restent - si vous le réactivez plus tard, les données historiques sont toujours là.

Use `Disabled` when:
- You want to take an agent out of rotation without losing it.
- An agent is misbehaving and you need to stop it immediately while you investigate.
- You are seasonally rotating agents in and out (e.g. a holiday-only greeter).

### Exécution à blanc - défaut pour les nouveaux agents

L'agent s'exécute de bout en bout - il traite les déclencheurs, appelle le LLM, sélectionne les appels d'outils, calcule les justifications et la confiance - mais **aucune action réelle n'est effectuée**. Chaque exécution est enregistrée avec le badge **Dry Run** dans [Historique d'exécution](#run-history).

Use `Dry Run` when:
- A new agent is just out of the box. Every starter template lands in dry-run.
- You have edited the prompt or changed the trigger set and want to see how the change plays out before committing.
- You are running a [test run / replay](#test-runs-replays) (replays force dry-run regardless of agent status).

The platform charges tokens for dry-run runs - the LLM call still happens, only the side-effects are skipped. Budget caps apply to dry-run too. See [Aperçu des budgets](#budgets-overview).

### Activé

L'agent prend des mesures réelles. Les appels d'outils s'exécutent - ou sont mis en file d'attente pour [approbation](#approval-workflow) si l'action est soumise à approbation.

Use `Enabled` after dry-run output looks correct.

### Changement de statut

Vous pouvez basculer entre n'importe lesquels des deux statuts dans le formulaire d'édition. Passer de `Dry Run` à `Enabled` ne réexécute pas rétroactivement les actions effectuées en dry-run - celles-ci restent comme historique en dry-run. Les nouveaux déclencheurs à partir de ce moment s'exécutent en direct.

Passer de `Enabled` à `Disabled` au milieu d'une exécution n'interrompt **pas** une exécution en cours. Le déclencheur en cours d'exécution se termine (avec ce qu'il a déjà commencé) ; le déclencheur suivant est abandonné car l'agent est maintenant `Disabled`.

### Statut en cas de problèmes de facturation

Si la facturation de votre locataire devient invalide, tous les agents sont effectivement mis en pause quel que soit le statut enregistré - les déclencheurs sont abandonnés avec `BILLING_INVALID` jusqu'à ce que la facturation soit rétablie. Le champ de statut enregistré n'est pas modifié ; le répartiteur refuse simplement d'exécuter. Voir [Forfaits et éligibilité](#plans-and-eligibility).
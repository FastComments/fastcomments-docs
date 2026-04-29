Cliquer sur **Voir** sur une ligne dans [Historique des exécutions](#run-history) ouvre la page de détails par exécution. C'est ici que vous lisez le raisonnement de l'agent et jugez ses décisions.

### En haut : résumé de l'exécution

- **Agent** - quel agent a été exécuté.
- **When** - horodatage.
- **Status** - Started / Success / Error, plus le **badge Exécution simulée** si applicable.
- **Cost** - coût par exécution dans la devise de votre tenant.
- **Cost per action** - coût divisé par le nombre d'actions non en attente, utile pour repérer des exécutions anormalement coûteuses.

### Actions effectuées

Une liste, dans l'ordre, de chaque appel d'outil effectué par l'exécution. Chaque entrée affiche :

- **Action label** - "A écrit un commentaire", "A marqué un commentaire comme spam", "A banni un utilisateur", etc. Le libellé est mappé depuis l'enum du type d'action.
- **Reference ID** - l'ID du commentaire, de l'utilisateur ou du badge affecté, affiché en texte monospace (pas un hyperlien).
- **Agent reasoning** - la justification fournie par l'agent avec l'appel.
- **Confidence** - la confiance auto-évaluée de l'agent, affichée en pourcentage.
- **badge en attente d'approbation** - si l'action est mise en file d'attente dans la [boîte de réception des approbations](#approval-workflow) au lieu d'être exécutée.

Si l'exécution n'a effectué aucune action, la section affiche : "Aucune action n'a été effectuée pendant cette exécution."

### Transcription LLM

Sous les actions, la transcription complète de la conversation de l'agent avec le LLM :

- **Système** - le prompt système (suffixe de plateforme + votre prompt initial + lignes directrices communautaires).
- **Utilisateur** - le message de contexte décrivant le déclencheur.
- **Assistant** - les réponses du modèle, y compris les appels d'outil.
- **Outil** - le résultat de l'outil renvoyé au modèle (par exemple, ce que `search_memory` a retourné).

Les messages longs sont réductibles ; cliquez sur **Afficher** / **Masquer** pour les consulter.

### Lecture des transcriptions

La transcription est la page la plus importante pour l'ajustement. Quand l'agent prend une décision avec laquelle vous êtes en désaccord, relisez-la pour voir :

- Ce que le modèle **a vu** (le message de contexte Utilisateur).
- Ce que le modèle **a décidé** (les appels d'outil de l'Assistant).
- Ce que le modèle **a considéré** (les résultats d'outil - par exemple, l'agent a-t-il réellement appelé `search_memory` et a-t-il trouvé quelque chose avant de bannir).

Si le modèle commet systématiquement le même type d'erreur, modifiez le [prompt initial](#personality-prompt) — ou utilisez [Affiner les prompts](#refining-prompts) à partir d'une approbation rejetée.

### Références d'action

Les Reference ID sont affichés en texte monospace (pas des hyperliens) :

- Commentaires : l'ID du commentaire.
- Utilisateurs : l'ID de l'utilisateur.
- Badges : l'ID du badge.

Vous pouvez copier l'ID pour rechercher l'enregistrement affecté dans la page de modération/administration concernée.

### Ce qui manque en exécution simulée

Les exécutions simulées montrent les **mêmes** actions, justifications et scores de confiance. La seule différence est le **badge Exécution simulée** sur la ligne de statut. Les Reference ID pour les commentaires / utilisateurs / badges sont toujours affichés - l'agent ne les a simplement pas affectés.

### Erreurs

Pour les exécutions en état `Error`, la page de détails affiche le message d'erreur sous-jacent. Erreurs courantes :

- **No LLM API key configured** - mauvaise configuration du tenant ou de la plateforme.
- **LLM call timeout** - le fournisseur LLM était lent ou indisponible.
- **Tool dispatch failure** - l'agent a choisi un outil avec de mauvais arguments (par ex., un ID de commentaire qui n'existe plus).
- **Budget exhausted mid-run** - le plafond de l'agent a été atteint pendant l'exécution. L'exécution a été arrêtée.

Les erreurs n'annulent pas les actions partielles - tout appel d'outil terminé avant l'erreur reste en l'état.
Cliquer sur **View** sur une ligne dans [Historique des exécutions](#run-history) ouvre la page de détail par exécution. C'est ici que vous lisez le raisonnement de l'agent et jugez ses décisions.

### En haut : résumé de l'exécution

- **Agent** - quel agent a exécuté.
- **When** - horodatage.
- **Status** - Started / Success / Error, plus le badge **Exécution à blanc** si applicable.
- **Cost** - coût par exécution dans la devise de votre locataire.
- **Cost per action** - coût divisé par le nombre d'actions non en attente, utile pour repérer les exécutions anormalement coûteuses.

### Actions effectuées

Une liste, dans l'ordre, de chaque appel d'outil effectué pendant l'exécution. Chaque entrée montre :

- **Action label** - « A écrit un commentaire », « A marqué un commentaire comme pourriel », « A banni un utilisateur », etc. Le libellé provient de l'énumération des types d'action.
- **Reference ID** - l'ID du commentaire, de l'utilisateur ou de l'insigne affecté, affiché en texte monospace (pas un hyperlien).
- **Agent reasoning** - la justification fournie par l'agent avec l'appel.
- **Confidence** - l'auto-évaluation de confiance de l'agent, affichée en pourcentage.
- **Pending approval** badge - si l'action est mise en file d'attente dans la [boîte de réception des approbations](#approval-workflow) au lieu d'être exécutée.

Si l'exécution n'a pris aucune action, la section indique : "Aucune action n'a été effectuée pendant cette exécution."

### Transcription du LLM

Sous les actions, la transcription complète de la conversation de l'agent avec le LLM :

- **System** - l'invite système (suffixe de la plateforme + votre invite initiale + les directives de la communauté).
- **User** - le message de contexte décrivant le déclencheur.
- **Assistant** - les réponses du modèle, y compris les appels d'outils.
- **Tool** - le résultat de l'outil renvoyé au modèle (par ex., ce que `search_memory` a retourné).

Les messages longs sont réductibles ; cliquez sur **Développer** / **Réduire** pour voir.

### Lire les transcriptions

La transcription est la page la plus importante pour le réglage. Lorsque l'agent prend une décision avec laquelle vous n'êtes pas d'accord, relisez-la pour voir :

- Ce que le modèle **a vu** (le message de contexte de l'Utilisateur).
- Ce que le modèle **a décidé** (les appels d'outils de l'Assistant).
- Ce que le modèle **a considéré** (tous les résultats d'outils - par ex., l'agent a-t-il réellement appelé `search_memory` et a-t-il trouvé quelque chose avant de bannir).

Si le modèle commet systématiquement le même type d'erreur, modifiez l'[invite initiale](#personality-prompt) — ou utilisez [Affiner les invites](#refining-prompts) à partir d'une approbation rejetée.

### Références d'action

Les ID de référence sont affichés en texte monospace (pas des hyperliens) :

- Commentaires : l'ID du commentaire.
- Utilisateurs : l'ID de l'utilisateur.
- Insignes : l'ID de l'insigne.

Vous pouvez copier l'ID pour rechercher l'enregistrement affecté dans la page de modération/admin correspondante.

### Ce qui manque dans l'exécution à blanc

Les exécutions à blanc affichent les **mêmes** actions, justifications et scores de confiance. La seule différence est le badge **Exécution à blanc** sur la ligne de statut. Les ID de référence pour les commentaires / utilisateurs / insignes sont toujours affichés — l'agent ne les a simplement pas affectés.

### Erreurs

Pour les exécutions en état `Error`, la page de détail affiche le message d'erreur sous-jacent. Erreurs courantes :

- **Aucune clé API LLM configurée** - mauvaise configuration du locataire ou de la plateforme.
- **Appel LLM expiré** - le fournisseur LLM était lent ou indisponible.
- **Échec d'envoi d'outil** - l'agent a choisi un outil avec des arguments invalides (p. ex., un ID de commentaire qui n'existe plus).
- **Budget épuisé en cours d'exécution** - le plafond de l'agent a été atteint pendant l'exécution. L'exécution a été interrompue.

Les erreurs n'annulent pas les actions partielles - tous les appels d'outils effectués avant l'erreur restent.
Une **exécution de test** (appelée aussi **replay**) exécute l'agent contre une fenêtre de commentaires historiques **sans effectuer d'actions réelles**. C'est le moyen le plus rapide de prévisualiser le comportement de l'agent avant de le mettre en service.

Accessible depuis la page de la liste des agents via le bouton **Exécution de test** dans la ligne de chaque agent.

### Ce que ça fait

La plateforme :

1. Sélectionne un échantillon de commentaires historiques correspondant au périmètre de l'agent, dans la fenêtre que vous choisissez.
2. Pour chaque commentaire, exécute l'agent de bout en bout comme si le commentaire venait d'être publié - même contexte, même appel LLM, même sélection d'outils, mêmes justifications et mêmes scores de confiance.
3. Enregistre chaque exécution comme un dry-run, étiqueté pour rester groupé avec le replay dont il provient et exclu des vues d'exécutions en direct.
4. **Compare** le verdict de l'agent avec ce qui s'est réellement passé pour le commentaire - a-t-il ensuite été approuvé, marqué comme spam, supprimé, bloqué par le moteur anti-spam, etc.

Le résultat est une différence par commentaire : « Le replay indique que l'agent marquerait ceci comme spam, mais le commentaire est actuellement approuvé et propre. »

### Configuration

La page d'exécution de test comporte un seul champ :

- **Jours de commentaires historiques à évaluer** - un champ numérique `days` entre 1 et 90. Les commentaires plus anciens ne sont pas admissibles.

La taille de l'échantillon et le plafond maximal ne sont **pas exposés dans l'interface** - les deux sont des valeurs par défaut côté serveur appliquées par plan. La page affiche des champs d'information :

- **Commentaires correspondant dans la fenêtre** - combien de commentaires seraient considérés.
- **Jusqu'à N commentaires de cette fenêtre seront traités** - la taille effective de l'échantillon donnée par le plafond côté serveur.
- **Coût estimé** - dans la monnaie de votre tenant.

### Limite de fréquence

Chaque utilisateur est limité à **10 exécutions de test par 24 heures** (limitation via la clé `replay-create:${requestedBy}`). Le bouton affiche une infobulle lorsque vous avez atteint la limite (« Vous avez atteint 10 exécutions de test au cours des dernières 24 heures. »).

### Concurrence

Un seul replay peut être actif par agent à la fois. Le lancement d'un deuxième replay pendant qu'un autre est en cours vous redirige vers celui en cours.

### Lecture des résultats

Lorsque le replay se termine, la page de résultat affiche des onglets :

- **Deltas** (actif par défaut) - le verdict du replay diffère de la réalité. (Le plus intéressant - « l'agent aurait marqué ce commentaire comme spam, mais le commentaire a été approuvé et est correct ».)
- **Matches** - le verdict du replay correspond à ce qui s'est réellement passé. (Rassurant - l'agent est d'accord avec la réalité.)
- **No action** - l'agent du replay a décidé de ne rien faire. (Parfois la bonne réponse; parfois l'agent a manqué quelque chose.)
- **All** - tous les résultats, indépendamment de la classification.

Pour chaque commentaire dans n'importe quel onglet :

- **Prior outcome** - la classification de ce qui s'est réellement passé : **POSITIVE**, **NEGATIVE**, ou **INDETERMINATE**, avec des **Evidence** (« Comment marked deleted at {date} », « Moteur : bayes », etc.).
- **Replay agent would** - l'action choisie par l'agent.
- **Why** - la justification.
- **Confidence** - affichée en pourcentage.

### Pourquoi les replays forcent le mode dry-run

Un replay contre un commentaire qui a été supprimé il y a quatre mois ne devrait pas le supprimer rétroactivement - il est déjà supprimé. Un replay contre un commentaire que l'agent voudrait maintenant approuver ne devrait pas modifier l'état actuel du commentaire. Le replay est un outil de prévisualisation. Forcer le dry-run est ce qui rend sûr l'exécution d'un replay contre n'importe quelle fenêtre historique.

### Reproductibilité

Les replays figent la configuration de l'agent au moment du démarrage du replay. Les modifications ultérieures de l'agent ne changent pas les résultats du replay - la page de résultats reste stable comme enregistrement de ce que cette version de l'agent aurait fait.

### Quand les budgets arrêtent un replay

Les replays sont soumis à :

- Leur propre **plafond maximal** (défini sur le formulaire de replay).
- Les plafonds de budget journaliers et mensuels de l'agent.
- Les plafonds de budget journaliers et mensuels du tenant.

Le premier atteint interrompt le replay avec un code d'erreur spécifique. Tous les résultats par commentaire produits avant l'interruption sont conservés dans [Historique d'exécution](#run-history).

### Comment les replays s'exécutent

Les replays s'exécutent en arrière-plan, pas de manière synchrone. Après avoir cliqué sur « Démarrer l'exécution de test », le replay est mis en file d'attente et un worker le prend en charge. Un long replay peut durer plusieurs minutes. La page de résultat interroge périodiquement le statut et affiche la progression (nombre traités, dépense jusqu'à présent) au fur et à mesure.

Si un worker meurt en cours de replay, la plateforme remet automatiquement le replay en file pour qu'il reprenne au passage suivant. Un bref incident n'abandonne jamais un replay.

### Ce que le replay ne fait pas

- **N'honore pas les [trigger delays](#trigger-deferred-delay).** Les replays s'exécutent immédiatement, pas 30 minutes plus tard.
- **N'écrit pas en mémoire.** Les agents de replay n'enregistrent pas de notes en mémoire, même si leur logique le ferait normalement.
- **Ne déclenche pas de webhooks.** Les triggers produits par le replay ne génèrent pas d'événements webhook `trigger.succeeded` / `trigger.failed`.
- **N'exclut pas les commentaires déjà rejoués.** Lancer un deuxième replay contre la même fenêtre couvre les mêmes commentaires.

### Voir aussi

- [Refining Prompts](#refining-prompts) - le flux de travail d'édition itérative qui s'associe bien aux replays.
- [Dry-Run Mode](#dry-run-mode) - la même idée, appliquée au trafic en direct.
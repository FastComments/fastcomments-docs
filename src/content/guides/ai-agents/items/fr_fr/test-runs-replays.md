Une **exécution de test** (aussi appelée **replay**) exécute l'agent sur une fenêtre de commentaires historiques **sans prendre d'actions réelles**. C'est le moyen le plus rapide de prévisualiser le comportement de l'agent avant de le mettre en production.

Accessible depuis la page de liste des agents via le bouton **Test run** sur chaque ligne d'agent.

### Ce que ça fait

La plateforme :

1. Sélectionne un échantillon de commentaires historiques correspondant au périmètre de l'agent, dans la fenêtre que vous choisissez.
2. Pour chaque commentaire, exécute l'agent de bout en bout comme si le commentaire venait juste d'être posté - même contexte, même appel LLM, même sélection d'outils, mêmes justifications et mêmes scores de confiance.
3. Enregistre chaque exécution comme un dry-run, étiquetée de sorte qu'elle reste groupée avec le replay dont elle provient et exclue des vues des exécutions en direct.
4. **Compare** le verdict de l'agent avec ce qui est réellement arrivé au commentaire - a-t-il été ultérieurement approuvé, marqué comme spam, supprimé, bloqué par le moteur anti-spam, etc.

Le résultat est un diff par commentaire : "L'agent en replay aurait marqué ceci comme spam, mais le commentaire est actuellement approuvé et propre."

### Configuration

La page du test-run contient une seule entrée :

- **Days of historical comments to evaluate** - un champ numérique `days` entre 1 et 90. Les commentaires plus anciens ne sont pas éligibles.

La taille de l'échantillon et le plafond maximal ne sont **pas exposés dans l'UI** - ce sont des valeurs par défaut côté serveur appliquées par plan. La page affiche des champs d'information :

- **Matching comments in window** - combien de commentaires seraient considérés.
- **Up to N comments from this window will be processed** - la taille d'échantillon effective compte tenu du plafond côté serveur.
- **Estimated cost** - dans la monnaie de votre tenant.

### Limite de fréquence

Chaque utilisateur est limité à **10 test runs par période de 24 heures** (rate-limited via key `replay-create:${requestedBy}`). Le bouton affiche une infobulle lorsque vous avez atteint la limite ("You've reached 10 test runs in the last 24 hours.").

### Concurrence

Un seul replay peut être actif par agent à la fois. Lancer un second replay alors qu'un replay est en cours vous redirige vers celui en cours.

### Lecture des résultats

Quand le replay se termine, la page de résultats affiche des onglets :

- **Deltas** (default-active) - le verdict de l'agent en replay diffère de la réalité. (Le plus intéressant - "l'agent aurait marqué ce commentaire comme spam, mais le commentaire a été approuvé et est correct".)
- **Matches** - le verdict de l'agent en replay correspond à ce qui est réellement arrivé. (Rassurant - l'agent est d'accord avec la réalité.)
- **No action** - l'agent en replay a décidé de ne rien faire. (Parfois la bonne réponse ; parfois l'agent est passé à côté de quelque chose.)
- **All** - tous les résultats indépendamment de la classification.

Pour chaque commentaire dans n'importe quel onglet :

- **Prior outcome** - la classification de ce qui s'est réellement passé : **POSITIVE**, **NEGATIVE**, ou **INDETERMINATE**, avec **Evidence** ("Comment marked deleted at {date}", "Engine: bayes", et ainsi de suite).
- **Replay agent would** - l'action choisie par l'agent.
- **Why** - la justification.
- **Confidence** - affichée en pourcentage.

### Pourquoi les replays forcent le dry-run

Un replay contre un commentaire supprimé il y a quatre mois ne doit pas le supprimer rétroactivement - il est déjà supprimé. Un replay contre un commentaire que l'agent voudrait maintenant approuver ne doit pas changer l'état actuel du commentaire. Le replay est un outil de prévisualisation. Forcer le dry-run est ce qui rend sûr l'exécution d'un replay contre n'importe quelle fenêtre historique.

### Reproductibilité

Les replays figent la configuration de l'agent au moment où le replay a été lancé. Les modifications ultérieures de l'agent ne changent pas les résultats du replay - la page de résultats reste stable comme enregistrement de ce que cette version de l'agent aurait fait.

### Quand les budgets arrêtent un replay

Les replays sont soumis à :

- Leur propre **hard cap** (défini sur le formulaire de replay).
- Les plafonds de budget journaliers et mensuels de l'agent.
- Les plafonds de budget journaliers et mensuels du tenant.

Le premier plafond atteint interrompt le replay avec un code d'erreur spécifique. Tous les résultats par commentaire produits avant l'interruption sont préservés dans [Run History](#run-history).

### Comment les replays s'exécutent

Les replays s'exécutent en arrière-plan, pas de manière synchrone. Après avoir cliqué sur "Start test run", le replay est mis en file d'attente et un worker le prend en charge. Un long replay peut durer plusieurs minutes. La page de résultats interroge périodiquement et affiche la progression (nombre traités, dépenses à ce jour) au fur et à mesure.

Si un worker meurt en cours de replay, la plateforme remet automatiquement le replay en file d'attente pour qu'il reprenne au passage suivant. Un petit incident ne laisse jamais un replay orphelin.

### Ce que le replay ne fait pas

- **Ne respecte pas les [trigger delays](#trigger-deferred-delay).** Les replays s'exécutent immédiatement, pas 30 minutes plus tard.
- **N'écrit pas en mémoire.** Les agents en replay ne sauvegardent pas de notes en mémoire, même si leur logique le ferait normalement.
- **Ne déclenche pas de webhooks.** Les triggers produits par les replays ne génèrent pas d'événements de webhook `trigger.succeeded` / `trigger.failed`.
- **N'exclut pas les commentaires déjà rejoués.** Lancer un second replay sur la même fenêtre couvre les mêmes commentaires.

### Voir aussi

- [Refining Prompts](#refining-prompts) - le workflow d'édition itérative qui se marie bien avec les replays.
- [Dry-Run Mode](#dry-run-mode) - la même idée, appliquée au trafic en direct.
Par défaut, un agent s'exécute **immédiatement** après que son déclencheur s'active. Le champ **Délai avant exécution** sur le formulaire d'édition modifie cela : la plateforme met le déclencheur en file d'attente et exécute l'agent à l'heure prévue.

### Quand utiliser un délai

- **Déclencheurs par seuil de signalement** - les signalements arrivent souvent par rafales. Un délai de 10 à 30 minutes permet à la situation de se stabiliser afin que l'agent agisse sur le nombre final de signalements plutôt que sur le moment d'arrivée.
- **Déclencheurs par seuil de votes** - même logique, en particulier pour les brigades de downvotes.
- **Résumé de fil** - le [Modèle Thread Summarizer](#template-thread-summarizer) utilise par défaut un délai de 30 minutes afin qu'il résume une conversation qui a eu le temps de se développer, et non un fil n'ayant que deux réponses.
- **Temps de refroidissement / réévaluation** - « 24 heures après le verrouillage d'un commentaire, envisager de le déverrouiller. »

### Configuration

- **Champ** : Délai avant exécution.
- **Plage** : 0 à 2,592,000 secondes (30 jours).
- **Unités** : secondes, minutes, heures ou jours.

### Idempotence

La file différée ne déduplique pas les déclencheurs. Deux signalements arrivant à 1 seconde d'intervalle pour un agent avec un délai de 30 minutes planifieront tous deux une exécution 30 minutes plus tard, et l'agent s'exécutera **deux fois**, les deux fois sur un contexte (pour l'essentiel) identique. Si vous souhaitez une sémantique « au plus une exécution par fenêtre », l'agent doit l'appliquer lui-même — typiquement en écrivant une [note en mémoire](#tools-overview) lors de la première exécution et en la vérifiant lors des exécutions suivantes.

### Note sur le coût

Les déclencheurs différés sont enregistrés **avant** leur exécution. Une rafale de déclencheurs sur un agent avec un délai élevé peut s'accumuler dans la file sans consommer de jetons ; le coût n'est facturé que lorsque le cron les déclenche. Utilisez [Run History](#run-history) et [Drop Reasons](#drop-reasons) pour voir à quelle fréquence les déclencheurs différés s'exécutent réellement par rapport à ceux qui sont abandonnés à l'exécution pour des raisons budgétaires.

### Les relectures n'honorent pas le délai

La fonctionnalité [Exécutions de test (Replays)](#test-runs-replays) exécute l'agent immédiatement sur des commentaires historiques — elle n'attend pas le délai configuré. Considérez cela comme une fonctionnalité : les relectures servent à prévisualiser ce que l'agent **ferait** compte tenu du contexte, et non à reproduire la planification en temps réel.
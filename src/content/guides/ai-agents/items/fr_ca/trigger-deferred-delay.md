Par défaut un agent s'exécute **immédiatement** après le déclenchement de son trigger. Le champ **Délai avant exécution** sur le formulaire d'édition modifie cela : la plateforme met le trigger en file d'attente et exécute l'agent à l'heure planifiée.

### Quand utiliser un délai

- **Flag-threshold triggers** - les flags arrivent souvent par rafales. Un délai de 10-30 minutes permet à la situation de se stabiliser afin que l'agent agisse sur le nombre final de flags plutôt que sur le moment d'arrivée.
- **Vote-threshold triggers** - même logique, particulièrement en cas de brigades de downvotes.
- **Thread summarization** - le [Thread Summarizer template](#template-thread-summarizer) utilise par défaut un délai de 30 minutes afin de résumer une conversation qui a eu le temps de se développer, et non un fil à deux réponses.
- **Cool-down / re-evaluation** - "24 hours after a comment is locked, consider whether to unlock it."

### Configuration

- **Field**: Delay before running.
- **Range**: 0 to 2,592,000 seconds (30 days).
- **Units**: Seconds, minutes, hours, or days.

### Idempotence

La file différée ne déduplique pas les triggers. Deux flags arrivant à 1 seconde d'intervalle sur un agent avec un délai de 30 minutes planifieront tous deux une exécution 30 minutes plus tard, et l'agent s'exécutera **deux fois**, les deux fois contre (principalement) le même contexte. Si vous voulez une sémantique de "au plus une exécution par fenêtre", l'agent doit l'imposer — typiquement en écrivant une [memory note](#tools-overview) lors de la première exécution et en la vérifiant lors des exécutions suivantes.

### Note de coût

Les triggers différés sont enregistrés **avant** leur exécution. Une rafale de triggers sur un agent à long délai peut s'accumuler dans la file d'attente sans consommer de tokens ; le coût est payé uniquement lorsque le cron les dispatch. Utilisez [Run History](#run-history) et [Drop Reasons](#drop-reasons) pour voir à quelle fréquence les triggers différés s'exécutent réellement par rapport à ceux qui sont abandonnés au moment de l'exécution pour des raisons budgétaires.

### Replay does not honor delay

La fonctionnalité [Test Runs (Replays)](#test-runs-replays) exécute l'agent immédiatement contre des commentaires historiques - elle n'attend pas le délai configuré. Considérez cela comme une fonctionnalité : les relectures servent à prévisualiser ce que l'agent **ferait** compte tenu du contexte, et non à reproduire la planification en temps réel.
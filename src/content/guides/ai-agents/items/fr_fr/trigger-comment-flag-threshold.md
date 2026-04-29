Se déclenche lorsque le nombre de signalements d'un commentaire atteint **exactement** le seuil configuré.

### Configuration requise

- **Seuil de signalement** - entier >= 1. Le déclencheur s’active au moment où `flagCount === flagThreshold`. Il ne se réactive pas sur des signalements supplémentaires dépassant le seuil.

Si le seuil est 3 et que trois utilisateurs signalent le commentaire, l'agent se déclenche une fois au troisième signalement. Un quatrième, cinquième ou sixième signalement ne le déclenchera pas à nouveau.

### Contexte que reçoit l'agent

- Le commentaire signalé.
- Contexte optionnel du fil / historique utilisateur / page tel que configuré.
- Le nombre de signalements figure dans le bloc du commentaire sous la forme `Flag Count: N`.

### Remarques

- Le déclencheur ne s'active que lorsque le commentaire franchit le seuil par le bas via le chemin de gestion des signalements de la plateforme (là où `didIncrement === true`). Les écritures directes en base de données qui définissent `flagCount` sur la valeur du seuil ne l'activent pas ; les signalements au-delà du seuil ne le réactivent pas non plus.
- Il n'indique pas qui a signalé le commentaire — les signalements sont anonymes pour l'agent. Si vous souhaitez examiner les utilisateurs ayant signalé, récupérez-les depuis vos propres données.
- Un délai d'activation (voir [Deferred Triggers](#trigger-deferred-delay)) est *fortement* recommandé pour ce déclencheur — les signalements arrivent souvent par rafales lors d'un fil animé, et un petit délai permet à la situation de se stabiliser avant que l'agent n'agisse.

### Utilisations courantes

- **Révision de modération** - un commentaire signalé est le signal canonique « les humains pensent que cela pourrait être problématique ». Le [Moderator template](#template-moderator) s'abonne à ce déclencheur par défaut avec un seuil de signalement de 3.
- **Augmentation de la file de pré-modération** - l'agent effectue une première passe et marque soit le commentaire pour modération (avec `mark_comment_reviewed`) soit l'escalade pour une action ultérieure.
- **Anti-brigading** - combinez ce déclencheur avec le [contexte historique utilisateur](#context-options) et laissez l'agent prendre en compte les bannissements précédents / signaux de contenu dupliqué avant d'agir.

### Recommandations d'association

Abonnez-vous aux **deux** `COMMENT_ADD` et `COMMENT_FLAG_THRESHOLD` si vous voulez un agent de modération qui attrape les cas évidents dès la première vue et réévalue les cas limites une fois les signalements accumulés. Les deux événements se déclenchent indépendamment - l'agent s'exécutera deux fois si les deux sont abonnés et se déclenchent, mais la deuxième exécution verra l'état désormais signalé.

---
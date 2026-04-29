Se déclenche lorsque le nombre de signalements d'un commentaire atteint **exactement** le seuil configuré.

### Configuration requise

- **Seuil de signalement** - entier >= 1. Le déclencheur se produit au moment où `flagCount === flagThreshold`. Il ne se déclenche pas à nouveau pour des signalements ultérieurs dépassant le seuil.

Si le seuil est de 3 et que trois utilisateurs signalent le commentaire, l'agent se déclenche une seule fois lors du troisième signalement. Un quatrième, cinquième ou sixième signalement ne le re-déclenche pas.

### Contexte que reçoit l'agent

- Le commentaire signalé.
- Contexte optionnel du fil / de l'historique de l'utilisateur / de la page, selon la configuration.
- Le nombre de signalements figure dans le bloc du commentaire sous la forme `Flag Count: N`.

### Remarques

- Le déclencheur ne se produit que lorsque le commentaire franchit le seuil par le bas via le chemin de gestion des signalements de la plateforme (là où `didIncrement === true`). Les écritures directes en base de données qui définissent `flagCount` à la valeur du seuil ne le déclenchent pas ; de même, des signalements au-delà du seuil ne le déclenchent pas à nouveau.
- Il n'indique pas qui a signalé le commentaire - les signalements sont anonymes pour l'agent. Si vous voulez identifier les utilisateurs qui ont signalé, récupérez-les depuis vos propres données.
- Un délai de déclenchement (voir [Déclencheurs différés](#trigger-deferred-delay)) est *fortement* recommandé pour ce déclencheur — les signalements arrivent souvent par rafales lors d'un fil animé, et un petit délai permet à la situation de se stabiliser avant que l'agent n'agisse.

### Utilisations courantes

- **Revue de modération** - un commentaire signalé est le signal canonique « les humains pensent que cela pourrait être problématique ». Le [Modèle du modérateur](#template-moderator) s'abonne à ce déclencheur par défaut avec un seuil de signalement de 3.
- **Augmentation de la file de pré-modération** - l'agent effectue un premier passage et marque soit le commentaire pour modération (avec `mark_comment_reviewed`) soit l'escalade pour une analyse plus approfondie.
- **Anti-brigading** - combinez ce déclencheur avec le [contexte d'historique de l'utilisateur](#context-options) et laissez l'agent voir les interdictions antérieures/signaux de contenu dupliqué avant d'agir.

### Recommandations d'association

Abonnez-vous aux **deux** événements `COMMENT_ADD` et `COMMENT_FLAG_THRESHOLD` si vous voulez qu'un agent de modération attrape les cas évidents au premier coup d'œil et réévalue les cas limites une fois que les signalements s'accumulent. Les deux événements se déclenchent indépendamment — l'agent s'exécutera deux fois si les deux sont abonnés et se déclenchent, mais la deuxième exécution verra l'état désormais signalé.
Pour la plupart des sites, la façon la plus simple d'ajouter des commentaires est d'attacher le champ `FastComments` à vos types de contenu. Allez à `Structure > Content types > [type] > Manage fields` et ajoutez le champ.

Chaque entité qui possède le champ obtient :

- Un **commutateur d'état** permettant aux éditeurs d'activer ou de désactiver les commentaires par entité.
- Un **identifiant personnalisé** facultatif vous permettant d'utiliser un ID stable qui n'est pas lié au chemin de l'entité Drupal.

Le bloc principal `FastComments Widget` reconnaît ce champ et ignorera les entités qui y sont déjà rattachées. Ainsi, vous pouvez combiner des commentaires par entité avec le bloc sans voir le widget deux fois sur la même page.

---
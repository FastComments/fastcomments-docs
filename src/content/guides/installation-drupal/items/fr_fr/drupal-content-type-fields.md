Pour la plupart des sites, la façon la plus simple d'ajouter des commentaires est d'attacher le champ `FastComments` à vos types de contenu. Allez dans `Structure > Content types > [type] > Manage fields` et ajoutez le champ.

Chaque entité qui possède ce champ reçoit :

- Une **bascule d'état** permettant aux éditeurs d'activer ou de désactiver les commentaires pour chaque entité.
- Un **identifiant personnalisé** optionnel afin que vous puissiez utiliser un ID stable qui n'est pas lié au chemin de l'entité Drupal.

Le bloc principal `FastComments Widget` connaît ce champ et ignorera les entités qui l'ont déjà attaché. De cette façon, vous pouvez mélanger des commentaires par entité avec le bloc sans voir le widget deux fois sur la même page.

---
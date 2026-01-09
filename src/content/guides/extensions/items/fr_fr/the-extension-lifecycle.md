Le script pour chaque extension est récupéré et invoqué avant que le widget de commentaires ne commence à récupérer le premier jeu de commentaires et à rendre l'interface utilisateur.

Au chargement initial, les données suivantes seront ajoutées à l'objet extension :

- `config` - Une référence à l'objet `config`.
- `translations` - Une référence à l'objet `translations`.
- `commentsById` - Une référence à tous les commentaires par id.
- `root` - Une référence au nœud DOM racine.

Les extensions doivent redéfinir les fonctions souhaitées, que le widget de commentaires invoquera aux moments appropriés.
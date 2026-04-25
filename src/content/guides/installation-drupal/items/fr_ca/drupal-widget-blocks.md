Le module inclut plusieurs blocs que vous pouvez placer depuis `Structure > Block layout` (`/admin/structure/block`).

- **FastComments Widget** - Le widget principal de commentaires. Détecte automatiquement l'entité courante. Il ignorera les entités qui ont déjà le champ FastComments attaché, donc vous ne verrez pas de widgets en double sur la même page.
- **FastComments Live Chat** - Chat en streaming en temps réel. Peut être placé à côté du champ de commentaires sur la même page.
- **FastComments Collab Chat** - Annotation et discussion par sélection de texte.
- **FastComments Image Chat** - Annotation basée sur des coordonnées sur les images. Les visiteurs cliquent sur une image pour laisser des commentaires liés à des emplacements spécifiques.
- **FastComments Recent Comments** - Affiche les commentaires récents sur l'ensemble de votre site. Le nombre affiché est configurable sur le bloc.
- **FastComments Top Pages** - Affiche les pages de votre site ayant le plus de commentaires.

Les blocs axés sur le contenu (Live Chat, Collab Chat, Image Chat) détectent automatiquement l'entité courante, et reviennent à un identifiant basé sur le chemin pour les pages non liées à une entité. Cela signifie qu'ils fonctionnent sur les pages de taxonomie, les vues et les routes personnalisées sans configuration supplémentaire.

---
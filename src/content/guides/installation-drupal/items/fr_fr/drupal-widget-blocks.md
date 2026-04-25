---
Le module propose plusieurs blocs que vous pouvez placer depuis `Structure > Block layout` (`/admin/structure/block`).

- **FastComments Widget** - Le widget de commentaires principal. Il détecte automatiquement l'entité actuelle. Il ignore les entités qui ont déjà le champ FastComments attaché, de sorte que vous ne verrez pas de widgets en double sur la même page.
- **FastComments Live Chat** - Chat en streaming en temps réel. Peut être placé à côté du champ de commentaire sur la même page.
- **FastComments Collab Chat** - Annotation et discussion par sélection de texte.
- **FastComments Image Chat** - Annotation basée sur des coordonnées sur les images. Les visiteurs cliquent sur une image pour laisser des commentaires liés à des emplacements précis.
- **FastComments Recent Comments** - Affiche les commentaires récents sur votre site. Le nombre est configurable sur le bloc.
- **FastComments Top Pages** - Affiche les pages de votre site ayant le plus de commentaires.

Les blocs centrés sur le contenu (Live Chat, Collab Chat, Image Chat) détectent automatiquement l'entité courante et utilisent un identifiant basé sur le chemin pour les pages qui ne correspondent pas à une entité. Cela signifie qu'ils fonctionnent sur les pages de taxonomie, les vues et les routes personnalisées sans configuration supplémentaire.

---
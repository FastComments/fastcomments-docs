---
Chez FastComments, nous écrivons nos propres extensions en utilisant la même API. Vous pouvez consulter le code non minifié de ces extensions aux points de terminaison suivants :

#### Mode sombre

L'extension Mode sombre est chargée de façon conditionnelle lorsqu'une page « sombre » est détectée. Cette extension ajoute simplement du CSS au widget de commentaires. Ainsi, nous n'avons pas à charger le CSS du mode sombre lorsqu'il n'est pas nécessaire.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Modération

Lorsque l'utilisateur actuel est modérateur, nous utilisons l'extension de modération.

C'est un bon exemple pour ajouter des écouteurs d'événements basés sur les clics, effectuer des requêtes API, et ajouter des éléments au menu et aux zones de commentaires.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Chat en direct

L'extension Chat en direct (en combinaison avec d'autres configurations et styles) transforme le widget de commentaires en un composant de chat en temps réel.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js

---
---
FastComments prend en charge les webhooks uniquement pour la ressource Comment.

Nous prenons en charge les webhooks pour la création, la suppression et la mise à jour des commentaires.

Chacun de ces cas est considéré comme un événement distinct dans notre système et possède donc des sémantiques
et des structures différentes pour les événements webhook.

Un nombre quelconque de points de terminaison peut s'abonner au même événement, depuis le tableau de bord ou via l'API
(voir Gestion des webhooks via l'API). Chaque webhook est livré indépendamment.

---
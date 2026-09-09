FastComments prend en charge les webhooks uniquement pour la ressource Comment.

Nous prenons en charge les webhooks pour la création, la suppression et la mise à jour des commentaires.

Chacun de ces cas est considéré comme un événement distinct dans notre système et possède ainsi des sémantiques et des structures différentes pour les événements webhook.

Un nombre illimité de points de terminaison peut s'abonner au même événement : un webhook par domaine peut être configuré dans le tableau de bord, et d'autres abonnements peuvent être créés via l'API (voir Gestion des webhooks via l'API).
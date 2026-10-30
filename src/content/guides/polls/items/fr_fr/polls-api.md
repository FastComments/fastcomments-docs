Les sondages peuvent également être gérés depuis votre propre code avec l'API REST, par exemple pour ajouter un sondage à un commentaire depuis votre backend, fermer un sondage ou exporter les votes :

- [Poll Structure](/guide-api.html#poll-structure) décrit l'objet sondage et ses limites.  
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) et [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` lisent, remplacent, modifient et suppriment le sondage d'un commentaire.  
- [PollVote Structure](/guide-api.html#poll-vote-structure) décrit un vote unique.  
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) et [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` listent, enregistrent et suppriment les votes.

Un sondage peut également être créé avec son commentaire en passant `poll` à `POST /api/v1/comments`.

L'API suit les mêmes paramètres que le widget : les sondages doivent être activés, et les votes sans utilisateur connecté sont rejetés sauf si les votes anonymes sont autorisés. Les votes individuels sur les sondages anonymes ne peuvent pas être lus via l'API.
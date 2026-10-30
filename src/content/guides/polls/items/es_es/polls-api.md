Las encuestas también pueden gestionarse desde su propio código con la API REST, por ejemplo para añadir una encuesta a un comentario desde su backend, cerrar una encuesta o exportar los votos:

- [Poll Structure](/guide-api.html#poll-structure) describe el objeto de la encuesta y sus límites.  
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) y [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` leen, reemplazan, editan y eliminan la encuesta en un comentario.  
- [PollVote Structure](/guide-api.html#poll-vote-structure) describe un voto individual.  
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) y [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` listan, emiten y eliminan votos.

Una encuesta también puede crearse junto con su comentario pasando `poll` a `POST /api/v1/comments`.

La API sigue la misma configuración que el widget: las encuestas deben estar habilitadas, y los votos sin un usuario autenticado son rechazados a menos que se permitan votos anónimos en encuestas. Los votos individuales en encuestas anónimas no pueden leerse a través de la API.
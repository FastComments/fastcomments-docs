I sondaggi possono anche essere gestiti dal tuo codice con la REST API, ad esempio per aggiungere un sondaggio a un commento dal tuo backend, chiudere un sondaggio o esportare i voti:

- [Poll Structure](/guide-api.html#poll-structure) descrive l'oggetto sondaggio e i suoi limiti.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) e [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` leggono, sostituiscono, modificano e rimuovono il sondaggio su un commento.
- [PollVote Structure](/guide-api.html#poll-vote-structure) descrive un singolo voto.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) e [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` elencano, registrano e rimuovono i voti.

Un sondaggio può anche essere creato insieme al suo commento passando `poll` a `POST /api/v1/comments`.

L'API segue le stesse impostazioni del widget: i sondaggi devono essere abilitati e i voti senza un utente autenticato vengono rifiutati a meno che non siano consentiti voti anonimi. I voti individuali sui sondaggi anonimi non possono essere letti tramite l'API.
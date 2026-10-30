Polls kunnen ook worden beheerd vanuit je eigen code met de REST API, bijvoorbeeld om een poll toe te voegen aan een reactie vanuit je backend, een poll te sluiten, of de stemmen te exporteren:

- [Poll Structure](/guide-api.html#poll-structure) beschrijft het poll‑object en de limieten.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) en [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` lezen, vervangen, bewerken en verwijderen de poll op een reactie.
- [PollVote Structure](/guide-api.html#poll-vote-structure) beschrijft een enkele stem.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) en [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` lijst, uitbrengen en verwijderen van stemmen.

Een poll kan ook worden aangemaakt samen met de bijbehorende reactie door `poll` door te geven aan `POST /api/v1/comments`.

De API volgt dezelfde instellingen als de widget: polls moeten ingeschakeld zijn, en stemmen zonder een ingelogde gebruiker worden afgewezen tenzij anonieme poll‑stemmen zijn toegestaan. Individuele stemmen op anonieme polls kunnen niet worden gelezen via de API.
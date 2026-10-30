Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) opisuje objekt ankete in njegove omejitve.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) in [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` prebere, nadomesti, uredi in odstrani anketo pri komentarju.
- [PollVote Structure](/guide-api.html#poll-vote-structure) opisuje posamezen glas.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) in [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` izpiše, odda in odstrani glasove.

Anketo je mogoče ustvariti tudi skupaj s komentarjem tako, da pošljete `poll` v `POST /api/v1/comments`.

API sledi istim nastavitvam kot gradnik: ankete morajo biti omogočene, glasovi brez prijavljenega uporabnika so zavrnjeni, razen če so dovoljeni anonimni glasovi v anketah. Posameznih glasov v anonimnih anketah ni mogoče prebrati prek API-ja.
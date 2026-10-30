Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) opisuje objekt ankete i njegove ograničenja.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) i [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` čitaju, zamjenjuju, uređuju i uklanjaju anketu na komentaru.
- [PollVote Structure](/guide-api.html#poll-vote-structure) opisuje pojedinačni glas.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) i [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` popisuje, izbacuje i uklanja glasove.

Anketa se također može stvoriti zajedno s njenim komentarom prosljeđivanjem `poll` u `POST /api/v1/comments`.

API slijedi iste postavke kao i widget: ankete moraju biti omogućene, a glasovi bez prijavljenog korisnika se odbacuju osim ako su anonimni glasovi u anketi dopušteni. Pojedinačni glasovi u anonimnim anketama ne mogu se čitati putem API-ja.
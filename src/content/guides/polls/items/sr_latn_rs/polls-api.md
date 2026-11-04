Ankete se mogu upravljati i iz vašeg koda putem REST API-ja, na primer da dodate anketu komentaru iz svog bekenda, zatvorite anketu ili izvezete glasove:

- [Struktura ankete](/guide-api.html#poll-structure) opisuje objekat ankete i njena ograničenja.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) i [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` čitaju, zamenjuju, menjaju i uklanjaju anketu na komentaru.
- [Struktura glasanja](/guide-api.html#poll-vote-structure) opisuje jedan glas.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) i [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` izlistavaju, daju i uklanjaju glasove.

Anketa se može kreirati i zajedno sa svojim komentarom prosleđivanjem `poll` na `POST /api/v1/comments`.

API prati ista podešavanja kao i vidžet: ankete moraju biti omogućene, a glasovi bez prijavljenog korisnika se odbijaju osim ako anonimna glasanja u anketama nisu dozvoljena. Pojedinačni glasovi na anonimnim anketama se ne mogu čitati putem API-ja.
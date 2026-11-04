Ankete se također mogu upravljati iz vlastitog koda putem REST API-ja, na primjer za dodavanje ankete komentaru s vašeg backenda, zatvaranje ankete ili izvoz glasova:

- [Struktura ankete](/guide-api.html#poll-structure) opisuje objekt ankete i njena ograničenja.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) i [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` čitaju, zamjenjuju, uređuju i uklanjaju anketu na komentaru.
- [Struktura glasa u anketi](/guide-api.html#poll-vote-structure) opisuje jedan glas.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) i [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` izlistavaju, daju i uklanjaju glasove.

Anketa se također može stvoriti zajedno sa svojim komentarom prosljeđivanjem `poll` na `POST /api/v1/comments`.

API slijedi iste postavke kao widget: ankete moraju biti omogućene, a glasovi bez prijavljenog korisnika se odbijaju osim ako su anonimni glasovi u anketi dopušteni. Pojedinačni glasovi na anonimnim anketama ne mogu se čitati putem API-ja.
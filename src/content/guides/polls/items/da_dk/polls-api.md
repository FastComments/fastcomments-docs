Afstemninger (polls) kan også administreres fra din egen kode via REST API'en, f.eks. for at tilføje en afstemning til en kommentar fra din backend, lukke en afstemning eller eksportere stemmerne:

- [Poll Structure](/guide-api.html#poll-structure) beskriver afstemningsobjektet og dets begrænsninger.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) og [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` læser, erstatter, redigerer og fjerner afstemningen på en kommentar.
- [PollVote Structure](/guide-api.html#poll-vote-structure) beskriver en enkelt stemme.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) og [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` viser, afgiver og fjerner stemmer.

En afstemning kan også oprettes sammen med dens kommentar ved at sende `poll` til `POST /api/v1/comments`.

API'en følger de samme indstillinger som widgetten: afstemninger skal være aktiveret, og stemmer uden en logget ind bruger afvises, medmindre anonyme afstemningsstemmer er tilladt. Individuelle stemmer på anonyme afstemninger kan ikke læses via API'en.
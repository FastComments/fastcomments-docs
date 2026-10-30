Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) beskriver afstemningsobjektet og dets begrænsninger.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), og [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` læser, erstatter, redigerer og fjerner afstemningen på en kommentar.
- [PollVote Structure](/guide-api.html#poll-vote-structure) beskriver en enkelt stemme.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), og [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` lister, afgiver og fjerner stemmer.

En afstemning kan også oprettes sammen med dens kommentar ved at sende `poll` til `POST /api/v1/comments`.

API'et følger de samme indstillinger som widget'en: afstemninger skal være aktiveret, og stemmer uden en logget ind bruger afvises, medmindre anonyme afstemningsstemmer er tilladt. Individuelle stemmer på anonyme afstemninger kan ikke læses via API'et.
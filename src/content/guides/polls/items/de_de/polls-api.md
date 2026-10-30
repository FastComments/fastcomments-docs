Umfragen können auch über Ihren eigenen Code mit der REST‑API verwaltet werden, zum Beispiel um einer Kommentar‑Umfrage hinzuzufügen, eine Umfrage zu schließen oder die Stimmen zu exportieren:

- [Poll Structure](/guide-api.html#poll-structure) beschreibt das Umfrage‑Objekt und seine Grenzen.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) und [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` lesen, ersetzen, bearbeiten und entfernen die Umfrage zu einem Kommentar.
- [PollVote Structure](/guide-api.html#poll-vote-structure) beschreibt eine einzelne Stimme.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) und [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` listen, abgeben und entfernen Stimmen.

Eine Umfrage kann auch zusammen mit ihrem Kommentar erstellt werden, indem `poll` an `POST /api/v1/comments` übergeben wird.

Die API folgt denselben Einstellungen wie das Widget: Umfragen müssen aktiviert sein, und Stimmen ohne angemeldeten Benutzer werden abgelehnt, es sei denn, anonyme Umfrage‑Stimmen sind erlaubt. Einzelne Stimmen bei anonymen Umfragen können über die API nicht gelesen werden.
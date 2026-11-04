Ankiety mogą być również zarządzane z własnego kodu za pomocą REST API, na przykład aby dodać ankietę do komentarza z backendu, zamknąć ankietę lub wyeksportować głosy:

- [Struktura ankiety](/guide-api.html#poll-structure) opisuje obiekt ankiety i jego ograniczenia.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) i [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` odczytują, zastępują, edytują i usuwają ankietę na komentarzu.
- [Struktura głosu w ankiecie](/guide-api.html#poll-vote-structure) opisuje pojedynczy głos.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) i [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` wyświetlają, oddają i usuwają głosy.

Ankieta może być również utworzona razem z jej komentarzem poprzez przekazanie `poll` do `POST /api/v1/comments`.

API stosuje te same ustawienia co widżet: ankiety muszą być włączone, a głosy bez zalogowanego użytkownika są odrzucane, chyba że dozwolone są anonimowe głosy w ankietach. Pojedynczych głosów w ankietach anonimowych nie można odczytać przez API.
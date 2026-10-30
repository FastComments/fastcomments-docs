Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) opisuje obiekt ankiety i jego limity.  
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) i [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` odczytują, zastępują, edytują i usuwają ankietę w komentarzu.  
- [PollVote Structure](/guide-api.html#poll-vote-structure) opisuje pojedynczy głos.  
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) i [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` listują, oddają i usuwają głosy.

Ankieta może być również utworzona razem z jej komentarzem, przekazując `poll` do `POST /api/v1/comments`.

API stosuje te same ustawienia co widget: ankiety muszą być włączone, a głosy bez zalogowanego użytkownika są odrzucane, chyba że dozwolone są anonimowe głosy w ankietach. Indywidualne głosy w anonimowych ankietach nie mogą być odczytane za pośrednictwem API.
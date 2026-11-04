Ankete lahko upravljate tudi iz lastne kode prek REST API-ja, na primer da dodate anketo komentarju iz svojega zaledja, zaprete anketo ali izvozite glasove:

- [Struktura ankete](/guide-api.html#poll-structure) opisuje objekt ankete in njene omejitve.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) in [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` preberejo, zamenjajo, uredijo in odstranijo anketo na komentarju.
- [Struktura glasov](/guide-api.html#poll-vote-structure) opisuje posamezen glas.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) in [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` izpišejo, oddajo in odstranijo glasove.

Anketo lahko ustvarite tudi skupaj z njenim komentarjem, tako da podate `poll` pri `POST /api/v1/comments`.

API sledi istim nastavitvam kot pripomoček (widget): ankete morajo biti omogočene, glasovi brez prijavljenega uporabnika so zavrnjeni, razen če so dovoljeni anonimni glasovi. Posameznih glasov na anonimnih anketah ni mogoče prebrati prek API-ja.
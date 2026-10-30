Анкете се такође могу управљати из вашег кода помоћу REST API‑ја, на пример да додате анкету коментару из вашег бекенда, затворите анкету или извезете гласове:

- [Poll Structure](/guide-api.html#poll-structure) описује објекат анкете и њена ограничења.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) и [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` чита, замењује, уређује и уклања анкету на коментару.
- [PollVote Structure](/guide-api.html#poll-vote-structure) описује појединачни глас.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) и [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` листа, баца и уклања гласове.

Анкета се такође може креирати заједно са својим коментаром прослеђивањем `poll` у `POST /api/v1/comments`.

API следи исте поставке као и виџет: анкете морају бити омогућене, а гласови без пријављеног корисника се одбијају осим ако су анонимни гласови у анкетама дозвољени. Појединачни гласови у анонимним анкетама не могу бити прочитани преко API‑ја.
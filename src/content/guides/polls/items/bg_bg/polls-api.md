Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) описва обекта на анкетата и неговите ограничения.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) и [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` четат, заменят, редактират и премахват анкетата към коментар.
- [PollVote Structure](/guide-api.html#poll-vote-structure) описва един глас.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) и [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` изброяват, подават и премахват гласове.

Анкета може също да бъде създадена заедно с нейния коментар, като се предаде `poll` към `POST /api/v1/comments`.

API‑то следва същите настройки като уиджета: анкетите трябва да бъдат активирани, а гласовете без влезъл потребител се отхвърлят, освен ако не са разрешени анонимни гласове за анкети. Индивидуалните гласове в анонимни анкети не могат да се четат чрез API‑то.
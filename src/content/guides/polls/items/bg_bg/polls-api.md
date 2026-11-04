Анкетите могат да се управляват и от вашия собствен код чрез REST API, например за да добавите анкета към коментар от вашия backend, да затворите анкета или да експортирате гласовете:

- [Poll Structure](/guide-api.html#poll-structure) описва обекта на анкетата и неговите ограничения.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) и [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` четат, заменят, редактират и премахват анкетата на коментар.
- [PollVote Structure](/guide-api.html#poll-vote-structure) описва единичен глас.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) и [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` изброяват, подават и премахват гласове.

Анкета може да бъде създадена заедно с нейния коментар чрез подаване на `poll` към `POST /api/v1/comments`.

API следва същите настройки като уиджета: анкетите трябва да са активирани, а гласовете без влязъл потребител се отхвърлят, освен ако не са разрешени анонимни гласувания в анкети. Отделните гласове в анонимни анкети не могат да бъдат прочетени чрез API.
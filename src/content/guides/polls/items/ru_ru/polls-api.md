Опросы также можно управлять из собственного кода с помощью REST API, например, чтобы добавить опрос к комментарию из вашего бэкенда, закрыть опрос или экспортировать голоса:

- [Poll Structure](/guide-api.html#poll-structure) описывает объект опроса и его ограничения.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) и [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` читают, заменяют, редактируют и удаляют опрос в комментарии.
- [PollVote Structure](/guide-api.html#poll-vote-structure) описывает отдельный голос.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) и [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` перечисляют, регистрируют и удаляют голоса.

Опрос также можно создать вместе с его комментарием, передав `poll` в `POST /api/v1/comments`.

API использует те же настройки, что и виджет: опросы должны быть включены, а голоса без авторизованного пользователя отклоняются, если только не разрешены анонимные голоса в опросе. Отдельные голоса в анонимных опросах нельзя прочитать через API.
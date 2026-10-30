---
Опитування також можна керувати зі свого коду за допомогою REST API, наприклад, щоб додати опитування до коментаря з вашого бекенду, закрити опитування або експортувати голоси:

- [Poll Structure](/guide-api.html#poll-structure) описує об’єкт опитування та його обмеження.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch) та [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` читає, замінює, редагує та видаляє опитування в коментарі.
- [PollVote Structure](/guide-api.html#poll-vote-structure) описує один голос.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post) та [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` перераховує, подає та видаляє голоси.

Опитування також можна створити разом з його коментарем, передавши `poll` у `POST /api/v1/comments`.

API дотримується тих самих налаштувань, що й віджет: опитування мають бути ввімкнені, а голоси без авторизованого користувача відхиляються, якщо не дозволено анонімні голоси в опитуваннях. Окремі голоси в анонімних опитуваннях не можна прочитати через API.

---
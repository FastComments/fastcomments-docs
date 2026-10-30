Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) は投票オブジェクトとその制限を説明します。
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), and [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` はコメント上の投票を取得、置換、編集、削除します。
- [PollVote Structure](/guide-api.html#poll-vote-structure) は単一の投票を説明します。
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), and [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` は投票の一覧取得、投票、削除を行います。

A poll can also be created along with its comment by passing `poll` to `POST /api/v1/comments`.

The API follows the same settings as the widget: polls must be enabled, and votes without a logged in user are rejected unless anonymous poll votes are allowed. Individual votes on anonymous polls cannot be read through the API.
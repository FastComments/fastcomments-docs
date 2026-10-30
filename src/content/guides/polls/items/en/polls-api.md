Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) describes the poll object and its limits.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), and [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` read, replace, edit, and remove the poll on a comment.
- [PollVote Structure](/guide-api.html#poll-vote-structure) describes a single vote.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), and [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` list, cast, and remove votes.

A poll can also be created along with its comment by passing `poll` to `POST /api/v1/comments`.

The API follows the same settings as the widget: polls must be enabled, and votes without a logged in user are rejected unless anonymous poll votes are allowed. Individual votes on anonymous polls cannot be read through the API.

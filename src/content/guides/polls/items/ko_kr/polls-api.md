Polls can also be managed from your own code with the REST API, for example to add a poll to a comment from your backend, close a poll, or export the votes:

- [Poll Structure](/guide-api.html#poll-structure) 설문조사 객체와 그 제한 사항을 설명합니다.
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), 및 [DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId`는 댓글에 대한 설문조사를 읽고, 교체하고, 편집하고, 제거합니다.
- [PollVote Structure](/guide-api.html#poll-vote-structure) 단일 투표를 설명합니다.
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), 및 [DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes`는 투표 목록을 조회하고, 투표를 진행하며, 투표를 제거합니다.

설문조사는 `POST /api/v1/comments`에 `poll`을 전달하여 댓글과 함께 생성할 수도 있습니다.

API는 위젯과 동일한 설정을 따릅니다: 설문조사가 활성화되어야 하며, 익명 설문조사 투표가 허용되지 않는 한 로그인하지 않은 사용자의 투표는 거부됩니다. 익명 설문조사에 대한 개별 투표는 API를 통해 읽을 수 없습니다.